//! bindings for gaussian_mixtures
#![allow(unused)]

use numpy::{IntoPyArray, PyArray1, PyArray2, PyArray3, PyReadonlyArray2};
use pyo3::prelude::*;

use ::gaussian_mixtures::hello;

#[pymodule]
fn gaussian_mixtures(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "hello")]
    fn hello_py(py: Python<'_>) {
        todo!()
    }
    Ok(())
}
