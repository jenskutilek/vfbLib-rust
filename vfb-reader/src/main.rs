use std::env;
use vfbreader::read_vfb;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let vfb = read_vfb(path).expect("Failed to read VFB file");
    let json = serde_json::to_string_pretty(&vfb).expect("Serialization failed");
    println!("{}", json);
}
