use std::process::Command;

pub fn get_android_version() -> String {
    let output = Command::new("getprop")
        .arg("ro.build.version.release")
        .output();

    if output.is_ok() {
        let res = output.unwrap().stdout;
        format!("{}", String::from_utf8_lossy(&res))
    } else {
        super::default_version()
    }
}