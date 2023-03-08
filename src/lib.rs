pub mod canvas;
pub mod matrix;
pub mod rays;
pub mod tuple;

use crate::canvas::Canvas;

const EPSILON: f64 = 0.00001;

pub fn is_float_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn save_image(canvas: Canvas, filename: &str) {
    use std::fs::File;
    use std::io::prelude::*;

    println!("Saving image...");
    let mut file = File::create(format!("images/{}", filename)).unwrap();
    file.write_all(canvas.to_ppm().as_bytes()).unwrap();
}
