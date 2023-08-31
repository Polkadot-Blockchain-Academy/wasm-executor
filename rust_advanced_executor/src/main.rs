mod executor;

use executor::{executor, SharedState};

use std::io::Read;

macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        }
    };
}

fn main() {
    // Init Value of the SharedState
    let mut state: SharedState = SharedState { val: vec![1, 2, 3] };
    let mut prev_executed_wasm_blob = String::new();

    loop {
        println!("");
        println!("Options:");
        println!("1 -> Exectue wasm code");
        println!(
            "2 -> Execute previous wasm code ({})",
            prev_executed_wasm_blob
        );
        println!("3 -> List wasm codes");
        println!("Current SharedState value: {:?}", state.val);
        println!("");

        match skip_fail!(get_input()) {
            1 => {
                println!("Insert wasm code name: ");
                let wasm_code_name: String = skip_fail!(get_input());

                state = skip_fail!(executor(wasm_code_name.as_ref(), state.clone()));
                prev_executed_wasm_blob = wasm_code_name;
                println!("New SharedState value: {:?}", state.val);
            }
            2 => {
                if !prev_executed_wasm_blob.is_empty() {
                    state = skip_fail!(executor(prev_executed_wasm_blob.as_ref(), state.clone()));
                    println!("New SharedState value: {:?}", state.val);
                } else {
                    println!("No previous wasm blob");
                }
            }
            3 => {
                let paths =
                    std::fs::read_dir("wasm_codes/").expect("wasm_codes directory does not exist");

                println!("Wasm codes:");
                for path in paths {
                    println!(
                        "{}",
                        path.unwrap()
                            .file_name()
                            .to_str()
                            .expect("Impossible list wasm codes names")
                    );
                }
            }
            _ => println!("Not valid Option"),
        }

        println!("");
        println!("Press Enter to continue");
        std::io::stdin().read(&mut [0]).unwrap();
    }
}

fn get_input<T: std::str::FromStr>() -> Result<T, &'static str> {
    let mut input_line = String::new();

    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

    println!("");

    input_line
        .trim()
        .parse::<T>()
        .map_err(|_| "Impossible Parse Input")
}
