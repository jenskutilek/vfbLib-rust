use std::env;
use vfbreader::read_vfb;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    match read_vfb(path) {
        Ok(vfb) => {
            let json = serde_json::to_string_pretty(&vfb).expect("Serialization failed");
            println!("{}", json);
        }
        Err(report) => {
            eprintln!("Error reading VFB file:");
            eprintln!("{:?}", report);
            std::process::exit(1);
        }
    }
}
