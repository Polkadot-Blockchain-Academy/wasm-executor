#![allow(unused_variables)]
/// Takes in a simple primitive signed 32-bit integer
/// and adds one to it, returning the result.
///
/// It is ready for wasm execution thanks to the `#[no_mangle]` attribute.
/// You may find that you also need `extern "c"`.
#[no_mangle]
pub extern "C" fn add_one(n: i32) -> i32 {
	todo!("After you implement this first one, compile and run it on both targets. See readme for instructions.")
}

/// Prints the number 42 to the terminal.
/// It does not do any math, the number is always 42.
///
/// Reminder, you may need to add some annotations to this function's declaration
/// to make it work with wasm.
pub fn print_forty_two() {
	todo!()
}

/// Calculates the division between two integers number
pub fn div(a: i32, b: i32) -> i32 {
	todo!()
}

/// Fetches the current system time and determines whether it is after the year 2000
/// Returns true iff the current time is after January 1st 2000
/// you may find https://doc.rust-lang.org/std/time/struct.SystemTime.html useful
pub fn wen_millennium() -> bool {
	todo!()
}

/// Calculates the sum of two floating point numbers
pub fn sum_floats(a: f32, b: f32) -> f32 {
	todo!()
}

/// Takes in a simple primitive signed 32-bit integer
/// and adds one to it, and writes the result to a file.
pub fn write_to_file() {
	// This can be made to work
	// See `wasmtime run --help` for a clue
	todo!()
}
