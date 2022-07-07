mod tuple;

fn virtual_cannon() {
    use tuple::Tuple;

    struct Projectile {
        position: Tuple,
        velocity: Tuple,
    }

    struct Environment {
        gravity: Tuple,
        wind: Tuple,
    }

    fn tick(env: &Environment, projectile: Projectile) -> Projectile {
        let position = projectile.position + projectile.velocity;
        let velocity = projectile.velocity + env.gravity + env.wind;
        Projectile { position, velocity }
    }

    let mut projectile = Projectile {
        position: Tuple::new_point(0.0, 1.0, 0.0),
        velocity: Tuple::new_vector(1.0, 1.0, 0.0).normalize(),
    };

    let env = Environment {
        gravity: Tuple::new_vector(0.0, -0.1, 0.0),
        wind: Tuple::new_vector(-0.01, 0.0, 0.0),
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
