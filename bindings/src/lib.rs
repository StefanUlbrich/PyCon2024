//! bindings for gaussian_mixtures
#![allow(unused, clippy::type_complexity)]

use numpy::{IntoPyArray, PyArray1, PyArray2, PyArray3, PyReadonlyArray2};
use pyo3::prelude::*;

use ::gaussian_mixtures::maximize;

#[pymodule]
fn gaussian_mixtures(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "maximize")]
    fn maximize_py<'py>(
        py: Python<'py>,
        data: PyReadonlyArray2<f64>,
        responsibilities: PyReadonlyArray2<f64>,
    ) -> (
        Bound<'py, PyArray2<f64>>,
        Bound<'py, PyArray3<f64>>,
        Bound<'py, PyArray1<f64>>,
    ) {
        let data = data.as_array();
        let responsibilities = responsibilities.as_array();
        let (means, covs, weights) = maximize(data, responsibilities);
        (
            means.into_pyarray_bound(py),
            covs.into_pyarray_bound(py),
            weights.into_pyarray_bound(py),
        )
    }
    Ok(())
}
