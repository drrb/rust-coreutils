extern crate getopts;

use std::io::stdio;
use std::os;
use getopts::{optflag, getopts};

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

    let parameters = [
        optflag("n", "", "Do not print the trailing newline character"),
    ];

    let options = match getopts(args.tail(), parameters) {
        Ok(options) => options,
        Err(failure) => fail!(failure.to_string())
    };

    let message = options.free.connect(" ");

    print(message);
    if ! options.opt_present("n") {
        print("\n".to_string());
    }
    return 0;
}
