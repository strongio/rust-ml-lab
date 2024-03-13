# Rust - Python Bindings with PyO3 and Maturin

## Setup

Start by installing `maturin`
```bash
pip install maturin
```

### (Skip) Initialize the project
This step is already done, but it's included here for illustrative purposes.

Initialize the project
```bash
maturin init
```
Choose `pyo3` as the binding crate.
In the end you should have a directory that looks like this:
```
.
├── Cargo.toml
├── README.md
├── pyproject.toml
└── src
    └── lib.rs
```

## Build Rust Library and Install Python Package

Maturin will build the Rust library and install the Python package in one step.
```bash
maturin develop
```

## Use the Python Package

You can now use the Python package as you would any other package.
```python
from maturin_pyo3 import fib, estimate_pi

fib(10)
estimate_pi(1_000_000)
```

## Bonus:

Try building an optimized `release` version of the library and compare speed.
```bash
maturin build --release
```

You can also experiment with adding more advanced data structures to the module and using them in Python.
```rust

#[pyclass]
struct MyStruct {
    field1: i32,
    field2: String,
    field3: Vec<f64>,
}

#[pymethods]
impl MyStruct {
    #[new]
    fn new(field1: i32, field2: &str, field3: Vec<f64>) -> Self {
        MyStruct {
            field1,
            field2: field2.to_string(),
            field3,
        }
    }

    fn get_fields(&self) -> (i32, &str, Vec<f64>) {
        (self.field1, &self.field2, self.field3.clone())
    }

    fn calculate_sum(&self) -> f64 {
        self.field3.iter().sum()
    }
}

#[pymodule]
fn maturin_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    /// ...
    /// Previous code
    /// ...
    m.add_class::<MyStruct>()?;
    Ok(())
}
```

Now you can use the `MyStruct` class in Python.
```python
from maturin_pyo3 import MyStruct

# Create an instance of MyStruct
my_instance = MyStruct(42, "Hello", [1.0, 2.5, 3.7])

# Access the fields using the get_fields method
fields = my_instance.get_fields()
print(f"Fields: {fields}")

# Calculate the sum using the calculate_sum method
sum_result = my_instance.calculate_sum()
print(f"Sum of field3: {sum_result}")
```
