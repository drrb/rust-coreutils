use std::os;

fn main() {
    let exit_status = run(os::args());
    os::set_exit_status(exit_status);
}

fn run(args: Vec<String>) -> int {
    let path = Path::new(args[1].as_bytes());
    let last_segment = path.components().last().unwrap();
    println!("{}", String::from_utf8_lossy(last_segment));
    return 0;
}
