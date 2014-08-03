#![crate_name = "hostname"]

extern crate getopts;
extern crate libc;

use std::io::stdio;
use std::os;
use getopts::{optflag, getopts, usage, OptGroup};
use libc::{c_char, c_int, size_t};

static HOSTNAME_MAX_LENGTH: uint = 256;

extern {
    fn gethostname(name: *mut c_char, namelen: size_t) -> c_int;
}

fn main() {
    let exit_status = run(os::args());
    os::set_exit_status(exit_status);
}

fn print_usage(program: &String, options: &[OptGroup]) {
    let instructions = format!("Usage: {} [options] [HOSTNAME]", program);
    println!("{}", usage(instructions.as_slice(), options));
}

fn err_println(message: &str) {
    let result = stdio::stderr().write(message.as_bytes());
    match result {
        Ok(_) => (),
        Err(failure) => fail!(format!("Failed to write to stderr: {}", failure))
    }
}

fn run(args: Vec<String>) -> int {
    let program = &args[0];

    let parameters = [
        optflag("V", "version", "Print the version number and exit"),
        optflag("h", "help", "Print this help message")
    ];

    let options = match getopts(args.tail(), parameters) {
        Ok(options) => { options },
        Err(failure) => { fail!(failure.to_string()) }
    };

    if options.opt_present("h") {
        print_usage(program, parameters);
        return 0;
    }

    if options.opt_present("V") {
        println!("hostname 1.0.0");
        return 0;
    }

    if options.free.len() == 1 {
        err_println("hostname: you must be root to change the host name\n");
        return 1;
    }

    let hostname = get_hostname();
    println!("{}", hostname);
    return 0;
}

fn get_hostname() -> String {
    let mut name = String::with_capacity(HOSTNAME_MAX_LENGTH).to_c_str();

    let result = unsafe { gethostname(name.as_mut_ptr(), HOSTNAME_MAX_LENGTH as size_t) };
    if result == 0 {
        name.to_string()
    } else {
        fail!("Failed to get hostname")
    }
}
