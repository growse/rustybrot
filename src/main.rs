use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use itertools::{Itertools};

const WIDTH: u32 = 1600;
const HEIGHT: u32 = 1600;
const MIN_X: f64 = -1.8;
const MIN_Y: f64 = -1.2;
const MAX_X: f64 = 0.7;
const MAX_Y: f64 = 1.2;
const ESCAPE_THRESHOLD: u32 = 25;
const LOOP_THRESHOLD: u32 = 1000;
const SET_PIXEL: Pixel = Pixel {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};

struct Coordinate {
    x: f64,
    y: f64,
}

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn mandelbrot(
    lower_left: Coordinate,
    upper_right: Coordinate,
    image_width: u32,
    image_height: u32,
    escape_threshold: u32,
    loop_threshold: u32,
    colour_function: fn(u32, u32) -> Pixel,
) -> Vec<u8> {
    let step_x = (upper_right.x - lower_left.x) / image_width as f64;
    let step_y = (upper_right.x - lower_left.x) / image_width as f64;
    (0..image_width)
        .cartesian_product(0..image_height)
        .map(|c| Coordinate {
            x: (c.0 as f64 * step_x) + lower_left.x,
            y: (c.1 as f64 * step_y) + lower_left.y,
        })
        .map(|coordinate| get_escape_iterations(coordinate, loop_threshold, escape_threshold))
        .map(|escape_iterations| {
            if escape_iterations > loop_threshold {
                SET_PIXEL
            } else {
                colour_function(escape_iterations, escape_threshold)
            }
        })
        .flat_map(|p| vec![p.r, p.g, p.b, p.a])
        .collect()
}

fn get_escape_iterations(
    coordinate: Coordinate,
    loop_threshold: u32,
    escape_threshold: u32,
) -> u32 {
    let cr = coordinate.x;
    let ci = coordinate.y;
    let mut zr = cr;
    let mut zi = ci;
    let mut counter = 0;
    loop {
        counter += 1;
        let temp_zr = zr;
        zr = ((zr * zr) - (zi * zi)) + cr;
        zi = (2.0 * temp_zr * zi) + ci;
        let abs = ((zr * zr) + (zi * zi)).sqrt();
        if abs > escape_threshold as f64 {
            break counter;
        }
        if counter > loop_threshold {
            break counter;
        }
    }
}

const BLUE_FACTOR: f32 = 1.0;
const GREEN_FACTOR: f32 = 0.5;
const RED_FACTOR: f32 = 0.05;

fn my_colour_function(iterations: u32, threshold: u32) -> Pixel {
    let fraction = (iterations as f32) / (threshold as f32);
    Pixel {
        r: (RED_FACTOR * fraction * 255 as f32).round() as u8,
        g: (GREEN_FACTOR * fraction * 255 as f32).round() as u8,
        b: (BLUE_FACTOR * fraction * 255 as f32).round() as u8,
        a: 255,
    }
}

fn main() {
    let path = Path::new(r"output.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let data = mandelbrot(
        Coordinate { x: MIN_X, y: MIN_Y },
        Coordinate { x: MAX_X, y: MAX_Y },
        WIDTH,
        HEIGHT,
        ESCAPE_THRESHOLD,
        LOOP_THRESHOLD,
        my_colour_function,
    );
    writer.write_image_data(&data).unwrap(); // Save
}
