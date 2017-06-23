use std::process::Command;

pub fn get_android_version() -> String {
    let output = Command::new("getprop")
        .arg("ro.build.version.release")
        .output();

    if output.is_ok() {
        let res = output.unwrap().stdout;
        let mut result = format!("{}", String::from_utf8_lossy(&res));

        let len = result.len();
        result.truncate(len - 1);

        return result;

    } else {
        super::default_version()
    }
}