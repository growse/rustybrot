use crate::pixel::{Pixel, SET_PIXEL};
use itertools::Itertools;

#[derive(Debug)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

fn _mandelbrot(
    lower_left: Coordinate,
    upper_right: Coordinate,
    image_width: u32,
    image_height: u32,
    escape_threshold: u32,
    loop_threshold: u32,
    colour_function: fn(u32, u32) -> Pixel,
) -> Vec<u8> {
    let step_x = (upper_right.x - lower_left.x) / image_width as f64;
    let step_y = (upper_right.y - lower_left.y) / image_height as f64;
    (0..image_height)
        .cartesian_product((0..image_width).rev())
        .map(|c| Coordinate {
            x: (c.1 as f64 * step_x) + lower_left.x,
            y: (c.0 as f64 * step_y) + lower_left.y,
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

pub(crate) fn get_escape_iterations(
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
