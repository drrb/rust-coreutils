use std::io::{stdio,IoError,EndOfFile};
use std::os;

fn main() {
    let exit_status = run(os::args());
    os::set_exit_status(exit_status);
}

fn run(_: Vec<String>) -> int {
    let mut buffer = [0, ..1024];
    loop {
        let bytes = match stdio::stdin_raw().read(buffer) {
            Ok(bytes_read) => buffer.slice_to(bytes_read),
            Err(IoError{ kind: EndOfFile, .. }) => return 0,
            _ => fail!("Error reading input")
        };
        stdio::stdout().write(bytes).unwrap();
    }
}
