use std::process::Command;

use regex::Regex;
use utils::*;

use super::{OSInformation, OSType, TryInformation};

#[derive(Debug, PartialEq)]
pub struct Uname {
    pub distro: Option<String>,
    pub version: Option<String>,
}

impl TryInformation for Uname {
    fn try_information() -> Option<OSInformation> {
        retrieve().and_then(|r| {
            let version = r.version.unwrap_or(OSInformation::default_version());
            let distro = r
                .distro
                .and_then(|d| d.split_whitespace().next().map(str::to_string))
                .unwrap_or("".to_string())
                .to_lowercase();
            match distro.as_str() {
                "cygwin_nt" => Some(OSInformation::new(OSType::Cygwin, version)),
                _ => None,
            }
        })
    }
}

fn retrieve() -> Option<Uname> {
    Command::new("uname")
        .arg("-a")
        .output()
        .map(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            parse(&stdout)
        })
        .ok()
}

fn parse(file: &str) -> Uname {
    let distrib_regex = Regex::new(r#"^(.*?)-"#).unwrap();
    let version_regex = Regex::new(r#"Windows\s(.*?)-"#).unwrap();

    let distro = get_first_capture(&distrib_regex, file);
    let version = get_first_capture(&version_regex, file);

    Uname { distro, version }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cygwin_3_4_8() {
        let sample =
            r#"CYGWIN_NT-10.0-22621 Windows 3.4.8-1.x86_64 2023-08-17 17:02 UTC x86_64 Cygwin"#;

        assert_eq!(
            parse(sample),
            Uname {
                distro: Some("CYGWIN_NT".to_string()),
                version: Some("3.4.8".to_string()),
            }
        );
    }
}
