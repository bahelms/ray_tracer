use ray_tracer::canvas::Canvas;
use ray_tracer::tuple::{Color, Point, Vector};
use std::fs::File;
use std::io::prelude::*;

fn virtual_cannon() {
    struct Projectile {
        position: Point,
        velocity: Vector,
    }

    struct Environment {
        gravity: Vector,
        wind: Vector,
    }

    fn tick(env: &Environment, projectile: Projectile) -> Projectile {
        Projectile {
            position: projectile.position + projectile.velocity,
            velocity: projectile.velocity + env.gravity + env.wind,
        }
    }

    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 7.85,
    };

    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(500, 300);

    while projectile.position.y > 0.0 {
        projectile = tick(&env, projectile);
        let pos = projectile.position;
        let pos_y = canvas.height - (pos.y as i32);
        if pos_y <= canvas.height {
            let color = Color::new(1.0, 0.0, 1.0);
            canvas.write_pixel(pos.x as i32, pos_y, color);
        }
    }

    let mut file = File::create("images/cannon.ppm").unwrap();
    file.write_all(canvas.to_ppm().as_bytes()).unwrap();
}

fn main() {
    virtual_cannon()
}
