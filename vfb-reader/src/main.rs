use std::env;
use vfbreader::read_vfb;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    read_vfb(&path);
}
