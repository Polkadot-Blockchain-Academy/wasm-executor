/*
 * TODO 1:
 * The object Store will be usable in each host function,
 * the SharedState that you need to be able to modify from the
 * HostFunctions needs to be inserted here
 *
 * TODO 2:
 * You will need to create before something called Linker
 * that on instantiation time will link the provided host
 * functions to the wasm code
 *
 * The default module name used in the wasm code for the
 * host functions is "env"
 *
 * In the closure used to implement the HostFunctions you will be able
 * to access the Storage, how? using the Caller object
 * (mandatory first parameter of the closure) you can use the `.data()`
 * or `.data_mut()` to retrieve the Store object
 *
 * TODO 3:
 * You can use the the just created Linker,
 * which will automatically resolve the imports
 *
 * TODO 4:
 * You can use the instantiated code to extract the entry point,
 * if you can't figure out how then look into `get_type_func`
 *
 * */
