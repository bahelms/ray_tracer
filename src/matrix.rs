use crate::{
    is_float_equal,
    tuple::{Point, Tuple},
};
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

impl Mul<Point> for Matrix {
    type Output = Point;

    // Hardcoded for a 4x4 matrix
    fn mul(self, other: Point) -> Self::Output {
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
        let mut point = Point::new(x, y, z);
        point.w = w;
        point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplying_a_matrix_with_a_tuple_returns_tuple() {
        let matrix = Matrix::populate(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        let tuple = Point::new(1.0, 2.0, 3.0);
        assert_eq!(matrix * tuple, Point::new(18.0, 24.0, 33.0));
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
