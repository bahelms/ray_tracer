use crate::is_float_equal;
use std::ops::{Index, IndexMut};

/*
* Implemented with a vector of vectors.
*/
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

#[cfg(test)]
mod tests {
    use super::*;

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
