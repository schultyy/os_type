use regex::Regex;
use std::process::Command;
use utils::*;

use super::{OSInformation, OSType, TryInformation};

#[derive(Debug, PartialEq)]
pub struct LsbRelease {
    distro: Option<String>,
    version: Option<String>,
}

impl TryInformation for LsbRelease {
    fn try_information() -> Option<OSInformation> {
        retrieve().and_then(|r| {
            let version = r.version.unwrap_or(OSInformation::default_version());
            let distro = r
                .distro
                .and_then(|d| d.split_whitespace().next().map(str::to_string))
                .unwrap_or("".to_string())
                .to_lowercase();
            match distro.as_str() {
                "arch" => Some(OSInformation::new(OSType::Arch, version)),
                "centos" => Some(OSInformation::new(OSType::CentOS, version)),
                "debian" => Some(OSInformation::new(OSType::Debian, version)),
                "fedora" => Some(OSInformation::new(OSType::Fedora, version)),
                "kali" => Some(OSInformation::new(OSType::Kali, version)),
                "manjarolinux" => Some(OSInformation::new(OSType::Manjaro, version)),
                "nixos" => Some(OSInformation::new(OSType::NixOS, version)),
                "opensuse" => Some(OSInformation::new(OSType::OpenSUSE, version)),
                "ubuntu" => Some(OSInformation::new(OSType::Ubuntu, version)),
                _ => None,
            }
        })
    }
}

pub fn retrieve() -> Option<LsbRelease> {
    Command::new("lsb_release")
        .arg("-a")
        .output()
        .map(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            parse(stdout.to_string())
        })
        .ok()
}

pub fn parse(file: String) -> LsbRelease {
    let distrib_regex = Regex::new(r"Distributor ID:\s*(\w+)").unwrap();
    let version_regex = Regex::new(r"Release:\s*([\w\.]+)").unwrap();

    let distro = get_first_capture(&distrib_regex, &file);
    let version = get_first_capture(&version_regex, &file);

    LsbRelease { distro, version }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fedora_38() {
        let sample = r#"LSB Version:	:core-4.1-amd64:core-4.1-noarch
Distributor ID:	Fedora
Description:	Fedora release 38 (Thirty Eight)
Release:	38
Codename:	ThirtyEight
"#
        .to_string();

        assert_eq!(
            parse(sample),
            LsbRelease {
                distro: Some("Fedora".to_string()),
                version: Some("38".to_string()),
            }
        );
    }

    #[test]
    pub fn debian_7_8() {
        let sample = r#"Distributor ID:	Debian
Description:	Debian GNU/Linux 7.8 (wheezy)
Release:	7.8
Codename:	wheezy"#
            .to_string();
        assert_eq!(
            parse(sample),
            LsbRelease {
                distro: Some("Debian".to_string()),
                version: Some("7.8".to_string()),
            }
        );
    }

    #[test]
    pub fn arch_rolling() {
        let sample = r#"LSB Version:	1.4
Distributor ID:	Arch
Description:	Arch Linux
Release:	rolling
Codename:	n/a
"#
        .to_string();
        assert_eq!(
            parse(sample),
            LsbRelease {
                distro: Some("Arch".to_string()),
                version: Some("rolling".to_string()),
            }
        );
    }

    #[test]
    pub fn manjaro_linux_17_1_7() {
        let sample = r#"LSB Version:	n/a
Distributor ID:	ManjaroLinux
Description:	Manjaro Linux
Release:	17.1.7
Codename:	Hakoila
"#
        .to_string();
        assert_eq!(
            parse(sample),
            LsbRelease {
                distro: Some("ManjaroLinux".to_string()),
                version: Some("17.1.7".to_string()),
            }
        );
    }

    #[test]
    pub fn opensuse_20170712() {
        let sample = r#"LSB Version:    n/a
Distributor ID: openSUSE
Description:    openSUSE Tumbleweed
Release:        20170712
Codename:       n/a
"#
        .to_string();
        assert_eq!(
            parse(sample),
            LsbRelease {
                distro: Some("openSUSE".to_string()),
                version: Some("20170712".to_string()),
            }
        );
    }

    #[test]
    pub fn tests_nixos_lsb_distro() {
        let sample = r#"No LSB modules are available.
Distributor ID:	NixOS
Description:	NixOS 21.11 (Porcupine)
Release:	21.11
Codename:	porcupine
"#
        .to_string();
        assert_eq!(
            parse(sample),
            LsbRelease {
                distro: Some("NixOS".to_string()),
                version: Some("21.11".to_string()),
            }
        );
    }
}
