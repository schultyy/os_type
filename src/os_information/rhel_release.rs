use regex::Regex;
use utils::*;

use super::{OSInformation, OSType, TryInformation};

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
                "red" => Some(OSInformation::new(OSType::RedHat, version)),
                _ => None,
            }
        })
    }
}

fn retrieve() -> Option<RHELRelease> {
    read_file("/etc/redhat-release")
        .map(parse)
        .or_else(|_| read_file("/etc/centos-release").map(parse))
        .or_else(|_| read_file("/etc/fedora-release").map(parse))
        .ok()
}

fn parse(file: String) -> RHELRelease {
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
        let parse_results = parse("CentOS Linux release 7.3.1611 (Core)".into());
        assert_eq!(parse_results.distro, Some("CentOS".to_string()));
        assert_eq!(parse_results.version, Some("7.3.1611".to_string()));
    }

    #[test]
    pub fn redhat_9_2() {
        let parse_results = parse("Red Hat Enterprise Linux release 9.2 (Plow)".into());
        assert_eq!(parse_results.distro, Some("Red Hat Enterprise".to_string()));
        assert_eq!(parse_results.version, Some("9.2".to_string()));
    }
}
