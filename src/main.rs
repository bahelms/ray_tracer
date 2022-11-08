mod canvas;
mod tuple;

fn virtual_cannon() {
    use tuple::{Point, Vector};

    struct Projectile {
        position: Point,
        velocity: Vector,
    }

    struct Environment {
        gravity: Vector,
        wind: Vector,
    }

    fn tick(env: &Environment, projectile: Projectile) -> Projectile {
        let position = projectile.position + projectile.velocity;
        let velocity = projectile.velocity + env.gravity + env.wind;
        Projectile { position, velocity }
    }

    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize(),
    };

    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut tick_count = 0;
    while projectile.position.y > 0.0 {
        projectile = tick(&env, projectile);
        tick_count += 1;
        println!("{:?}", projectile.position);
    }
    println!("Total ticks: {:?}", tick_count);
}

fn main() {
    virtual_cannon()
}
