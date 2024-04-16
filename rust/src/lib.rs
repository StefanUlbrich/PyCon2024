//! Gaussian Mixtures in Rust
#![allow(unused)]

use ndarray::prelude::*;

/// Maximization step in the EM algorithm
pub fn maximize(data: ArrayView2<f64>, responsibilities: ArrayView2<f64>) -> (Array2<f64>, Array3<f64>, Array1<f64>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray_npy::read_npy;

    #[test]
    fn test_maximize() {}
}
