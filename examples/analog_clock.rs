use ray_tracer::canvas::Canvas;
use ray_tracer::matrix::Matrix;
use ray_tracer::tuple::{Color, Tuple};
use std::f64::consts::PI;

const RADIANS_IN_AN_HOUR: f64 = PI / 6.0;

fn main() {
    let mut canvas = Canvas::new(250, 250);
    let start_point = Tuple::point(0.0, -100.0, 0.0);

    for hour in 1..=12 {
        let transformation = Matrix::identity()
            .rotate_z(hour as f64 * RADIANS_IN_AN_HOUR)
            .translate((canvas.width / 2) as f64, 125.0, 0.0);
        let new_point = transformation * start_point;
        canvas.write_pixel(&new_point, Color::white());
    }

    ray_tracer::save_image(canvas, "analog_clock.ppm");
}
