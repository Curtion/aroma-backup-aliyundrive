use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;
fn main() {
    let last_name = get_will_last_filename();
    let path = file2name(&last_name);
    let pathfile = Path::new(&path).join(last_name);
    upload(pathfile.as_os_str().to_str().unwrap());
}

fn upload(name: &str) {
    let path = Path::new(env::current_dir().unwrap().as_os_str())
        .join("aliyundrive")
        .join("alidrive");
    match Command::new(path).arg(name).arg("GTNH").output() {
        Ok(_) => {
            println!("提交上传任务成功");
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn get_will_last_filename() -> String {
    let path = Path::new(env::current_dir().expect("获取当前路径失败").as_os_str())
        .join("World")
        .join("backupstore.txt");
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e);
            return String::new();
        }
    };
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("读取backupstore.txt文件失败");
    let lines = content.split("\r\n");
    let mut vesc = lines.collect::<Vec<&str>>();
    vesc.pop();
    match vesc.pop() {
        Some(last) => equal2reduce(last),
        None => {
            println!("文件内容为空");
            String::new()
        }
    }
}

fn equal2reduce(name: &str) -> String {
    //World=2022=7=29=22=59"
    let vesc = name.split("=").collect::<Vec<&str>>();
    let mut name = "Backup".to_string();
    name.push('-');
    name.push_str(vesc[0]);
    name.push('-');
    name.push_str(vesc[1]);
    name.push('-');
    name.push_str(vesc[2]);
    name.push('-');
    name.push_str(vesc[3]);
    name.push_str("--");
    name.push_str(vesc[4]);
    name.push('-');
    name.push_str(vesc[5]);
    name.push_str(".zip");
    name
}

fn file2name(filename: &str) -> String {
    //Backup-World-2022-7-29--22-59.zip
    let keys = filename.split("-").collect::<Vec<&str>>();
    let filepath = Path::new(env::current_dir().expect("获取当前路径失败").as_os_str())
        .join(keys[1])
        .join(keys[2])
        .join(keys[3])
        .join(keys[4]);
    filepath.to_str().expect("解析路径失败").to_string()
}
