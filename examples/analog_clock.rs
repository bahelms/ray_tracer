use ray_tracer::canvas::Canvas;
use ray_tracer::matrix::Matrix;
use ray_tracer::tuple::{Color, Tuple};
use std::f64::consts::PI;

fn main() {
    let mut canvas = Canvas::new(250, 250);
    let start_point = Tuple::point(0.0, -100.0, 0.0);
    let identity = Matrix::identity();

    let twelve = identity.translate(125.0, 125.0, 0.0) * start_point;
    canvas.write_pixel(&twelve, Color::white());

    for hour in 1..12 {
        let new_point = identity
            .rotate_z(hour as f64 * PI / 6.0)
            .translate(125.0, 125.0, 0.0)
            * start_point;
        canvas.write_pixel(&new_point, Color::white());
    }

    save_image(canvas);
}

fn save_image(canvas: Canvas) {
    use std::fs::File;
    use std::io::prelude::*;

    println!("Saving image...");
    let mut file = File::create("images/analog_clock.ppm").unwrap();
    file.write_all(canvas.to_ppm().as_bytes()).unwrap();
}
