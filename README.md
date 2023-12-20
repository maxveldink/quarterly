# Quarterly

A simple Rust crate containing types for handling calendar quarters.

## Usage

```rust
use quarterly::*;

/// Basics
let quarter = Quarter::new(QuarterNumber::Q4, 2023)
quarter.next_quarter() /// => Quarter::new(QuarterNumber::Q1, 2024)

/// String parsing
let quarter: Quarter = "Q2 2023".parse::().unwrap();
let quarter: Quarter = "q3 1994".parse::().unwrap();
```

## Development

Clone the repo, and then run the standard cargo commands to verify the project.

`cargo clippy; cargo fmt; cargo test`
