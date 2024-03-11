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
