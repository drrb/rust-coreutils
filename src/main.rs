#![crate_name = "hostname"]

extern crate libc;
extern crate rlibc;

use libc::{c_char, c_int, size_t};

static HOSTNAME_MAX_LENGTH: uint = 256;

#[cfg(not(test))]
extern {
    fn gethostname(name: *mut c_char, namelen: size_t) -> c_int;
}

#[cfg(not(test))]
fn main() {
    use std::os;

    let result = get_hostname(os_get_hostname);
    println!("{}", result.unwrap());
    let exit_status = match result {
        Ok(_) => 0,
        Err(_) => 1,
    };
    os::set_exit_status(exit_status);
}

fn get_hostname(os_get_hostname: unsafe fn(*mut c_char, size_t) -> c_int) -> Result<String, String> {
    let mut name = String::with_capacity(HOSTNAME_MAX_LENGTH).to_c_str();

    let result = unsafe { os_get_hostname(name.as_mut_ptr(), HOSTNAME_MAX_LENGTH as size_t) };
    if result == 0 {
        Ok(name.to_string())
    } else {
        Err(String::from_str("Failed to get hostname"))
    }
}

#[cfg(not(test))]
unsafe fn os_get_hostname(name: *mut c_char, namelen: size_t) -> c_int {
    gethostname(name, namelen)
}

#[cfg(test)]
mod hostname {
    extern crate libc;
    extern crate rlibc;

    use libc::{c_char, c_int, size_t};
    use super::get_hostname;

    unsafe fn os_get_hostname_success(name: *mut c_char, _namelen: size_t) -> c_int {
        let hostname = "host.example.com".to_c_str();
        rlibc::memcpy(name as *mut u8, hostname.as_bytes().as_ptr(), hostname.as_bytes().len());
        return 0;
    }

    unsafe fn os_get_hostname_failure(_name: *mut c_char, _namelen: size_t) -> c_int {
        return 1;
    }

    #[test]
    fn looks_up_the_hostname() {
        let result = get_hostname(os_get_hostname_success);
        assert!(result.is_ok());
        assert!(result.unwrap() == "host.example.com".to_string());
    }

    #[test]
    fn returns_an_error_message_on_failure() {
        let result = get_hostname(os_get_hostname_failure);
        assert!(result.is_err());
        assert!(result.err().unwrap() == "Failed to get hostname".to_string());
    }
}
