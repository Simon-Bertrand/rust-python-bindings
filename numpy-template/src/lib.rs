
use ndarray;
use pyo3::prelude::*;
use numpy::{PyArray2, IntoPyArray};

#[pymodule]
fn numpy_lib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "eye")]
    fn eye<'py>(py: Python<'py>, size: usize) -> &PyArray2<f64> {
        let array = ndarray::Array::eye(size);
        array.into_pyarray(py)
    }
    Ok(())
}

 