use ray_tracer::canvas::Canvas;
use ray_tracer::matrix::Matrix;
use ray_tracer::tuple::{Color, Tuple};
use std::f64::consts::PI;

fn main() {
    let mut canvas = Canvas::new(200, 200);
    let point = Tuple::point(100.0, 10.0, 0.0);

    canvas.write_pixel(&point, Color::white());

    let transform = Matrix::identity().rotate_z(PI / 2.0);
    let new_point = transform * point;
    canvas.write_pixel(&point, Color::white());

    let transform = Matrix::identity().rotate_z(PI / 1.0);
    let new_point = transform * point;
    canvas.write_pixel(&point, Color::white());

    save_image(canvas);
}

fn save_image(canvas: Canvas) {
    use std::fs::File;
    use std::io::prelude::*;

    println!("Saving image...");
    let mut file = File::create("images/analog_clock.ppm").unwrap();
    file.write_all(canvas.to_ppm().as_bytes()).unwrap();
}
