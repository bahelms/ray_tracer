use ray_tracer::canvas::Canvas;
use ray_tracer::matrix::Matrix;
use ray_tracer::rays::{hit, lighting, Material, Object, PointLight, Ray, Sphere};
use ray_tracer::tuple::{Color, Tuple};

fn main() {
    // arbitrarily chosen values
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 8.0;
    let canvas_pixels = 300;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let transform = Matrix::identity();
    let mut sphere = Sphere::with_transform(transform);
    sphere.material = Material::new().color(Color::new(1.0, 0.2, 1.0));

    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white());

    let world_pixel_size = wall_size / canvas_pixels as f64;
    let half_wall_size = wall_size / 2.0;

    print!("Casting rays...");
    use std::io::Write;
    std::io::stdout().flush().unwrap();
    let now = std::time::Instant::now();

    for x in 0..canvas.width {
        for y in 0..canvas.height {
            let world_y = half_wall_size - world_pixel_size * y as f64;
            let world_x = -half_wall_size + world_pixel_size * x as f64;

            let world_position = Tuple::point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (world_position - ray_origin).normalize());

            if let Some(intersections) = ray.intersect(&sphere) {
                if let Some(intersection) = hit(&intersections) {
                    let canvas_point = Tuple::point(x as f64, y as f64, 0.0);

                    let hit_point = ray.position(intersection.time);
                    let normal = sphere.normal_at(&hit_point).unwrap();
                    let eye = -ray.direction;
                    let color = lighting(
                        &intersection.object.material,
                        &light,
                        hit_point,
                        eye,
                        normal,
                    );
                    canvas.write_pixel(&canvas_point, color);
                }
            }
        }
    }
    println!(" done: {} seconds", now.elapsed().as_secs());

    ray_tracer::save_image(canvas, "lighted_sphere.ppm");
}
