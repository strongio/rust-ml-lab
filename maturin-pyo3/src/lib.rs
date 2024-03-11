use pyo3::prelude::*;
use rand::{thread_rng, Rng};

#[pyfunction]
fn fib(n: isize) -> PyResult<u128> {
    if n == 1 {
        return Ok(1);
    } else if n == 2 {
        return Ok(2);
    }

    let mut sum: u128 = 0;
    let mut last: u128 = 0;
    let mut curr: u128 = 1;
    for _ in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    Ok(sum)
}

#[pyfunction]
fn estimate_pi(num_samples: usize) -> PyResult<f32> {
    let mut rng = thread_rng();
    let mut inside_circle = 0;

    for _ in 0..num_samples {
        let x: f32 = rng.gen_range(-1.0..1.0);
        let y: f32 = rng.gen_range(-1.0..1.0);
        if x.powi(2) + y.powi(2) <= 1.0 {
            inside_circle += 1;
        }
    }
    Ok(4.0 * (inside_circle as f32) / (num_samples as f32))
}

/// A Python module implemented in Rust.
#[pymodule]
fn maturin_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fib, m)?)?;
    m.add_function(wrap_pyfunction!(estimate_pi, m)?)?;
    Ok(())
}
