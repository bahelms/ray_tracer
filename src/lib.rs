pub mod canvas;
pub mod matrix;
pub mod tuple;

const EPSILON: f64 = 0.00001;

pub fn is_float_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
