extern crate getopts;
extern crate serialize;

use std::os;
use std::io::stdio;
use serialize::base64::{FromBase64,ToBase64,Config,Standard};
use getopts::{optflag, getopts};

fn main() {
    let exit_status = run(os::args());
    os::set_exit_status(exit_status);
}

fn print(bytes: &[u8]) {
    let result = stdio::stdout().write(bytes);
    match result {
        Ok(_) => (),
        Err(failure) => fail!(format!("Failed to write to stdout: {}", failure))
    }
}

fn run(args: Vec<String>) -> int {
    let parameters = [
        optflag("D", "decode", "decodes input"),
    ];

    let options = match getopts(args.tail(), parameters) {
        Ok(options) => options,
        Err(failure) => fail!(failure.to_string())
    };

    let stdin = stdio::stdin().read_to_string().ok().expect("Failed to read from stdin");
    let input = stdin.as_bytes();
    if options.opt_present("D") {
        let output = input.from_base64().ok().expect("Failed to decode");
        print(output.as_slice());
    } else {
        let output = input.to_base64(Config { char_set: Standard, pad: true, line_length: None});
        print(output.as_bytes());
    }
    return 0;
}
