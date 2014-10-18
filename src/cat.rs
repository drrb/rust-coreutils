use std::io::{stdio,File,EndOfFile,IoError,Reader};
use std::os;

fn main() {
    let exit_status = run(os::args());
    os::set_exit_status(exit_status);
}

fn run(args: Vec<String>) -> int {
    let mut input: Box<Reader> = if args.len() == 1 {
        box stdio::stdin_raw()
    } else {
        let path = Path::new(args[1].as_bytes());
        box File::open(&path).unwrap()
    };

    let mut buffer = [0, ..1024];
    loop {
        let bytes = match input.read(buffer) {
            Ok(bytes_read) => buffer.slice_to(bytes_read),
            Err(IoError{ kind: EndOfFile, .. }) => break,
            _ => fail!("Error reading input")
        };
        stdio::stdout().write(bytes).unwrap();
    }

    return 0;
}
