use super::{OSInformation, OSType, TryInformation};
use regex::Regex;
use std::process::Command;
use utils::get_first_capture;

#[derive(Debug, PartialEq)]
pub struct Uname {
    distro: Option<String>,
    version: Option<String>,
}

impl TryInformation for Uname {
    fn try_information() -> Option<OSInformation> {
        retrieve().and_then(|r| {
            let version = r.version.unwrap_or(OSInformation::default_version());
            let distro = r.distro.unwrap_or("".to_string()).to_lowercase();
            match distro.as_str() {
                "cygwin" => Some(OSInformation::new(OSType::Cygwin, version)),
                _ => None,
            }
        })
    }
}

fn retrieve() -> Option<Uname> {
    Command::new("uname")
        .arg("-or")
        .output()
        .map(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            parse(stdout.trim())
        })
        .ok()
}

fn parse<S: AsRef<str>>(file: S) -> Uname {
    let distrib_regex = Regex::new(r"(\w+)$").unwrap();
    let version_regex = Regex::new(r"^([\w\.]+)").unwrap();

    let distro = get_first_capture(&distrib_regex, &file);
    let version = get_first_capture(&version_regex, &file);

    Uname { distro, version }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cygwin_3_4_8() {
        let sample = r#"3.4.8-1.x86_64 Cygwin"#;

        assert_eq!(
            parse(sample),
            Uname {
                distro: Some("Cygwin".to_string()),
                version: Some("3.4.8".to_string()),
            }
        );
    }
}
