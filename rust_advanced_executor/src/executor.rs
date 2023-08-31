use wasmtime::*;

// Shared State between the Executor and all the Wams Blobs
#[derive(Clone)]
pub struct SharedState {
    pub val: Vec<u8>,
}

fn load_wasm_code(name: &str) -> Result<Vec<u8>, &'static str> {
    use std::io::Read;
    let path = match name.ends_with(".wasm") {
        true => format!("wasm_codes/{}", name),
        false => format!("wasm_codes/{}.wasm", name),
    };

    let mut f = std::fs::File::open(&path).map_err(|_| "file not found")?;

    let mut code = Vec::new();
    f.read_to_end(&mut code).expect("impossible read wasm_code");
    Ok(code)
}

// This function implements the Advanced Executor
pub fn executor(name: &str, shared_state: SharedState) -> Result<SharedState, String> {
    // Firstly the wasm code is neeed
    let wasm_code = load_wasm_code(name).map_err(|err| err.to_string())?;

    // Global compilation environment for WebAssembly
    let engine = Engine::default();

    // Compile the Wasm code into a Module,
    // the in-memory JIT code which is ready
    // to be execute after being instantiated
    let module = Module::new(&engine, wasm_code).map_err(|err| err.to_string())?;

    // The Store will contain all the information related to
    // WebAssembly objects such as functions, instances, memories, etc
    let mut store = Store::<SharedState>::new(&engine, shared_state);

    // Crete the Host Functions
    //
    // In this case those will be more complex than the first Rust_Executor,
    // in this case we will move Vectors between the executor and the wasm code
    // This is possible thanks to the Linear Memory
    let mut linker = Linker::new(&engine);

    // The `set_vec` host function will get a vector from the wasm code and insert it
    // in the SharedState. As you can the the function does no accept a normal vec but only a
    // pointer (casted to u32) and the size of the vec, how those two arguments let us
    // coping a vec from wasm to the executor is explained in the `read_vec` function
    linker
        .func_wrap(
            "env",
            "set_vec",
            |mut caller: Caller<'_, SharedState>, ptr: u32, size: u32| -> Result<()> {
                // Read the vec from Wasm Linear Memory
                let vec = read_vec(&mut caller, ptr, size)?;
                // Update the SharedState with the new Vec
                caller.data_mut().val = vec;
                Ok(())
            },
        )
        .map_err(|err| err.to_string())?;

    // `get_vec` host function instead write the SharedState vec to the wasm code,
    // even if the signature is almost the same of `set_vec`. How the Vector is written in the
    // Wasm Linear Memory is explained in the `write_vec` function.
    linker
        .func_wrap(
            "env",
            "get_vec",
            |mut caller: Caller<'_, SharedState>, ptr: u32, size: u32| -> Result<u32> {
                // Write the SharedState Vec to the Wasm Linear Memory
                let size_written_vec = write_vec(&mut caller, ptr, size)?;
                Ok(size_written_vec)
            },
        )
        .map_err(|err| err.to_string())?;

    // Instantiate the wasm code
    let instance = linker
        .instantiate(&mut store, &module)
        .map_err(|err| err.to_string())?;

    // Extract the entry point "start" end execute it
    let start = instance
        .get_typed_func::<(), ()>(&mut store, "start")
        .map_err(|err| err.to_string())?;

    start.call(&mut store, ()).map_err(|err| err.to_string())?;

    // Just return the new SharedState
    Ok(SharedState {
        val: store.data().val.clone(),
    })
}

/// The function has three input arguments:
/// + the caller, that give us access to lot of wasm's things,
/// such as the linear memory and other exports
/// + the pointer to the beginning of the Vec in the wasm's Linear Memory
/// + the size of the Vec
pub fn read_vec(
    caller: &mut Caller<'_, SharedState>,
    ptr: u32,
    size: u32,
) -> Result<Vec<u8>, Trap> {
    // First we need to extract the memory, which is defined
    // as and Export in Wasm
    let mem = match caller.get_export("memory") {
        Some(wasmtime::Extern::Memory(mem)) => mem,
        _ => return Err(Trap::UnreachableCodeReached),
    };

    // From the memory object we can extract the Wasm Linear Memory
    // as a Slice and then
    // Use the `ptr` and `size` values to get a sub-slice of the wasm-memory
    let wasm_slice: Option<&[u8]> = mem
        .data(&caller)
        .get(ptr as u32 as usize..)
        .and_then(|arr| arr.get(..size as u32 as usize));

    // If the extraction of the slice from  wasm successful
    // then translate it to a vec
    match wasm_slice {
        Some(w) => Ok(w.to_vec()),
        None => Err(Trap::UnreachableCodeReached),
    }
}

// The input arguments are the same as `read_vec` but they are logically different,
// the `ptr` and `size` refers to an area in the Wasm Linear Memory but not for read it,
// instead it provided to let write a vector in it, the size.
//
// The return value is the size of the just written vec
pub fn write_vec(
    caller: &mut Caller<'_, SharedState>,
    ptr: u32,
    max_size: u32,
) -> Result<u32, Trap> {
    // let's get access to wasm Linear Memory
    let mem = match caller.get_export("memory") {
        Some(wasmtime::Extern::Memory(mem)) => mem,
        _ => return Err(Trap::UnreachableCodeReached),
    };

    // Clone the vector from the ShareState
    let vec = caller.data().val.clone();

    // Make sure that the max size of the Vec provided by the
    // wasm code is enough to contain the new Vec
    if (max_size as usize) < vec.len() {
        return Err(Trap::UnreachableCodeReached);
    }

    // Get the slice we want to use to store the new Vec from the LinearMemory
    let wasm_buffer = &mut mem.data_mut(caller)[ptr as usize..ptr as usize + vec.len()];

    // Save the Vec in the just extracted slice
    wasm_buffer.copy_from_slice(&vec[..]);

    Ok(vec.len() as u32)
}
