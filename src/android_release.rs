#[cfg(target_os = "android")]
pub mod android_release {
    use std::process::Command;


    /// Parse the android version from the command line string.
    /// Expected version to be similar to: "5.1.1\n"
    ///
    /// returns: The version of android
    pub fn parse_android_version(version: &mut String) -> String {
        let len = version.len();
        version.truncate(len - 1); // Strip new line
        version.clone()
    }

    /// Get the android version. This command executes
    /// getprop on the command line, and parse the result
    /// by reading the result
    pub fn get_android_version() -> String {
        let output = Command::new("getprop")
            .arg("ro.build.version.release")
            .output();

        if output.is_ok() {
            let res = output.unwrap().stdout;
            let mut result = format!("{}", String::from_utf8_lossy(&res));
            parse_android_version(&mut result)
        } else {
            super::default_version()
        }
    }
}