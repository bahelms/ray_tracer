use super::Matrix;

impl Matrix {
    pub fn translate(&self, x: f64, y: f64, z: f64) -> Self {
        let mut transform = Matrix::identity();
        transform[0][3] = x;
        transform[1][3] = y;
        transform[2][3] = z;
        &transform * self
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Self {
        let mut transform = Matrix::identity();
        transform[0][0] = x;
        transform[1][1] = y;
        transform[2][2] = z;
        &transform * self
    }

    fn rotate_x(&self, radians: f64) -> Self {
        let mut transform = Matrix::identity();
        transform[1][1] = radians.cos();
        transform[1][2] = -radians.sin();
        transform[2][1] = radians.sin();
        transform[2][2] = radians.cos();
        &transform * self
    }

    pub fn rotate_y(&self, radians: f64) -> Self {
        let mut transform = Matrix::identity();
        transform[0][0] = radians.cos();
        transform[0][2] = radians.sin();
        transform[2][0] = -radians.sin();
        transform[2][2] = radians.cos();
        &transform * self
    }

    pub fn rotate_z(&self, radians: f64) -> Self {
        let mut transform = Matrix::identity();
        transform[0][0] = radians.cos();
        transform[0][1] = -radians.sin();
        transform[1][0] = radians.sin();
        transform[1][1] = radians.cos();
        &transform * self
    }

    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let mut transform = Matrix::identity();
        transform[0][1] = xy;
        transform[0][2] = xz;
        transform[1][0] = yx;
        transform[1][2] = yz;
        transform[2][0] = zx;
        transform[2][1] = zy;
        &transform * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_float_equal;
    use crate::tuple::Tuple;
    use std::f64::consts::PI;

    #[test]
    fn chaining_transformations() {
        let transform = Matrix::identity()
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);
        let point = transform * Tuple::point(1.0, 0.0, 1.0);
        assert!(is_float_equal(point.x, 15.0));
        assert!(is_float_equal(point.y, 0.0));
        assert!(is_float_equal(point.z, 7.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let point = Tuple::point(2.0, 3.0, 4.0);
        let transform = Matrix::identity().shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(transform * point, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let point = Tuple::point(2.0, 3.0, 4.0);
        let transform = Matrix::identity().shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(transform * point, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let point = Tuple::point(2.0, 3.0, 4.0);
        let transform = Matrix::identity().shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(transform * point, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let point = Tuple::point(2.0, 3.0, 4.0);
        let transform = Matrix::identity().shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert_eq!(transform * point, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let point = Tuple::point(2.0, 3.0, 4.0);
        let transform = Matrix::identity().shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(transform * point, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let point = Tuple::point(2.0, 3.0, 4.0);
        let transform = Matrix::identity().shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(transform * point, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_a_point_around_z_axis() {
        let point = Tuple::point(0.0, 1.0, 0.0);
        let matrix = Matrix::identity();
        let eighth = matrix.rotate_z(PI / 4.0);
        let quarter = matrix.rotate_z(PI / 2.0);

        let eighth_point = eighth * point;
        assert!(is_float_equal(eighth_point.x, -(2.0_f64.sqrt()) / 2.0));
        assert!(is_float_equal(eighth_point.y, 2.0_f64.sqrt() / 2.0));
        assert_eq!(eighth_point.z, 0.0);

        let quarter_point = quarter * point;
        assert!(is_float_equal(quarter_point.x, -1.0));
        assert!(is_float_equal(quarter_point.y, 0.0));
        assert!(is_float_equal(quarter_point.z, 0.0));
    }

    #[test]
    fn rotating_a_point_around_y_axis() {
        let point = Tuple::point(0.0, 0.0, 1.0);
        let matrix = Matrix::identity();
        let eighth = matrix.rotate_y(PI / 4.0);
        let quarter = matrix.rotate_y(PI / 2.0);

        let eighth_point = eighth * point;
        assert!(is_float_equal(eighth_point.x, 2.0_f64.sqrt() / 2.0));
        assert_eq!(eighth_point.y, 0.0);
        assert!(is_float_equal(eighth_point.z, 2.0_f64.sqrt() / 2.0));

        let quarter_point = quarter * point;
        assert!(is_float_equal(quarter_point.x, 1.0));
        assert_eq!(quarter_point.y, 0.0);
        assert!(is_float_equal(quarter_point.z, 0.0));
    }

    #[test]
    fn rotation_x_inverse_rotates_in_opposite_direction() {
        let point = Tuple::point(0.0, 1.0, 0.0);
        let eighth = Matrix::identity().rotate_x(PI / 4.0);

        let eighth_inverse_point = eighth.inverse().unwrap() * point;
        assert_eq!(eighth_inverse_point.x, 0.0);
        assert_eq!(eighth_inverse_point.y, 2.0_f64.sqrt() / 2.0);
        assert!(is_float_equal(
            eighth_inverse_point.z,
            -(2.0_f64.sqrt()) / 2.0
        ));
    }

    #[test]
    fn rotating_a_point_around_x_axis() {
        let point = Tuple::point(0.0, 1.0, 0.0);
        let matrix = Matrix::identity();
        let eighth = matrix.rotate_x(PI / 4.0);
        let quarter = matrix.rotate_x(PI / 2.0);

        let eighth_point = eighth * point;
        assert_eq!(eighth_point.x, 0.0);
        assert_eq!(eighth_point.y, 2.0_f64.sqrt() / 2.0);
        assert!(is_float_equal(eighth_point.z, 2.0_f64.sqrt() / 2.0));

        let quarter_point = quarter * point;
        assert_eq!(quarter_point.x, 0.0);
        assert!(is_float_equal(quarter_point.y, 0.0));
        assert!(is_float_equal(quarter_point.z, 1.0));
    }

    #[test]
    fn scaling_with_a_negative_value_reflects_the_tuple() {
        assert_eq!(
            Matrix::identity().scale(-1.0, 1.0, 1.0) * Tuple::point(2.0, 3.0, 4.0),
            Tuple::point(-2.0, 3.0, 4.0)
        );
    }

    #[test]
    fn multiply_scaling_matrix_inverse_with_a_point() {
        assert_eq!(
            Matrix::identity().scale(2.0, 3.0, 4.0).inverse().unwrap()
                * Tuple::vector(-4.0, 6.0, 8.0),
            Tuple::vector(-2.0, 2.0, 2.0)
        );
    }

    #[test]
    fn multiply_scaling_matrix_with_a_vector() {
        assert_eq!(
            Matrix::identity().scale(2.0, 3.0, 4.0) * Tuple::vector(-4.0, 6.0, 8.0),
            Tuple::vector(-8.0, 18.0, 32.0)
        );
    }

    #[test]
    fn multiply_scaling_matrix_with_a_point() {
        assert_eq!(
            Matrix::identity().scale(2.0, 3.0, 4.0) * Tuple::point(-4.0, 6.0, 8.0),
            Tuple::point(-8.0, 18.0, 32.0)
        );
    }

    #[test]
    fn multiply_translation_matrix_with_a_vector_does_not_change_vector() {
        let vector = Tuple::vector(-3.0, 4.0, 5.0);
        assert_eq!(
            Matrix::identity().translate(5.0, -3.0, 2.0) * vector,
            vector
        );
    }

    #[test]
    fn multiply_translation_matrix_inverse_with_a_point() {
        assert_eq!(
            Matrix::identity()
                .translate(5.0, -3.0, 2.0)
                .inverse()
                .unwrap()
                * Tuple::point(-3.0, 4.0, 5.0),
            Tuple::point(-8.0, 7.0, 3.0)
        );
    }

    #[test]
    fn multiply_translation_matrix_with_a_point() {
        assert_eq!(
            Matrix::identity().translate(5.0, -3.0, 2.0) * Tuple::point(-3.0, 4.0, 5.0),
            Tuple::point(2.0, 1.0, 7.0)
        );
    }
}
