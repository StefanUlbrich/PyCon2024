//! bindings for gaussian_mixtures
#![allow(unused)]

use numpy::{IntoPyArray, PyArray1, PyArray2, PyArray3, PyReadonlyArray2};
use pyo3::{pymodule, types::PyModule, PyResult, Python};

use ::gaussian_mixtures::maximize;

#[pymodule]
fn gaussian_mixtures(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "maximize")]
    fn maximize_py<'py>(
        py: Python<'py>,
        data: PyReadonlyArray2<f64>,
        responsibilities: PyReadonlyArray2<f64>,
    ) -> (&'py PyArray2<f64>, &'py PyArray3<f64>, &'py PyArray1<f64>) {
        todo!()
    }
    Ok(())
}
