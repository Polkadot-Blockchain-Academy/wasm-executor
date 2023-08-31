#! /usr/bn/env python3

from wasmtime import Store, Module, Instance

# Wasmtime APIs are almost identical in Rust or Python

# Create the Store, which will contain all the information related to
# WebAssembly objects such as functions, instances, memories, etc
store = Store()

# Compile the Wasm code, the output will represent the in-memory JIT code which is ready
# to be executed after being instantiated
module = Module.from_file(store.engine, '../wasm_code/target/wasm32-unknown-unknown/release/wasm_code.wasm')

# Instantiate the wasm code
instance = Instance(store, module, [])

# Python is not so strongly typed, so we can provide the
# entry point as a string
start = instance.exports(store)["div"]

# Execute the wasm function!
result = start(store, 10, 2)

print("10 / 2 = " + str(result))

# Now you can try to extend either of the executors to call other functions
#  in your wasm blob.
