extern crate serialize;

use std::os;
use std::io::stdio;
use serialize::base64::{ToBase64,Config,Standard};

fn main() {
    let exit_status = run(os::args());
    os::set_exit_status(exit_status);
}

fn print(message: String) {
    let result = stdio::stdout().write(message.as_bytes());
    match result {
        Ok(_) => (),
        Err(failure) => fail!(format!("Failed to write to stdout: {}", failure))
    }
}

fn run(args: Vec<String>) -> int {
    let input = stdio::stdin().read_to_string().ok().expect("Failed to read from stdin");
    let output = input.as_bytes().to_base64(Config { char_set: Standard, pad: true, line_length: None});
    print(output);
    return 0;
}
