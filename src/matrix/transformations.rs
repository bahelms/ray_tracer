use super::Matrix;

fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut matrix = Matrix::identity();
    matrix[0][3] = x;
    matrix[1][3] = y;
    matrix[2][3] = z;
    matrix
}

fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut matrix = Matrix::identity();
    matrix[0][0] = x;
    matrix[1][1] = y;
    matrix[2][2] = z;
    matrix
}

fn rotation_x(radians: f64) -> Matrix {
    let mut matrix = Matrix::identity();
    matrix[1][1] = radians.cos();
    matrix[1][2] = -radians.sin();
    matrix[2][1] = radians.sin();
    matrix[2][2] = radians.cos();
    matrix
}

fn rotation_y(radians: f64) -> Matrix {
    let mut matrix = Matrix::identity();
    matrix[0][0] = radians.cos();
    matrix[0][2] = radians.sin();
    matrix[2][0] = -radians.sin();
    matrix[2][2] = radians.cos();
    matrix
}

fn rotation_z(radians: f64) -> Matrix {
    let mut matrix = Matrix::identity();
    matrix[0][0] = radians.cos();
    matrix[0][1] = -radians.sin();
    matrix[1][0] = radians.sin();
    matrix[1][1] = radians.cos();
    matrix
}

fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut matrix = Matrix::identity();
    matrix[0][1] = xy;
    matrix[0][2] = xz;
    matrix[1][0] = yx;
    matrix[1][2] = yz;
    matrix[2][0] = zx;
    matrix[2][1] = zy;
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_float_equal;
    use crate::tuple::Tuple;
    use std::f64::consts::PI;

    #[test]
    fn chained_transformations_must_be_in_reverse_order() {
        let mut point = Tuple::point(1.0, 0.0, 1.0);
        point = translation(10.0, 5.0, 7.0) * scaling(5.0, 5.0, 5.0) * rotation_x(PI / 2.0) * point;
        assert!(is_float_equal(point.x, 15.0));
        assert!(is_float_equal(point.y, 0.0));
        assert!(is_float_equal(point.z, 7.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let point = Tuple::point(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(transform * point, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let point = Tuple::point(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(transform * point, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let point = Tuple::point(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(transform * point, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let point = Tuple::point(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert_eq!(transform * point, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let point = Tuple::point(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(transform * point, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let point = Tuple::point(2.0, 3.0, 4.0);
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(transform * point, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_a_point_around_z_axis() {
        let point = Tuple::point(0.0, 1.0, 0.0);
        let eighth = rotation_z(PI / 4.0);
        let quarter = rotation_z(PI / 2.0);

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
        let eighth = rotation_y(PI / 4.0);
        let quarter = rotation_y(PI / 2.0);

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
        let eighth = rotation_x(PI / 4.0);

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
        let eighth = rotation_x(PI / 4.0);
        let quarter = rotation_x(PI / 2.0);

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
            scaling(-1.0, 1.0, 1.0) * Tuple::point(2.0, 3.0, 4.0),
            Tuple::point(-2.0, 3.0, 4.0)
        );
    }

    #[test]
    fn multiply_scaling_matrix_inverse_with_a_point() {
        assert_eq!(
            scaling(2.0, 3.0, 4.0).inverse().unwrap() * Tuple::vector(-4.0, 6.0, 8.0),
            Tuple::vector(-2.0, 2.0, 2.0)
        );
    }

    #[test]
    fn multiply_scaling_matrix_with_a_vector() {
        assert_eq!(
            scaling(2.0, 3.0, 4.0) * Tuple::vector(-4.0, 6.0, 8.0),
            Tuple::vector(-8.0, 18.0, 32.0)
        );
    }

    #[test]
    fn multiply_scaling_matrix_with_a_point() {
        assert_eq!(
            scaling(2.0, 3.0, 4.0) * Tuple::point(-4.0, 6.0, 8.0),
            Tuple::point(-8.0, 18.0, 32.0)
        );
    }

    #[test]
    fn multiply_translation_matrix_with_a_vector_does_not_change_vector() {
        let vector = Tuple::vector(-3.0, 4.0, 5.0);
        assert_eq!(translation(5.0, -3.0, 2.0) * vector, vector);
    }

    #[test]
    fn multiply_translation_matrix_inverse_with_a_point() {
        assert_eq!(
            translation(5.0, -3.0, 2.0).inverse().unwrap() * Tuple::point(-3.0, 4.0, 5.0),
            Tuple::point(-8.0, 7.0, 3.0)
        );
    }

    #[test]
    fn multiply_translation_matrix_with_a_point() {
        assert_eq!(
            translation(5.0, -3.0, 2.0) * Tuple::point(-3.0, 4.0, 5.0),
            Tuple::point(2.0, 1.0, 7.0)
        );
    }
}
