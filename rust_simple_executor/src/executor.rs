#![allow(unused_variables)]
use wasmtime::*;

/// Helper unction to load a wasm file
/// If the name provided does not end with `.wasm` it is appended.
fn load_wasm_code(name: &str) -> Result<Vec<u8>, &'static str> {
	use std::io::Read;

	// If the file does not end with a `.wasm` extension, add it.
	let path = match name.ends_with(".wasm") {
		true => format!("{}", name),
		false => format!("{}.wasm", name),
	};

	let mut f = std::fs::File::open(&path).map_err(|_| "file not found")?;

	let mut code = Vec::new();
	f.read_to_end(&mut code).expect("impossible read wasm_code");
	Ok(code)
}

// This function implements the Executor,
// you will use wasmtime as Embedder of the wasm code and all the needed documentation
// is here: https://docs.rs/wasmtime/latest/wasmtime/
pub fn executor() -> Result<(), String> {
	// TODO 1:
	// Firstly, the wasm code is needed,
	// It is represented in a binary format so we will just load it
	// from the file (you can the use the function just right above).
	let wasm_code = todo!();

	// TODO 2:
	// You now need to create the Global compilation environment for
	// WebAssembly. We will use the default one as they do in the docs.
	let engine = todo!();

	// TODO 3:
	// Compile the Wasm code, the output will represent
	// the in-memory JIT code which is ready
	// to be executed after being instantiated
	let module = todo!();

	// TODO 4:
	// Create the Store, which will contain all the information related to
	// WebAssembly objects such as functions, instances, memories, etc
	//
	// The Store also allows inserting arbitrary data, but we will not use
	// them in this executor
	let mut store = todo!();

	// TODO 5:
	// Instantiate the wasm code
	let instance = todo!();

	// TODO 6:
	// Extract the entry point "div" from the just-instantiated code
	let div = todo!();

	// TODO 7:
	// Execute the wasm function!
	let (x, y) = (10, 2);
	let result: i32 = todo!();

	println!("{x} / {y} = {result}");

	Ok(())
}

// After you have made this executor work, you should explore the python executor
// as described in the readme.

// Then you can try to extend either of the executors to call other functions
// in your wasm blob.
