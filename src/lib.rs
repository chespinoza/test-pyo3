#![feature(specialization)]

#[macro_use]
extern crate pyo3;

use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a+b).to_string())
}

#[pymodinit]
fn string_sum(py: Python, m: &PyModule) -> PyResult<()>{
    m.add_function(wrap_function!(sum_as_string))?;
    Ok(())
}