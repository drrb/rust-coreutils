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

fn println(message: String) {
    println!("{}", message);
}

fn err_println(message: String) {
    let result = stdio::stderr().write(message.as_bytes());
    match result {
        Ok(_) => (),
        Err(failure) => fail!(format!("Failed to write to stderr: {}", failure))
    }
}

fn usage_message(program: &String, options: &[OptGroup]) -> String {
    let instructions = format!("Usage: {} [options] [HOSTNAME]", program);
    usage(instructions.as_slice(), options)
}

fn run(args: Vec<String>) -> int {
    let program = &args[0];

    let parameters = [
        optflag("V", "version", "Print the version number and exit"),
        optflag("h", "help", "Print this help message")
    ];

    let options = match getopts(args.tail(), parameters) {
        Ok(options) => options,
        Err(failure) => fail!(failure.to_string())
    };

    if options.opt_present("h") {
        println(usage_message(program, parameters));
        return 0;
    }

    if options.opt_present("V") {
        println!("hostname 1.0.0");
        return 0;
    }

    if options.free.len() == 1 {
        err_println("hostname: you must be root to change the host name\n".to_string());
        return 1;
    }

    match get_hostname() {
        Ok(hostname) => println(hostname),
        Err(error) => err_println(error)
    }
    return 0;
}

fn get_hostname() -> Result<String, String> {
    let mut name = String::with_capacity(HOSTNAME_MAX_LENGTH).to_c_str();

    let result = unsafe { gethostname(name.as_mut_ptr(), HOSTNAME_MAX_LENGTH as size_t) };
    if result == 0 {
        Ok(name.to_string())
    } else {
        Err("Failed to get hostname".to_string())
    }
}
