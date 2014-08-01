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

fn run(args: Vec<String>) -> int {
    let program = &args[0];

    let options = [
        optflag("V", "version", "Print the version number and exit")
    ];

    let matches = getopts(args.tail(), options).unwrap();

    if matches.opt_present("V") {
        println!("hostname 1.0.0");
        return 0;
    }

    if matches.free.len() == 1 {
        stdio::stderr().write("hostname: you must be root to change the host name\n".as_bytes());
        return 1;
    }

    let result = get_hostname();
    println!("{}", result.unwrap());
    match result {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

fn get_hostname() -> Result<String, String> {
    let mut name = String::with_capacity(HOSTNAME_MAX_LENGTH).to_c_str();

    let result = unsafe { gethostname(name.as_mut_ptr(), HOSTNAME_MAX_LENGTH as size_t) };
    if result == 0 {
        Ok(name.to_string())
    } else {
        Err(String::from_str("Failed to get hostname"))
    }
}
