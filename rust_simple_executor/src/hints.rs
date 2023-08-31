/*
 * TODO 2:
 *
 * A Global compilation environment in wasmtime is represented
 * by the struct `Engine`, for this version of the executor you don't
 * need something particular so you can go with the Default one
 *
 * TODO 3:
 *
 * A wasm code is composed of a single module,
 * and the structure that represents a compiled wasm code in
 * wasmtime is a `Module`
 *
 * TODO 5:
 * You can use the struct 'Instance' to instantiate the wasm code
 * and then from here you can actually acquire a function from
 *
 * TODO 6:
 * You can use the instantiated code to extract the entry point,
 * if you can't figure out how then look into `get_type_func`
 *
 * TODO 7:
 * You can't just call the function as you would with
 * a normal Rust Function, you need to pass through the type
 * Func and the method 'call'
 * */
