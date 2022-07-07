use std::ops::{Add, Div, Mul, Neg, Sub};

const EPSILON: f64 = 0.00001;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tuple {
    x: f64,
    pub y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 0.0)
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 1.0)
    }

    fn is_equal(&self, other: &Self) -> bool {
        is_float_equal(self.x, other.x)
            && is_float_equal(self.y, other.y)
            && is_float_equal(self.z, other.z)
            && is_float_equal(self.w, other.w)
    }

    /// The distance of a vector.
    /// It's the length of a straight line from end to end of the vector.
    fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
    }

    /// Converting an arbitrary vector into a unit vector.
    /// A unit vector is a vector with magnitude 1.
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self::new(
            self.x / magnitude,
            self.y / magnitude,
            self.z / magnitude,
            self.w / magnitude,
        )
    }

    /// Multiply this vector with another one to reduce it to one scalar number.
    /// This applies the directional growth of one vector to another.
    /// The smaller the value, the larger the angle between the vectors.
    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Cross product of two vectors.
    /// The returned vector is perpendicular to the other two.
    /// Order is important. `other.cross(&self)` would return a vector in the
    /// opposite direction.
    fn cross(&self, other: &Self) -> Self {
        Self::new_vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

fn is_float_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_product_of_two_vectors_returns_a_vector() {
        let v1 = Tuple::new_vector(1.0, 2.0, 3.0);
        let v2 = Tuple::new_vector(2.0, 3.0, 4.0);
        assert_eq!(v1.cross(&v2), Tuple::new_vector(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(&v1), Tuple::new_vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn dot_product_of_two_vectors_returns_a_scalar() {
        let v1 = Tuple::new_vector(1.0, 2.0, 3.0);
        let v2 = Tuple::new_vector(2.0, 3.0, 4.0);
        assert_eq!(v1.dot(&v2), 20.0);
    }

    #[test]
    fn normalizing_vectors() {
        assert_eq!(
            Tuple::new_vector(4.0, 0.0, 0.0).normalize(),
            Tuple::new_vector(1.0, 0.0, 0.0)
        );

        let sqrt = 14.0_f64.sqrt();
        assert_eq!(
            Tuple::new_vector(1.0, 2.0, 3.0).normalize(),
            Tuple::new_vector(1.0 / sqrt, 2.0 / sqrt, 3.0 / sqrt)
        );
    }

    #[test]
    fn computing_magnitude_of_other_vectors() {
        let expected_value = 14.0_f64.sqrt();
        assert_eq!(Tuple::new_vector(1.0, 2.0, 3.0).magnitude(), expected_value);
        assert_eq!(
            Tuple::new_vector(-1.0, -2.0, -3.0).magnitude(),
            expected_value
        );
    }

    #[test]
    fn computing_magnitude_of_unit_vectors() {
        assert_eq!(Tuple::new_vector(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(Tuple::new_vector(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(Tuple::new_vector(0.0, 0.0, 1.0).magnitude(), 1.0);
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let t1 = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let t2 = Tuple::new(0.5, -1.0, 1.5, -2.0);
        assert_eq!(t1 / 2.0, t2);
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let t1 = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let t2 = Tuple::new(3.5, -7.0, 10.5, -14.0);
        assert_eq!(t1 * 3.5, t2);
    }

    #[test]
    fn negating_a_tuple() {
        let tuple = Tuple::new_vector(5.0, 6.0, 7.0);
        assert_eq!(-tuple, Tuple::new_vector(-5.0, -6.0, -7.0));
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Tuple::new_vector(0.0, 0.0, 0.0);
        let v = Tuple::new_vector(5.0, 6.0, 7.0);
        assert_eq!(zero - v, Tuple::new_vector(-5.0, -6.0, -7.0));
    }

    #[test]
    fn subtracting_two_vectors_creates_a_vector() {
        let v1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let v2 = Tuple::new_vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, Tuple::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_point_creates_a_point() {
        let point = Tuple::new_point(3.0, 2.0, 1.0);
        let vector = Tuple::new_vector(5.0, 6.0, 7.0);
        assert_eq!(point - vector, Tuple::new_point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_points_creates_a_vector() {
        let p1 = Tuple::new_point(3.0, 2.0, 1.0);
        let p2 = Tuple::new_point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Tuple::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn adding_two_tuples() {
        let vector = Tuple::new_vector(3.0, -2.0, 5.0);
        let point = Tuple::new_point(-2.0, 3.0, 1.0);
        assert_eq!(vector + point, Tuple::new_point(1.0, 1.0, 6.0));
    }

    #[test]
    fn two_equal_vectors() {
        let vec1 = Tuple::new_vector(4.9, -4.1, 3.0);
        let vec2 = Tuple::new_vector(4.9, -4.1, 3.0);
        assert!(vec1.is_equal(&vec2));
    }

    #[test]
    fn two_unequal_vectors() {
        let vec1 = Tuple::new_vector(4.9, -4.1, 3.0);
        let vec2 = Tuple::new_vector(4.9, -4.1, 3.1);
        assert_eq!(vec1.is_equal(&vec2), false);
    }

    #[test]
    fn points_and_vectors_are_not_equal() {
        let vector = Tuple::new_vector(4.9, -4.1, 3.0);
        let point = Tuple::new_point(4.9, -4.1, 3.0);
        assert_eq!(vector.is_equal(&point), false);
    }

    #[test]
    fn tuple_with_0_w_is_a_vector() {
        let tuple = Tuple::new_vector(4.9, -4.1, 3.0);
        assert_eq!(tuple.w, 0.0);
    }

    #[test]
    fn tuple_with_1_w_is_a_point() {
        let tuple = Tuple::new_point(4.9, -4.1, 3.0);
        assert_eq!(tuple.w, 1.0);
    }
}
