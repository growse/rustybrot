use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const MIN_X: f64 = -2.0;
const MIN_Y: f64 = -2.0;
const MAX_X: f64 = 2.0;
const MAX_Y: f64 = 2.0;

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
		50,
		1000,
	);
	writer.write_image_data(&data).unwrap(); // Save
}

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
) -> Vec<u8> {
	let mut pixels: Vec<Pixel> = Vec::with_capacity((image_width * image_height) as usize);
	let step_x = (upper_right.x - lower_left.x) / image_width as f64;
	let step_y = (upper_right.x - lower_left.x) / image_width as f64;
	for x in 0..image_width as usize {
		for y in 0..image_height as usize {
			let cr = (x as f64 * step_x) + lower_left.x;
			let ci = (y as f64 * step_y) + lower_left.y;
			let mut zr = cr;
			let mut zi = ci;
			let mut counter = 0;
			let escape_iterations = loop {
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
			};
			if escape_iterations > loop_threshold {
				pixels.push(Pixel {
					r: 0,
					g: 0,
					b: 0,
					a: 255,
				});
			} else {
				pixels.push(Pixel {
					r: 255,
					g: 255,
					b: 255,
					a: 255,
				})
			}
		}
	}
	return pixels
		.iter()
		.flat_map(|p| vec![p.r, p.g, p.b, p.a])
		.collect();
}
