use std::env;
use std::fs::{File,OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;
fn main() {
    create_file_log();
    let mut file =  OpenOptions::new().append(true).open("upload.log").expect("open file error");
    upload("README.md",&mut file);
}

fn upload(name: &str,file: &mut File) {
    let path = Path::new(env::current_dir().unwrap().as_os_str())
        .join("aliyundrive")
        .join("alidrive");
    match Command::new(path).arg(name).arg("GTNH").output() {
        Ok(output) => {
            file.write_all(String::from_utf8_lossy(&output.stdout).as_bytes()).expect("write file error");
            file.write_all(b"..............................................................................................................................................").expect("write file error");
        }
        Err(e) => {
            file.write_all(e.to_string().as_bytes()).expect("write file error");
        }
    }
}

fn create_file_log() {
    let is_exists = Path::new("upload.log").exists();
    if !is_exists {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("upload.log")
            .unwrap();
        file.write_all(b"").unwrap();
    }
}