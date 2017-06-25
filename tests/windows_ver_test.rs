extern crate regex;

#[path="../src/windows_ver.rs"]
mod windows_ver;

#[cfg(test)]
fn default_version() -> String {
    "0.0.0".into()
}

#[test]
pub fn test_windows_version_parse() {
    let ver_result = "Version:\r\nMicrosoft Windows [Version 10.0.14393]".to_string();
    let result = windows_ver::parse(ver_result);
    assert_eq!(result.version, "10");

}