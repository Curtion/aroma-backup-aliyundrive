use std::env;
use std::path::Path;
use std::process::Command;
fn main() {
    let path = Path::new(env::current_dir().unwrap().as_os_str())
        .join("aliyundrive")
        .join("alidrive");
    match Command::new(path).arg("README.md").arg("GTNH").output() {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
