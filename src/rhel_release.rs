use super::{OSInformation, OSType, TryInformation};
use regex::Regex;
use std::fs::read_to_string;
use utils::get_first_capture;

#[derive(Debug, PartialEq)]
pub struct RhelRelease {
    distro: Option<String>,
    version: Option<String>,
}

impl TryInformation for RhelRelease {
    fn try_information() -> Option<OSInformation> {
        retrieve().map(parse).and_then(|r| {
            let distro = r.distro.unwrap_or("".to_string()).to_lowercase();
            match distro.as_str() {
                "centos" => OSInformation::some_new(OSType::CentOS, r.version),
                "fedora" => OSInformation::some_new(OSType::Fedora, r.version),
                "red" => OSInformation::some_new(OSType::Redhat, r.version),
                _ => None,
            }
        })
    }
}

fn retrieve() -> Option<String> {
    read_to_string("/etc/redhat-release")
        .or_else(|_| read_to_string("/etc/centos-release"))
        .or_else(|_| read_to_string("/etc/fedora-release"))
        .ok()
}

fn parse<S: AsRef<str>>(file: S) -> RhelRelease {
    let distrib_regex = Regex::new(r"(\w+)(?:\s\w+)*\srelease").unwrap();
    let version_regex = Regex::new(r"release\s([\w\.]+)").unwrap();

    let distro = get_first_capture(&distrib_regex, &file);
    let version = get_first_capture(&version_regex, &file);

    RhelRelease { distro, version }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn centos_7_3_1611() {
        let sample = "CentOS Linux release 7.3.1611 (Core)";
        assert_eq!(
            parse(sample),
            RhelRelease {
                distro: Some("CentOS".to_string()),
                version: Some("7.3.1611".to_string())
            }
        );
    }

    #[test]
    pub fn redhat_9_2() {
        let sample = "Red Hat Enterprise Linux release 9.2 (Plow)";
        assert_eq!(
            parse(sample),
            RhelRelease {
                distro: Some("Red".to_string()),
                version: Some("9.2".to_string())
            }
        );
    }

    #[test]
    pub fn fedora_38() {
        let sample = "Fedora release 38 (Thirty Eight)";
        assert_eq!(
            parse(sample),
            RhelRelease {
                distro: Some("Fedora".to_string()),
                version: Some("38".to_string())
            }
        );
    }
}
