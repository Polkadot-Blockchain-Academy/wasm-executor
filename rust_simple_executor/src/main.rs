#![allow(unused_variables)]
//! This binary application embeds the wasmtime library to execute wasm bytecode.
//!
//! The code in this file is given as useful utilities.
//! Your job is to implement the functions in executor.rs

mod executor;

use executor::executor;

fn main() {
	executor().unwrap();
}
