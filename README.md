# Rust and ML

Simple introduction into Rust.
From Hello World to ML.

## Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Overview of Examples

### 1. `rust_intro`
- Introduces `cargo` and `rustc` commands
- Guides through building a simple program


### 2. `maturin-pyo3`

- Introduces building Python modules with Rust
- Illustrates performance benefits of Rust over Python

### 3. `opencv-rs`

- More advanced example of using Rust binding for OpenCV C++ API
- Shows examples of imutable (`&`) and mutable (`&mut`) borrowing in Rust

### 4. `control-net-rs`
 
 - An example of ControlNet in Rust. This example makes use of two ML focused crates:
   - `tch-rs`: Rust bindings for the PyTorch C++ API
   - `diffusers-rs`: A Rust implementation of HuggingFace's Diffusers Python library. Written with `tch`.