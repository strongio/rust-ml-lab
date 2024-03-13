# Introduction to Rust and Cargo

## Initialize with Cargo

Start by initializing the project within the `rust_intro` directory with:
```bash
cargo init
```

You should have the following directory structure:
```bash
.
├── Cargo.toml
├── README.md
└── src
    └── main.rs
```
The `main.rs` contains a simple "Hello, world!" program.

NOTE: You can also start a new project with a specific name with: `cargo new <project name>`.

## Build and Run

You can build and run the code with the following:
```bash
cargo run
```
By default this builds a `debug` version of the code. You can also build a `release` version with:
```bash
cargo build --release
```

## Computing the nth Fibonacci Number

Add a depdenency to the `Cargo.toml` file:
```bash
cargo add rand
```
Check that the `Cargo.toml` file has the following:
```toml
[package]
name = "rust_intro"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
```

Add a new function to compute the nth Fibonacci number in `src/main.rs`:
```rust
fn fib(n: isize) -> u128 {
    let mut sum: u128 = 0;
    let mut last: u128 = 0;
    let mut curr: u128 = 1;
    for _ in 0..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}
```

On the `main` function generate a random number and compute the nth Fibonacci number by calling the `fib()` function.

```rust
use rand::{thread_rng, Rng};

// fib function here

fn main() {
    // Generate a random number between 1 and 100
    let mut rng = thread_rng();
    let n = rng.gen_range(1..100);

    // Call the fib function
    let sum: u128 = fib(n);

    println!("The {}th Fibonacci number is: {}", n, sum);
}
```

Build and run
```bash
cargo run --release
```