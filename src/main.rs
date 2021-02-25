use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::mandelbrot::{get_escape_iterations, Coordinate};
use crate::pixel::{Pixel, SET_PIXEL};

mod mandelbrot;
mod pixel;

type R<T> = anyhow::Result<T>;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const LOWER_LEFT: Coordinate = Coordinate { x: -1.8, y: -1.2 };
const UPPER_RIGHT: Coordinate = Coordinate { x: 0.7, y: 1.2 };
const ESCAPE_THRESHOLD: u32 = 25;
const LOOP_THRESHOLD: u32 = 1000;

const BLUE_FACTOR: f32 = 1.0;
const GREEN_FACTOR: f32 = 0.5;
const RED_FACTOR: f32 = 0.05;

fn my_colour_function(iterations: u32, threshold: u32) -> Pixel {
    let fraction = (iterations as f32) / (threshold as f32);
    Pixel {
        r: (RED_FACTOR * fraction * 255_f32).round() as u8,
        g: (GREEN_FACTOR * fraction * 255_f32).round() as u8,
        b: (BLUE_FACTOR * fraction * 255_f32).round() as u8,
        a: 255,
    }
}

struct MandelbrotState {
    width: u32,
    height: u32,
    canvas: Vec<Vec<Pixel>>,
    loop_threshold: u32,
    escape_threshold: u32,
    lower_left: Coordinate,
    step_x: f64,
    step_y: f64,
    last_rendered: u32,
}

impl MandelbrotState {
    fn new(
        width: u32,
        height: u32,
        loop_threshold: u32,
        escape_threshold: u32,
        lower_left: Coordinate,
        upper_right: Coordinate,
    ) -> Self {
        let step_x = (upper_right.x - lower_left.x) / width as f64;
        let step_y = (upper_right.x - lower_left.x) / height as f64;
        Self {
            width,
            height,
            canvas: vec![vec![SET_PIXEL; height as usize]; width as usize],
            loop_threshold,
            escape_threshold,
            lower_left,
            step_x,
            step_y,
            last_rendered: 0,
        }
    }
    fn render_next(&mut self) {
        let x = (self.last_rendered % self.width) as usize;
        let y = (self.last_rendered / self.width) as usize;
        let coordinate = Coordinate {
            x: (x as f64 * self.step_x) + self.lower_left.x,
            y: (y as f64 * self.step_y) + self.lower_left.y,
        };
        let escape_iterations =
            get_escape_iterations(coordinate, self.loop_threshold, self.escape_threshold);
        self.canvas[x][y] = if escape_iterations > self.loop_threshold {
            SET_PIXEL
        } else {
            my_colour_function(escape_iterations, self.escape_threshold)
        };
        self.last_rendered += 1;
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            // The mandelbrot uses positive-right, whereas pixels uses positive-left, so we have to flip
            let x = (WIDTH - 1) as usize - (i % WIDTH as usize);
            let y = i / WIDTH as usize;
            let rgba = self.canvas[x][y].to_slice();
            pixel.copy_from_slice(&rgba);
        }
    }
}

fn main() -> R<()> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Rustybrot")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mandelbrot = Arc::new(Mutex::new(MandelbrotState::new(
        WIDTH,
        HEIGHT,
        LOOP_THRESHOLD,
        ESCAPE_THRESHOLD,
        LOWER_LEFT,
        UPPER_RIGHT,
    )));

    let workerbrot = Arc::clone(&mandelbrot);
    thread::spawn(move || {
        for _ in 0..WIDTH * HEIGHT {
            workerbrot.lock().unwrap().render_next()
        }
    });

    // join_handle.join();
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            mandelbrot.lock().unwrap().draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| println!("pixels.render() failed: {:?}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}
