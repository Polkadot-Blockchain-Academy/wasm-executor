# Web Assembly Activity

In this activity, you will practice compiling Rust code to a few different wasm targets.
You will also learn to embed a web assembly executor in your own binary application.
We will focus on Rust, but also briefly look at executing wasm from Python to demonstrate the platform agnostic nature of Wasm.

## 0. Install Prerequisites

For this activity, you will need some wasm tooling installed.

1. Wasmtime CLI - https://github.com/bytecodealliance/wasmtime/blob/main/docs/cli-install.md
2. wasm32-unknown-unknown target - `rustup target add wasm32-unknown-unknown`
3. wasm32-wasi target - `rustup target add wasm32-wasi`

## 1. Write a Wasm Library

To get started, you will create a simple Rust library, and compile it to Wasm bytecode which can then be run in many different environments.
We will explore both the `wasm32-unknown-unknown` target and the `wasm32-wasi` target.

To get started, implement the first function `wasm_code/src/lib_ex_1.rs`.

### Compilation and Execution

Begin by compiling your library to both targets.

```sh
cd wasm_code
cargo build --release --target wasm32-unknown-unknown
cargo build --release --target wasm32-wasi
```

Then try running your first function on both wasm targets. See `wasmtime --help` for much more information

```sh
# General form
# wasmtime <wasm_code_path> --invoke <name_function> <parameters>
wasmtime target/wasm32-unknown-unknown/release/wasm_code.wasm --invoke add_one 7

wasmtime target/wasm32-wasi/release/wasm_code.wasm --invoke add_one 7
```

Have you noticed any differences between the two targets so far?
If not, that's fine. Keep paying attention and there will be a noticeable difference eventually.

### More Functions

Work through the rest of the library, completing each function.
For each function, compile and run on both targets.
Keep track of what works and what doesn't on each target.

### Hints

If you can't figure out the correct signatures for the function you're developing in wasm then the problems could be:

- Usually, compilers change the name of things, like functions or variables name but you don't want that, because you're searching for specific names in the executor, you should then look for `#[no_mangle]`
- Rust uses it's own ABI (Application Binary Interface) and this could not be compliant with the ABI used by executors, a good thing could be use `extern "C"` to use the Standard C code ABI

If you're not compiling the wasm code properly then some problems could be:

- If you don't know how to specify the compilation target using cargo then you should look for the `--target`
- If you're compiling without `--release` then is possible that the wasm code will then expect some Host Functions related to debug things

### FAQs

- Why `crate-type = ["cdylib"]` in `wasm_code/Cargo.toml`?
  https://users.rust-lang.org/t/why-do-i-need-to-set-the-crate-type-to-cdylib-to-build-a-wasm-binary/93247/6
- Why not all std library works in wasm32-unknown-unknown?
  https://www.reddit.com/r/rust/comments/kyae22/comment/gjissev/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

## 2. Implement a Simple Wasm Executor in Rust

We just used a program called the Wasmtime CLI to execute our Wasm library.
This Cli is an example of a program that embeds the wasmtime executor library.
As we will learn next week, the Substrate client also embeds the wasmtime executor.

In this section we will write a binary program in Rust that also embeds the wasmtime executor library.
The core steps in this process are:

- Create the environment
- Instantiate and execute the wasm code

To get started, look at `rust_simple_executor/src/executor.rs`.

### Compilation and Execution

```sh
cd wasm_code
cargo run
```

## 3. A Simple Python Executor

So far we have managed to call a function written in Rust from a program written in Rust using web assembly as an intermediate.
By using wasm, we are able to re-use our library anywhere that Wasm can be run.
We'll demonstrate this by writing the same executor again in Python.

Explore the python code at `python_simple_executor/executor.py`, and see how the API compares to the Rust one.

## Running the Executor

First, you need to follow those instructions to create an environment and install the required dependency, wasmtime. You can find the documentation here: https://bytecodealliance.github.io/wasmtime-py/

```sh
cd python_executor

python3 -m venv env
source env/bin/activate
pip install -r requirements
```

To test the executor now you can simply:

```sh
python3 executor.py
```

The default implementation is connected to your wasm library and will call the `div(i32, i32) -> i32` entrypoint.

To stop the Python environment:

```sh
deactivate
```

## 4. Implement an Executor with host functions

Now it's time to implement some custom Host Functions.
We used the standard system interface provided by wasi earlier.
Now we will define and implement our own fixed set of host functions.
These host functions will allow access to a piece of shared state between the wasm code and the executor.

The only constraints required to the wasm code that our executor will be able to execute are:

- Each wasm code must provide a specific entry point

  ```rust
  fn start();
  ```

- Each wasm code will have access to a shared state with the executor, more precisely an `u32` and the access will be managed by two host functions:

  ```rust
  fn get() -> u32;
  fn set(u32);
  ```

### Shared State

In the first executor you just created the environment, instantiate and then execute the wasm code. In this executor, given the wasm code name and the current `SharedState` the function will:

- Create the environment
- Create and bind the host functions
- Instantiate and execute the wasm code on the `SharedState`

To get started, follow the instructions in `wasm_code/src/lib_ex_3.rs` **and** `rust_executor/src/executor.rs``

### Compilation and Execution

You must first make this change in `wasm_code/Cargo.toml`:

```diff
path = "src/lib_ex1_solution.rs"
path = "src/lib_ex4_solution.rs"
```

Remember, that you need to specify the compilation target and the release build mode.

```sh
cd wasm_code
cargo build --release --target wasm32-unknown-unknown
```

The output should be copied into the executor which can then be run directly

```sh
cp wasm_code/target/wasm32-unknown-unknown/release/wasm_code.wasm rust_executor/wasm_codes/custom.wasm
cargo run
```

## 5. Advanced Executor

You can now change what's contained in the `SharedState` and try maybe to use a `Vec<u8>` instead of a simple u32.

Examples of new possible host functions:

```rust
    fn get_vec() -> Vec<u32>;
    fn set_vec(Vec<u32>);
```

If you start implementing it you will discover how complex things start being. In the folder `rust_advanced_executor` you can find an implementation of an executor that uses a `Vec<u8>` as `SharedState`. You will notice that the defined HostFunctions are different then the ones just described, in the code you will find a lot of comments and you will understand why they are different.

In the `wasm_code/src/lib_ex5.rs` you will find an implementation of a rust code able to correctly implement the required HostFunctions, you just need to change the path again in `wasm_code/Cargo.toml` to build the correct wasm code. If you want you can try to implement more complex logic and test it interactively as the `rust_executor` (remember to put the wasm code in `rust_advanced_executor/wasm_codes/`)

## License

Licensed under the terms of the [GPL-3](./LICENSE.md) or later.
