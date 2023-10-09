use pyo3::prelude::*;

#[pyfunction]
fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[pyfunction]
fn sub(a: f64, b: f64) -> f64 {
    a - b
}

#[pyfunction]
fn mul(a: f64, b: f64) -> f64 {
    a * b
}

#[pyfunction]
fn div(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        return Err(pyo3::exceptions::PyZeroDivisionError::new_err("Division By Zero!"));
    }
    let result = a / b;
    Ok(result)
    
}

#[pymodule]
fn pyproject(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(sub, m)?)?;
    m.add_function(wrap_pyfunction!(mul, m)?)?;
    m.add_function(wrap_pyfunction!(div, m)?)?;
    Ok(())
}



