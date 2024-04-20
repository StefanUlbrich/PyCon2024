//! Gaussian Mixtures in Rust
#![allow(unused)]

use itertools::izip;
use ndarray::{
    parallel::prelude::{IntoParallelIterator, ParallelIterator},
    prelude::*,
};

fn maximize_index(
    data: ArrayView2<f64>,
    responsibilities: ArrayView2<f64>,
) -> (Array2<f64>, Array3<f64>, Array1<f64>) {
    let k = *responsibilities.shape().get(1).unwrap();
    let d = *data.shape().get(1).unwrap();

    // Similar to the Python code
    let sum_responsibilities = responsibilities.sum_axis(Axis(0));

    let means = (&responsibilities.slice(s![.., .., NewAxis])
        * &data.slice(s![.., NewAxis, ..]))
        .sum_axis(Axis(0))
        / sum_responsibilities.slice(s![.., NewAxis]);

    // n x k x d
    let adjusted =
        &data.slice(s![.., NewAxis, ..]) - &means.slice(s![NewAxis, .., ..]);

    // Initialize memory
    let mut covs = Array3::<f64>::zeros((k, d, d));

    for i in 0..k {
        let x = adjusted.slice(s![.., i, ..]);
        let y = &x * &responsibilities.slice(s![.., i, NewAxis]);

        covs.slice_mut(s![i, .., ..]).assign(&x.t().dot(&y));
    }

    covs = &covs / &sum_responsibilities.slice(s![.., NewAxis, NewAxis]);

    let weights = &sum_responsibilities / sum_responsibilities.sum();

    (means, covs, weights)
}

fn maximize_iter(
    data: ArrayView2<f64>,
    responsibilities: ArrayView2<f64>,
) -> (Array2<f64>, Array3<f64>, Array1<f64>) {
    let k = *responsibilities.shape().get(1).unwrap();
    let d = *data.shape().get(1).unwrap();

    // Similar to the Python code
    let sum_responsibilities = responsibilities.sum_axis(Axis(0));

    let means = (&responsibilities.slice(s![.., .., NewAxis])
        * &data.slice(s![.., NewAxis, ..]))
        .sum_axis(Axis(0))
        / sum_responsibilities.slice(s![.., NewAxis]);

    // n x k x d
    let adjusted =
        &data.slice(s![.., NewAxis, ..]) - &means.slice(s![NewAxis, .., ..]);

    // Initialize memory
    let mut covs = Array3::<f64>::zeros((k, d, d));

    for (x, mut c, r) in izip!(
        adjusted.axis_iter(Axis(1)),
        covs.axis_iter_mut(Axis(0)),
        responsibilities.axis_iter(Axis(1))
    ) {
        let y = &x * &r.slice(s![.., NewAxis]);
        c += &x.t().dot(&y);
    }

    covs = &covs / &sum_responsibilities.slice(s![.., NewAxis, NewAxis]);

    let weights = &sum_responsibilities / sum_responsibilities.sum();

    (means, covs, weights)
}

fn maximize_func(
    data: ArrayView2<f64>,
    responsibilities: ArrayView2<f64>,
) -> (Array2<f64>, Array3<f64>, Array1<f64>) {
    let k = *responsibilities.shape().get(1).unwrap();
    let d = *data.shape().get(1).unwrap();

    // Similar to the Python code
    let sum_responsibilities = responsibilities.sum_axis(Axis(0));

    let means = (&responsibilities.slice(s![.., .., NewAxis])
        * &data.slice(s![.., NewAxis, ..]))
        .sum_axis(Axis(0))
        / sum_responsibilities.slice(s![.., NewAxis]);

    // n x k x d
    let adjusted =
        &data.slice(s![.., NewAxis, ..]) - &means.slice(s![NewAxis, .., ..]);

    // Initialize memory
    let mut covs = Array3::<f64>::zeros((k, d, d));

    izip!(
        adjusted.axis_iter(Axis(1)),
        covs.axis_iter_mut(Axis(0)),
        responsibilities.axis_iter(Axis(1))
    )
    .for_each(|(x, mut cov, resp)| {
        let y = &x * &resp.slice(s![.., NewAxis]);
        cov += &x.t().dot(&y);
    });

    covs = &covs / &sum_responsibilities.slice(s![.., NewAxis, NewAxis]);

    let weights = &sum_responsibilities / sum_responsibilities.sum();

    (means, covs, weights)
}

/// Maximization step in the EM algorithm
pub fn maximize_par(
    data: ArrayView2<f64>,
    responsibilities: ArrayView2<f64>,
) -> (Array2<f64>, Array3<f64>, Array1<f64>) {
    let k = *responsibilities.shape().get(1).unwrap();
    let d = *data.shape().get(1).unwrap();

    let sum_responsibilities = responsibilities.sum_axis(Axis(0));

    let means = (&responsibilities.slice(s![.., .., NewAxis])
        * &data.slice(s![.., NewAxis, ..]))
        .sum_axis(Axis(0))
        / sum_responsibilities.slice(s![.., NewAxis]);

    // n x k x d
    let adjusted =
        &data.slice(s![.., NewAxis, ..]) - &means.slice(s![NewAxis, .., ..]);

    let mut covs = Array3::<f64>::zeros((k, d, d));

    (
        adjusted.axis_iter(Axis(1)),
        covs.axis_iter_mut(Axis(0)),
        responsibilities.axis_iter(Axis(1)),
    )
        .into_par_iter()
        .for_each(|(x, mut cov, resp)| {
            let y = &x * &resp.slice(s![.., NewAxis]);
            cov += &x.t().dot(&y);
        });

    covs = &covs / &sum_responsibilities.slice(s![.., NewAxis, NewAxis]);

    let weights = &sum_responsibilities / sum_responsibilities.sum();

    (means, covs, weights)
}

pub fn maximize(
    data: ArrayView2<f64>,
    responsibilities: ArrayView2<f64>,
) -> (Array2<f64>, Array3<f64>, Array1<f64>) {
    maximize_par(data, responsibilities)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray_npy::read_npy;

    #[test]
    fn test_maximize() {
        let data: Array2<f64> = read_npy("data/data.npy").unwrap();
        let responsibilities: Array2<f64> =
            read_npy("data/responsibilities.npy").unwrap();
        let means: Array2<f64> = read_npy("data/means.npy").unwrap();
        let covs: Array3<f64> = read_npy("data/covs.npy").unwrap();
        let weights: Array1<f64> = read_npy("data/weights.npy").unwrap();

        // warning the responsibilities are accidentally transposed
        let (means_computed, covs_computed, weights_computed) =
            maximize(data.view(), responsibilities.view().t());

        println!("{}", means_computed);

        assert!(means_computed.abs_diff_eq(&means, 1e-9));
        assert!(covs_computed.abs_diff_eq(&covs, 1e-9));
        assert!(weights_computed.abs_diff_eq(&weights, 1e-9));
    }
}
