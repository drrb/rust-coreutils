extern crate getopts;

use getopts::{optflag, getopts};
use std::io::stdio;
use std::os;

fn main() {
    let exit_status = run(os::args());
    os::set_exit_status(exit_status);
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

fn print(message: String) {
    let result = stdio::stdout().write(message.as_bytes());
    match result {
        Ok(_) => (),
        Err(failure) => fail!(format!("Failed to write to stdout: {}", failure))
    }
}
