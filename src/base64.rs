extern crate getopts;
extern crate serialize;

use std::os;
use std::io::{stdio,IoError,EndOfFile};
use serialize::base64::{FromBase64,ToBase64,Config,Standard};
use getopts::{optflag, getopts};

const ENCODE_BLOCK_SIZE: uint = 30 * 1024;
const DECODE_BLOCK_SIZE: uint = 3 * 1024;

fn main() {
    let exit_status = run(os::args());
    os::set_exit_status(exit_status);
}

fn run(args: Vec<String>) -> int {
    let parameters = [
        optflag("D", "decode", "decodes input"),
    ];

    let options = match getopts(args.tail(), parameters) {
        Ok(options) => options,
        Err(failure) => fail!(failure.to_string())
    };

    let decoding = options.opt_present("D");
    if decoding {
        process(decode_block, [0, ..DECODE_BLOCK_SIZE]);
    } else {
        process(encode_block, [0, ..ENCODE_BLOCK_SIZE]);
        print(b"\n");
    }

    return 0;
}

fn process(encode_or_decode: fn (&[u8]), buffer: &mut [u8]) {
    loop {
        let input = match stdio::stdin_raw().read(buffer) {
            Ok(bytes_read) => buffer.slice_to(bytes_read),
            Err(IoError{ kind: EndOfFile, .. }) => return,
            _ => fail!("Error reading input")
        };
        encode_or_decode(input);
    }
}

fn encode_block(block: &[u8]) {
    let output = block.to_base64(Config { char_set: Standard, pad: true, line_length: None });
    print(output.as_bytes())
}

fn decode_block(block: &[u8]) {
    let output = match block.from_base64() {
        Ok(decoded) => decoded,
        Err(message) => fail!(format!("Failed to decode: {}", message))
    };
    print(output.as_slice())
}

fn print(bytes: &[u8]) {
    stdio::stdout().write(bytes).unwrap()
}
