mod transformations;

use crate::tuple::Tuple;
use std::ops::{Index, IndexMut, Mul};

/*
* Implemented with a vector of vectors.
*/
#[derive(Debug, PartialEq, Clone)]
struct Matrix {
    rows: Vec<Vec<f64>>,
}

impl Matrix {
    fn new(row_count: i32, col_count: usize) -> Self {
        let mut rows = Vec::new();
        for _ in 0..row_count {
            rows.push(vec![0.0; col_count]);
        }
        Self { rows }
    }

    fn populate(rows: Vec<Vec<f64>>) -> Self {
        Self { rows }
    }

    // hardcoded for 4x4
    fn identity() -> Self {
        Self::populate(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    // changes the rows into columns
    fn transpose(&self) -> Self {
        let mut transposed = self.clone();
        for (col, row) in self.rows.iter().enumerate() {
            for (idx, value) in row.iter().enumerate() {
                transposed[idx][col] = *value;
            }
        }
        transposed
    }

    // recurse for matrices larger than 2x2
    fn determinant(&self) -> f64 {
        if self.rows.len() == 2 {
            self[0][0] * self[1][1] - self[0][1] * self[1][0]
        } else {
            let mut determinant = 0.0;
            for (col, value) in self.rows[0].iter().enumerate() {
                determinant += self.cofactor(0, col) * value;
            }
            determinant
        }
    }

    // drops the row and column at given indexes
    fn submatrix(&self, row_idx: usize, col_idx: usize) -> Self {
        let mut submatrix = Self { rows: Vec::new() };
        for (i, row) in self.rows.iter().enumerate() {
            let mut new_row = Vec::new();
            if i == row_idx {
                continue;
            }
            for (col, value) in row.iter().enumerate() {
                if col == col_idx {
                    continue;
                }
                new_row.push(*value);
            }
            submatrix.rows.push(new_row);
        }
        submatrix
    }

    // determinant of the submatrix of 3x3 matrix
    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    // when specified matrix index is odd, negate the minor
    fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    fn inverse(&self) -> Option<Self> {
        if self.is_invertible() {
            let mut inverted_matrix = self.clone();
            for (row_idx, row) in self.rows.iter().enumerate() {
                for col in 0..row.len() {
                    let cofactor = self.cofactor(row_idx, col);
                    inverted_matrix[col][row_idx] = cofactor / self.determinant();
                }
            }
            Some(inverted_matrix)
        } else {
            None
        }
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}

impl Mul for Matrix {
    type Output = Self;

    // Hardcoded for a 4x4 matrix
    fn mul(self, other: Matrix) -> Self::Output {
        let mut product = self.clone();
        let width = self.rows[0].len();

        for row in 0..width {
            for col in 0..width {
                product[row][col] = self[row][0] * other[0][col]
                    + self[row][1] * other[1][col]
                    + self[row][2] * other[2][col]
                    + self[row][3] * other[3][col];
            }
        }
        product
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    // Hardcoded for a 4x4 matrix
    fn mul(self, other: &Matrix) -> Self::Output {
        let mut product = self.clone();
        let width = self.rows[0].len();

        for row in 0..width {
            for col in 0..width {
                product[row][col] = self[row][0] * other[0][col]
                    + self[row][1] * other[1][col]
                    + self[row][2] * other[2][col]
                    + self[row][3] * other[3][col];
            }
        }
        product
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    // Hardcoded for a 4x4 matrix
    fn mul(self, other: Tuple) -> Self::Output {
        let x = self[0][0] * other.x
            + self[0][1] * other.y
            + self[0][2] * other.z
            + self[0][3] * other.w;
        let y = self[1][0] * other.x
            + self[1][1] * other.y
            + self[1][2] * other.z
            + self[1][3] * other.w;
        let z = self[2][0] * other.x
            + self[2][1] * other.y
            + self[2][2] * other.z
            + self[2][3] * other.w;
        let w = self[3][0] * other.x
            + self[3][1] * other.y
            + self[3][2] * other.z
            + self[3][3] * other.w;
        let mut point = Tuple::point(x, y, z);
        point.w = w;
        point
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_float_equal;

    #[test]
    fn multiplying_a_product_matrix_by_the_inverse_of_an_operand_gets_other_operand() {
        let matrix1 = Matrix::populate(vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ]);
        let matrix2 = Matrix::populate(vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0],
        ]);
        let product = &matrix1 * &matrix2;
        let result = product * matrix2.inverse().unwrap();
        assert!(is_float_equal(result[0][0], matrix1[0][0]));
        assert!(is_float_equal(result[0][1], matrix1[0][1]));
        assert!(is_float_equal(result[0][2], matrix1[0][2]));
        assert!(is_float_equal(result[0][3], matrix1[0][3]));
        assert!(is_float_equal(result[1][0], matrix1[1][0]));
        assert!(is_float_equal(result[1][1], matrix1[1][1]));
        assert!(is_float_equal(result[1][2], matrix1[1][2]));
        assert!(is_float_equal(result[1][3], matrix1[1][3]));
        assert!(is_float_equal(result[2][0], matrix1[2][0]));
        assert!(is_float_equal(result[2][1], matrix1[2][1]));
        assert!(is_float_equal(result[2][2], matrix1[2][2]));
        assert!(is_float_equal(result[2][3], matrix1[2][3]));
        assert!(is_float_equal(result[3][0], matrix1[3][0]));
        assert!(is_float_equal(result[3][1], matrix1[3][1]));
        assert!(is_float_equal(result[3][2], matrix1[3][2]));
        assert!(is_float_equal(result[3][3], matrix1[3][3]));
    }

    #[test]
    fn inverting_an_uninvertible_matrix_returns_none() {
        let matrix = Matrix::populate(vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(matrix.inverse(), None);
    }

    #[test]
    fn inverting_a_matrix_3() {
        let matrix = Matrix::populate(vec![
            vec![9.0, 3.0, 0.0, 9.0],
            vec![-5.0, -2.0, -6.0, -3.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0],
        ]);
        let expected_inverse = Matrix::populate(vec![
            vec![-0.04074, -0.07778, 0.14444, -0.22222],
            vec![-0.07778, 0.03333, 0.36667, -0.33333],
            vec![-0.02901, -0.14630, -0.10926, 0.12963],
            vec![0.17778, 0.06667, -0.26667, 0.33333],
        ]);
        let inverse = matrix.inverse().unwrap();
        assert!(is_float_equal(inverse[0][0], expected_inverse[0][0]));
        assert!(is_float_equal(inverse[0][1], expected_inverse[0][1]));
        assert!(is_float_equal(inverse[0][2], expected_inverse[0][2]));
        assert!(is_float_equal(inverse[0][3], expected_inverse[0][3]));
        assert!(is_float_equal(inverse[1][0], expected_inverse[1][0]));
        assert!(is_float_equal(inverse[1][1], expected_inverse[1][1]));
        assert!(is_float_equal(inverse[1][2], expected_inverse[1][2]));
        assert!(is_float_equal(inverse[1][3], expected_inverse[1][3]));
        assert!(is_float_equal(inverse[2][0], expected_inverse[2][0]));
        assert!(is_float_equal(inverse[2][1], expected_inverse[2][1]));
        assert!(is_float_equal(inverse[2][2], expected_inverse[2][2]));
        assert!(is_float_equal(inverse[2][3], expected_inverse[2][3]));
        assert!(is_float_equal(inverse[3][0], expected_inverse[3][0]));
        assert!(is_float_equal(inverse[3][1], expected_inverse[3][1]));
        assert!(is_float_equal(inverse[3][2], expected_inverse[3][2]));
        assert!(is_float_equal(inverse[3][3], expected_inverse[3][3]));
    }

    #[test]
    fn inverting_a_matrix_2() {
        let matrix = Matrix::populate(vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0],
        ]);
        let expected_inverse = Matrix::populate(vec![
            vec![-0.15385, -0.15385, -0.28205, -0.53846],
            vec![-0.07692, 0.12308, 0.02564, 0.03077],
            vec![0.35897, 0.35897, 0.43590, 0.92308],
            vec![-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        let inverse = matrix.inverse().unwrap();
        assert!(is_float_equal(inverse[0][0], expected_inverse[0][0]));
        assert!(is_float_equal(inverse[0][1], expected_inverse[0][1]));
        assert!(is_float_equal(inverse[0][2], expected_inverse[0][2]));
        assert!(is_float_equal(inverse[0][3], expected_inverse[0][3]));
        assert!(is_float_equal(inverse[1][0], expected_inverse[1][0]));
        assert!(is_float_equal(inverse[1][1], expected_inverse[1][1]));
        assert!(is_float_equal(inverse[1][2], expected_inverse[1][2]));
        assert!(is_float_equal(inverse[1][3], expected_inverse[1][3]));
        assert!(is_float_equal(inverse[2][0], expected_inverse[2][0]));
        assert!(is_float_equal(inverse[2][1], expected_inverse[2][1]));
        assert!(is_float_equal(inverse[2][2], expected_inverse[2][2]));
        assert!(is_float_equal(inverse[2][3], expected_inverse[2][3]));
        assert!(is_float_equal(inverse[3][0], expected_inverse[3][0]));
        assert!(is_float_equal(inverse[3][1], expected_inverse[3][1]));
        assert!(is_float_equal(inverse[3][2], expected_inverse[3][2]));
        assert!(is_float_equal(inverse[3][3], expected_inverse[3][3]));
    }

    #[test]
    fn inverting_a_matrix_1() {
        let matrix = Matrix::populate(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ]);
        let expected_inverse = Matrix::populate(vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        assert!(matrix.is_invertible());
        assert_eq!(matrix.determinant(), 532.0);
        let inverse = matrix.inverse().unwrap();
        assert_eq!(matrix.cofactor(2, 3), -160.0);
        assert_eq!(inverse[3][2], -160.0 / 532.0);
        assert_eq!(matrix.cofactor(3, 2), 105.0);
        assert_eq!(inverse[2][3], 105.0 / 532.0);
        assert!(is_float_equal(inverse[0][0], expected_inverse[0][0]));
        assert!(is_float_equal(inverse[0][1], expected_inverse[0][1]));
        assert!(is_float_equal(inverse[0][2], expected_inverse[0][2]));
        assert!(is_float_equal(inverse[0][3], expected_inverse[0][3]));
        assert!(is_float_equal(inverse[1][0], expected_inverse[1][0]));
        assert!(is_float_equal(inverse[1][1], expected_inverse[1][1]));
        assert!(is_float_equal(inverse[1][2], expected_inverse[1][2]));
        assert!(is_float_equal(inverse[1][3], expected_inverse[1][3]));
        assert!(is_float_equal(inverse[2][0], expected_inverse[2][0]));
        assert!(is_float_equal(inverse[2][1], expected_inverse[2][1]));
        assert!(is_float_equal(inverse[2][2], expected_inverse[2][2]));
        assert!(is_float_equal(inverse[2][3], expected_inverse[2][3]));
        assert!(is_float_equal(inverse[3][0], expected_inverse[3][0]));
        assert!(is_float_equal(inverse[3][1], expected_inverse[3][1]));
        assert!(is_float_equal(inverse[3][2], expected_inverse[3][2]));
        assert!(is_float_equal(inverse[3][3], expected_inverse[3][3]));
    }

    #[test]
    fn non_invertible_matrices_have_determinants_of_zero() {
        let matrix = Matrix::populate(vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(matrix.determinant(), 0.0);
        assert!(!matrix.is_invertible());
    }

    #[test]
    fn invertible_matrices_have_non_zero_determinants() {
        let matrix = Matrix::populate(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(matrix.determinant(), -2120.0);
        assert!(matrix.is_invertible());
    }

    #[test]
    fn calculate_determinant_of_4x4_matrix() {
        let matrix = Matrix::populate(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(matrix.cofactor(0, 0), 690.0);
        assert_eq!(matrix.cofactor(0, 1), 447.0);
        assert_eq!(matrix.cofactor(0, 2), 210.0);
        assert_eq!(matrix.cofactor(0, 3), 51.0);
        assert_eq!(matrix.determinant(), -4071.0);
    }

    #[test]
    fn calculate_determinant_of_3x3_matrix() {
        let matrix = Matrix::populate(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ]);
        assert_eq!(matrix.cofactor(0, 0), 56.0);
        assert_eq!(matrix.cofactor(0, 1), 12.0);
        assert_eq!(matrix.cofactor(0, 2), -46.0);
        assert_eq!(matrix.determinant(), -196.0);
    }

    #[test]
    fn calculate_determinant_of_2x2_matrix() {
        let matrix = Matrix::populate(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);
        assert_eq!(matrix.determinant(), 17.0);
    }

    #[test]
    fn cofactor_of_a_matrix() {
        let matrix = Matrix::populate(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        assert_eq!(matrix.cofactor(0, 0), -12.0);
        assert_eq!(matrix.cofactor(1, 0), -25.0);
    }

    #[test]
    fn minor_is_the_determinant_of_the_submatrix_of_3x3_matrix() {
        let matrix = Matrix::populate(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        assert_eq!(matrix.minor(1, 0), 25.0);
    }

    #[test]
    fn submatrix_returns_matrix_with_given_row_and_col_removed() {
        let matrix = Matrix::populate(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);
        let submatrix_3x3 = Matrix::populate(vec![
            vec![0.0, 9.0, 0.0],
            vec![9.0, 8.0, 8.0],
            vec![0.0, 0.0, 8.0],
        ]);
        let submatrix_2x2 = Matrix::populate(vec![vec![9.0, 8.0], vec![0.0, 8.0]]);
        assert_eq!(matrix.submatrix(2, 2), submatrix_3x3);
        assert_eq!(submatrix_3x3.submatrix(0, 1), submatrix_2x2);
    }

    #[test]
    fn transposing_a_the_identity_matrix_returns_the_identity() {
        assert_eq!(Matrix::identity().transpose(), Matrix::identity());
    }

    #[test]
    fn transposing_a_matrix_turns_the_rows_into_columns() {
        let matrix = Matrix::populate(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);
        let transposed = Matrix::populate(vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ]);
        assert_eq!(matrix.transpose(), transposed);
    }

    #[test]
    fn multiplying_identity_matrix_with_point_returns_point() {
        let tuple = Tuple::point(1.0, 2.0, 3.0);
        let identity = Matrix::identity();
        assert_eq!(identity * tuple, tuple);
    }

    #[test]
    fn multiplying_matrix_with_identity_matrix_returns_matrix() {
        let matrix1 = Matrix::populate(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let identity = Matrix::identity();
        assert_eq!(matrix1.clone() * identity, matrix1);
    }

    #[test]
    fn multiplying_a_matrix_with_a_point_returns_point() {
        let matrix = Matrix::populate(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        let tuple = Tuple::point(1.0, 2.0, 3.0);
        assert_eq!(matrix * tuple, Tuple::point(18.0, 24.0, 33.0));
    }

    #[test]
    fn multiplying_matrices() {
        let matrix1 = Matrix::populate(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let matrix2 = Matrix::populate(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);
        let product = matrix1 * matrix2;
        assert_eq!(
            product,
            Matrix::populate(vec![
                vec![20.0, 22.0, 50.0, 48.0],
                vec![44.0, 54.0, 114.0, 108.0],
                vec![40.0, 58.0, 110.0, 102.0],
                vec![16.0, 26.0, 46.0, 42.0],
            ])
        );
    }

    #[test]
    fn different_matrices_compare_as_false() {
        let matrix1 = Matrix::populate(vec![vec![1.0, 2.0, 3.1, 4.0], vec![5.0, 6.0, 7.0, 8.0]]);
        let matrix2 = Matrix::populate(vec![vec![1.0, 2.0, 3.0, 4.0], vec![5.0, 6.0, 7.0, 8.0]]);
        assert_ne!(matrix1, matrix2);
    }

    #[test]
    fn identical_matrices_compare_as_true() {
        let matrix1 = Matrix::populate(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.0, 14.0, 15.0, 16.0],
        ]);
        let matrix2 = Matrix::populate(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.0, 14.0, 15.0, 16.0],
        ]);
        assert_eq!(matrix1, matrix2);
    }

    #[test]
    fn populating_a_matrix() {
        let matrix = Matrix::populate(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.0, 14.0, 15.0, 16.0],
        ]);
        assert_eq!(matrix[3][3], 16.0);
    }

    #[test]
    fn inserting_into_a_matrix() {
        let mut matrix = Matrix::new(4, 4);
        matrix[2][3] = 12.0;
        assert_eq!(matrix[2][3], 12.0);
    }

    #[test]
    #[should_panic]
    fn accessing_matrix_out_of_bounds() {
        let matrix = Matrix::new(4, 4);
        assert_eq!(matrix[4][3], 0.0);
    }

    #[test]
    fn creating_and_accessing_a_default_matrix() {
        let matrix = Matrix::new(4, 4);
        assert_eq!(matrix[2][3], 0.0);
        assert_eq!(matrix[0][0], 0.0);

        let matrix = Matrix::new(2, 2);
        assert_eq!(matrix[0][0], 0.0);
        assert_eq!(matrix[1][1], 0.0);
    }
}
