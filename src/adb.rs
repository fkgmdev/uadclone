use std::process::Command;

pub fn get_packages() -> Vec<String> {
    let output = Command::new("adb")
        .arg("shell")
        .arg("pm")
        .arg("list")
        .arg("packages")
        .output()
        .expect("Failed to execute adb");
    if !output.status.success() {
        eprintln!("adb result error");
        std::process::exit(1);
    }
    let packages = String::from_utf8_lossy(&output.stdout).into_owned();
    let mut packagelist: Vec<String> = Vec::new();

    for line in packages.lines() {
        packagelist.push(line.replace("package:", "").trim().to_string());
    }
    return packagelist;
}