use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn hello_world() -> PyResult<String> {
    Ok("Hello World".into())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn native_lib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_world, m)?)?;
    Ok(())
}