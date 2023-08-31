#![allow(unused_variables)]
use wasmtime::*;

/// Shared State between the Executor and all the Wasm Blobs
#[derive(Clone)]
pub struct SharedState {
	pub val: u32,
}

/// Helper unction to load a wasm file
/// If the name provided does not end with `.wasm` it is appended.
fn load_wasm_code(name: &str) -> Result<Vec<u8>, &'static str> {
	use std::io::Read;

	// If the file does not end with a `.wasm` extension, add it.
	let path = match name.ends_with(".wasm") {
		true => format!("wasm_codes/{}", name),
		false => format!("wasm_codes/{}.wasm", name),
	};

	let mut f = std::fs::File::open(&path).map_err(|_| "file not found")?;

	let mut code = Vec::new();
	f.read_to_end(&mut code).expect("impossible read wasm_code");
	Ok(code)
}

/// This function implements the Executor,
/// you will use wasmtime as Embedder of the wasm code and all the needed documentation
/// is here: https://docs.rs/wasmtime/latest/wasmtime/
///
/// It accepts the name of the wasm blob that needs to be executed, and
/// the current SharedState
///
/// The Result is the new SharedState or an Error containing a String describing the Error
pub fn executor(name: &str, shared_state: SharedState) -> Result<SharedState, String> {
	let wasm_code = load_wasm_code(name).map_err(|err| err.to_string())?;

	let engine = Engine::default();

	let module = Module::new(&engine, wasm_code).map_err(|err| err.to_string())?;

	// TODO 1:
	// Create the Store, which will contain all the information related to
	// WebAssembly objects such as functions, instances, memories, etc
	//
	// The Store allows inserting also arbitrary data (SharedState)
	todo!();

	// TODO 2:
	// Crete the Host Functions
	todo!();

	// TODO 3:
	// Instantiate the wasm code
	todo!();

	// TODO 4:
	// Extract the entry point "start" from the just instantiated link
	// end execute it!!
	todo!();

	// TODO 5:
	// Just return the new SharedState
	todo!()
}
