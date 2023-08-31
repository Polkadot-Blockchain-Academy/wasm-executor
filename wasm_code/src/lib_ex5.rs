#![allow(unused_variables)]
#![feature(vec_into_raw_parts)]

use std::vec::Vec;

// The host functions let us work with a Vector SharedState,
// set_vec and get_vec will be not directly used in the code
// because they require to implement a FFI (Function Foreign Interface).
// What's FFI? It is just a way to make possible the sharing of more complex
// data then just integers with the Embedder.
//
// `set_vec` only require the pointer of the vector we want to pass
// to the executor and it size, the embedder will then read the vec from
// the memory and copy it in the SharedState
//
// `get_vec` instead provide a pointer and a size of free space
// where the embedder can write a Vector in it, the function returns
// the size of the written memory.
//
// Vectors will be allocated in the heap and wasm will use the LinearMemory
// as heap. Each pointer to Vector you will pass to the
// embedder then is just an index in LinearMemory.
extern "C" {
    pub fn set_vec(ptr: u32, size: u32);
    pub fn get_vec(ptr: u32, max_size: u32) -> u32;
}

// Working directly with the host functions is complex,
// here you can find an abstraction around the host functions
// that take care of all the conversions between Vectors
// and what is required by the embedder.
pub fn set_vec_hf(vec: Vec<u8>) {
    // into_raw_parts does perfectly what is requested by
    // the host function, it return a pointer to the data
    // and the size
    let (ptr, size, _) = vec.into_raw_parts();
    unsafe { set_vec(ptr as u32, size as u32) }
}

pub fn get_vec_hf() -> Vec<u8> {
    // Getting a vec from the embedder is a little bit trickier,
    // the concept is to provide so free memory to the embedder and
    // let it write the vec in the provided memory
    //
    // First we allocate a vec
    const MAX_SIZE: usize = 100;
    let mut vec = vec![0; MAX_SIZE];

    // The host function is called with the just allocated array,
    // it will return the size of the written Vec
    let size = unsafe { get_vec(vec[..].as_ptr() as u32, MAX_SIZE as u32) };

    // Knowing the size of the Vec we can truncate the original
    vec.truncate(size as usize);
    vec
}

#[no_mangle]
fn start() {
    // Implementation Test: Multiply each element by 3
    let mut vec = get_vec_hf();

    for v in vec.iter_mut() {
        *v *= 3;
    }

    set_vec_hf(vec);
}
