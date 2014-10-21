use std::os;
use std::str;

fn main() {
    let exit_status = run(os::args());
    os::set_exit_status(exit_status);
}

fn run(args: Vec<String>) -> int {
    let path = Path::new(args[1].as_slice());
    let suffix_to_strip = match args.len() {
        2 => None,
        _ => Some(args[2].as_slice())
    };

    let last_segment = str::from_utf8(path.components().last().unwrap()).unwrap();

    let basename = match suffix_to_strip {
        None => last_segment,
        Some(suffix) => {
            if last_segment.ends_with(suffix) && last_segment != suffix {
                last_segment.slice_to(last_segment.len() - suffix.len())
            } else {
                last_segment
            }
        }
    };
    println!("{}", basename);
    return 0;
}
