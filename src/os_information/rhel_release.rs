use std::fs::read_to_string;

use regex::Regex;
use utils::*;

use super::{OSInformation, OSType, TryInformation};

#[derive(Debug, PartialEq)]
pub struct RHELRelease {
    distro: Option<String>,
    version: Option<String>,
}

impl TryInformation for RHELRelease {
    fn try_information() -> Option<OSInformation> {
        retrieve().and_then(|r| {
            let version = r.version.unwrap_or(OSInformation::default_version());
            let distro = r
                .distro
                .and_then(|d| d.split_whitespace().next().map(str::to_string))
                .unwrap_or("".to_string())
                .to_lowercase();
            match distro.as_str() {
                "centos" => Some(OSInformation::new(OSType::CentOS, version)),
                "red" => Some(OSInformation::new(OSType::Redhat, version)),
                _ => None,
            }
        })
    }
}

fn retrieve() -> Option<RHELRelease> {
    read_to_string("/etc/redhat-release")
        .or_else(|_| read_to_string("/etc/centos-release"))
        .or_else(|_| read_to_string("/etc/fedora-release"))
        .map(parse)
        .ok()
}

fn parse<S: AsRef<str>>(file: S) -> RHELRelease {
    let distrib_regex = Regex::new(r"((?:\w+(?:\s\w+)?)+) release").unwrap();
    let version_regex = Regex::new(r"release\s([\w\.]+)").unwrap();

    let distro = get_first_capture(&distrib_regex, &file);
    let version = get_first_capture(&version_regex, &file);

    RHELRelease { distro, version }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn centos_7_3_1611() {
        let sample = "CentOS Linux release 7.3.1611 (Core)";
        assert_eq!(
            parse(sample),
            RHELRelease {
                distro: Some("CentOS Linux".to_string()),
                version: Some("7.3.1611".to_string())
            }
        );
    }

    #[test]
    pub fn redhat_9_2() {
        let sample = "Red Hat Enterprise Linux release 9.2 (Plow)";
        assert_eq!(
            parse(sample),
            RHELRelease {
                distro: Some("Red Hat Enterprise Linux".to_string()),
                version: Some("9.2".to_string())
            }
        );
    }

    #[test]
    pub fn fedora_38() {
        let sample = "Fedora release 38 (Thirty Eight)";
        assert_eq!(
            parse(sample),
            RHELRelease {
                distro: Some("Fedora".to_string()),
                version: Some("38".to_string())
            }
        );
    }
}
