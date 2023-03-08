use ray_tracer::canvas::Canvas;
use ray_tracer::matrix::Matrix;
use ray_tracer::rays::{hit, Ray, Sphere};
use ray_tracer::tuple::{Color, Tuple};
// use std::f64::consts::PI;

fn main() {
    // arbitrarily chosen values
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 8.0;
    let canvas_pixels = 300;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let transform = Matrix::identity();
    let sphere = Sphere::with_transform(transform);
    let world_pixel_size = wall_size / canvas_pixels as f64;
    let half_wall_size = wall_size / 2.0;

    print!("Casting rays...");
    use std::io::Write;
    std::io::stdout().flush().unwrap();
    let now = std::time::Instant::now();

    for x in 0..canvas.width {
        for y in 0..canvas.height {
            // magic
            let world_y = half_wall_size - world_pixel_size * y as f64;
            let world_x = -half_wall_size + world_pixel_size * x as f64;

            let world_position = Tuple::point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (world_position - ray_origin).normalize());

            if let Some(intersections) = ray.intersect(&sphere) {
                if hit(&intersections).is_some() {
                    let point = Tuple::point(x as f64, y as f64, 0.0);
                    canvas.write_pixel(&point, Color::red());
                }
            }
        }
    }
    println!(" done: {} seconds", now.elapsed().as_secs());

    ray_tracer::save_image(canvas, "circle-2d.ppm");
}
