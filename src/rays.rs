use crate::matrix::Matrix;
use crate::tuple::{Color, Tuple};
use rand::prelude::*;

pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, time: f64) -> Tuple {
        self.direction * time + self.origin
    }

    pub fn intersect<'a>(&'a self, sphere: &'a Sphere) -> Option<Vec<Intersection>> {
        // Hardcoded unit sphere
        let sphere_center = Tuple::point(0.0, 0.0, 0.0);
        // Transform the ray instead of the sphere - let's the sphere stay at unit
        let transform_inverse = match sphere.transform.inverse() {
            Some(transform_inverse) => transform_inverse,
            None => return None,
        };
        let new_ray = self.transform(transform_inverse);

        // magic
        // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection.html
        let center_to_origin = new_ray.origin - sphere_center;
        let a = new_ray.direction.dot(&new_ray.direction);
        let b = 2.0 * new_ray.direction.dot(&center_to_origin);
        let c = center_to_origin.dot(&center_to_origin) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrt = discriminant.sqrt();
            Some(vec![
                Intersection::new((-b - sqrt) / (2.0 * a), sphere),
                Intersection::new((-b + sqrt) / (2.0 * a), sphere),
            ])
        }
    }

    fn transform(&self, transformation: Matrix) -> Self {
        Self {
            origin: &transformation * self.origin,
            direction: &transformation * self.direction,
        }
    }
}

trait Object {
    fn normal_at(&self, point: &Tuple) -> Option<Tuple>;
}

#[derive(Debug, PartialEq)]
pub struct Sphere {
    id: f64,
    transform: Matrix,
    material: Material,
}

impl Sphere {
    fn new() -> Self {
        Self::with_transform(Matrix::identity())
    }

    pub fn with_transform(transform: Matrix) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: rng.gen(),
            material: Material::new(),
            transform,
        }
    }
}

impl Object for Sphere {
    fn normal_at(&self, world_point: &Tuple) -> Option<Tuple> {
        match self.transform.inverse() {
            Some(inverse) => {
                let center = Tuple::point(0.0, 0.0, 0.0); // Hardcoded unit sphere
                let object_point = &inverse * *world_point;
                let object_normal = object_point - center;
                let mut world_normal = inverse.transpose() * object_normal;
                world_normal.w = 0.0; // hack - see page 82
                Some(world_normal.normalize())
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    fn new() -> Self {
        Self {
            color: Color::black(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    time: f64,
    object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    fn new(time: f64, object: &'a Sphere) -> Self {
        Self { time, object }
    }
}

// Find the hit in a collection of intersections.
pub fn hit<'a>(intersections: &'a Vec<Intersection>) -> Option<&'a Intersection<'a>> {
    let mut hit = None;
    for intersection in intersections {
        if intersection.time < 0.0 {
            continue;
        }

        match hit {
            None => hit = Some(intersection),
            Some(last_hit) => {
                if intersection.time < last_hit.time {
                    hit = Some(intersection)
                }
            }
        }
    }
    hit
}

pub struct PointLight {
    position: Tuple,
    intensity: Color,
}

impl PointLight {
    fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

fn lighting(
    material: Material,
    light: PointLight,
    position: Tuple,
    eye: Tuple,
    normal: Tuple,
) -> Color {
    // combine surface color with the light's color/intensity
    let effective_color = &material.color + &light.intensity;
    let light_direction = (light.position - position).normalize();
    let ambient = &effective_color * material.ambient;
    let mut diffuse = Color::black();
    let mut specular = Color::black();

    // This is the cosine of the angle between the light vector and normal.
    // A negative value means the light is on the other side of the surface.
    let light_dot_normal = light_direction.dot(&normal);
    if light_dot_normal > 0.0 {
        diffuse = &(&effective_color * material.diffuse) * light_dot_normal;

        // This is the cosine of the angle between the eye and reflection.
        // A negative value means the light reflects away from the eye.
        let reflection_direction = -light_direction.reflect(&normal);
        let reflect_dot_eye = reflection_direction.dot(&eye);

        if reflect_dot_eye <= 0.0 {
            specular = Color::black();
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = &(&light.intensity * material.specular) * factor;
        }
    }
    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_float_equal;
    use crate::matrix::Matrix;
    use core::f64::consts::{FRAC_1_SQRT_2, PI};

    #[test]
    fn lighting_with_light_behind_surface() {
        let material = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::white());
        let color = lighting(material, light, position, eye, normal);
        assert_eq!(color, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let material = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, 2.0_f64.sqrt() / -2.0, 2.0_f64.sqrt() / -2.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::white());
        let color = lighting(material, light, position, eye, normal);
        assert!(is_float_equal(color.red, 1.6364));
        assert!(is_float_equal(color.green, 1.6364));
        assert!(is_float_equal(color.blue, 1.6364));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let material = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::white());
        let color = lighting(material, light, position, eye, normal);
        assert!(is_float_equal(color.red, 0.7364));
        assert!(is_float_equal(color.green, 0.7364));
        assert!(is_float_equal(color.blue, 0.7364));
    }

    #[test]
    fn lighting_with_eye_offset_45_degrees_between_light_and_surface() {
        let material = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / -2.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white());
        let color = lighting(material, light, position, eye, normal);
        assert_eq!(color, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_surface() {
        let material = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eye = Tuple::vector(0.0, 0.0, -1.0);
        let normal = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::white());
        let color = lighting(material, light, position, eye, normal);
        assert_eq!(color, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn sphere_can_have_material_assigned() {
        let mut sphere = Sphere::new();
        let mut material = Material::new();
        material.ambient = 1.99;
        sphere.material = material;
        assert_eq!(sphere.material.ambient, 1.99);
    }

    #[test]
    fn sphere_has_a_default_material() {
        assert_eq!(Sphere::new().material, Material::new());
    }

    #[test]
    fn material_default_values() {
        let material = Material::new();
        assert_eq!(material.color, Color::black());
        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
    }

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color::black();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);
        assert_eq!(light.position, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(light.intensity, Color::black());
    }

    #[test]
    fn calculate_normal_on_transformed_sphere() {
        let sphere =
            Sphere::with_transform(Matrix::identity().rotate_z(PI / 5.0).scale(1.0, 0.5, 1.0));
        let value = 2.0_f64.sqrt() / 2.0;
        let normal = sphere.normal_at(&Tuple::point(0.0, value, -value)).unwrap();
        assert!(is_float_equal(normal.y, 0.97014));
        assert!(is_float_equal(normal.z, -0.24254));
    }

    #[test]
    fn calculate_normal_on_translated_sphere() {
        let sphere = Sphere::with_transform(Matrix::identity().translate(0.0, 1.0, 0.0));
        let normal = sphere
            .normal_at(&Tuple::point(0.0, 1.70711, -FRAC_1_SQRT_2))
            .unwrap();
        assert!(is_float_equal(normal.y, FRAC_1_SQRT_2));
        assert!(is_float_equal(normal.z, -FRAC_1_SQRT_2));
    }

    #[test]
    fn normals_are_always_normalized() {
        let sphere = Sphere::new();
        let value = 3.0_f64.sqrt() / 3.0;
        let normal = sphere
            .normal_at(&Tuple::point(value, value, value))
            .unwrap();
        assert_eq!(normal, normal.normalize());
    }

    #[test]
    fn normal_of_a_sphere_on_nonaxial_point() {
        let sphere = Sphere::new();
        let value = 3.0_f64.sqrt() / 3.0;
        let normal = sphere
            .normal_at(&Tuple::point(value, value, value))
            .unwrap();
        assert_eq!(normal, Tuple::vector(value, value, value));
    }

    #[test]
    fn normal_of_a_sphere_on_z_axis() {
        let sphere = Sphere::new();
        let normal = sphere.normal_at(&Tuple::point(0.0, 0.0, 1.0)).unwrap();
        assert_eq!(normal, Tuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_of_a_sphere_on_y_axis() {
        let sphere = Sphere::new();
        let normal = sphere.normal_at(&Tuple::point(0.0, 1.0, 0.0)).unwrap();
        assert_eq!(normal, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_of_a_sphere_on_x_axis() {
        let sphere = Sphere::new();
        let normal = sphere.normal_at(&Tuple::point(1.0, 0.0, 0.0)).unwrap();
        assert_eq!(normal, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::with_transform(Matrix::identity().translate(5.0, 0.0, 0.0));
        assert_eq!(ray.intersect(&sphere), None);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::with_transform(Matrix::identity().scale(2.0, 2.0, 2.0));
        let intersections = ray.intersect(&sphere).unwrap();
        assert_eq!(intersections[0].time, 3.0);
        assert_eq!(intersections[1].time, 7.0);
    }

    #[test]
    fn new_sphere_has_default_transform_and_can_be_changed() {
        let mut sphere = Sphere::new();
        assert_eq!(sphere.transform, Matrix::identity());
        sphere.transform = Matrix::identity().translate(2.0, 0.0, 1.0);
        assert_eq!(
            sphere.transform,
            Matrix::identity().translate(2.0, 0.0, 1.0)
        );
    }

    #[test]
    fn new_sphere_returns_unique_value() {
        let one = Sphere::new();
        let two = Sphere::new();
        assert_ne!(one, two);
    }

    #[test]
    fn scaling_a_ray() {
        let ray = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let scaling = Matrix::identity().scale(2.0, 3.0, 4.0);
        let new_ray = ray.transform(scaling);
        assert_eq!(new_ray.origin, Tuple::point(2.0, 6.0, 12.0));
        assert_eq!(new_ray.direction, Tuple::vector(0.0, 3.0, 0.0));
    }

    #[test]
    fn translating_a_ray() {
        let ray = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let translation = Matrix::identity().translate(3.0, 4.0, 5.0);
        let new_ray = ray.transform(translation);
        assert_eq!(new_ray.origin, Tuple::point(4.0, 6.0, 8.0));
        assert_eq!(new_ray.direction, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn hit_is_always_lowest_nonnegative_intersection() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(5.0, &sphere);
        let i2 = Intersection::new(7.0, &sphere);
        let i3 = Intersection::new(-3.0, &sphere);
        let i4 = Intersection::new(2.0, &sphere);
        assert_eq!(
            hit(&vec![i2, i1, i3, i4]).unwrap(),
            &Intersection::new(2.0, &sphere)
        );
    }

    #[test]
    fn hit_when_all_intersections_have_negative_times() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(-2.0, &sphere);
        let i2 = Intersection::new(-1.0, &sphere);
        assert_eq!(hit(&vec![i2, i1]), None);
    }

    #[test]
    fn hit_when_some_intersections_have_negative_times() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(-1.0, &sphere);
        let i2 = Intersection::new(1.0, &sphere);
        assert_eq!(
            hit(&vec![i2, i1]).unwrap(),
            &Intersection::new(1.0, &sphere)
        );
    }

    #[test]
    fn hit_when_all_intersections_have_positive_times() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(1.0, &sphere);
        let i2 = Intersection::new(2.0, &sphere);
        assert_eq!(
            hit(&vec![i2, i1]).unwrap(),
            &Intersection::new(1.0, &sphere)
        );
    }

    #[test]
    fn new_intersection() {
        let sphere = Sphere::new();
        let intersection = Intersection::new(3.5, &sphere);
        assert_eq!(intersection.time, 3.5);
        assert_eq!(intersection.object, &sphere);
    }

    #[test]
    fn rays_have_negative_units_when_origin_is_in_front_of_sphere() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let ints = ray.intersect(&sphere).unwrap();
        assert_eq!(ints[0].time, -6.0);
        assert_eq!(ints[0].object, &sphere);
        assert_eq!(ints[1].time, -4.0);
        assert_eq!(ints[1].object, &sphere);
    }

    #[test]
    fn rays_inside_spheres_have_a_negative_unit() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let ints = ray.intersect(&sphere).unwrap();
        assert_eq!(ints[0].time, -1.0);
        assert_eq!(ints[1].time, 1.0);
    }

    #[test]
    fn intersect_returns_none_when_there_is_no_intersection() {
        let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        assert_eq!(ray.intersect(&sphere), None);
    }

    #[test]
    fn intersect_units_are_equal_on_tangents() {
        let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let ints = ray.intersect(&sphere).unwrap();
        assert_eq!(ints[0].time, 5.0);
        assert_eq!(ints[1].time, 5.0);
    }

    #[test]
    fn rays_intersect_spheres_at_two_time_units() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new();
        let ints = ray.intersect(&sphere).unwrap();
        assert_eq!(ints[0].time, 4.0);
        assert_eq!(ints[1].time, 6.0);
    }

    #[test]
    fn calculate_point_of_ray_from_distance() {
        let ray = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(ray.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn creating_a_ray() {
        let ray = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(4.0, 5.0, 6.0));
        assert_eq!(ray.origin.x, 1.0);
        assert_eq!(ray.direction.x, 4.0);
    }
}
