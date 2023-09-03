use super::{OSInformation, OSType, TryInformation};
use regex::RegexBuilder;
use std::process::Command;
use utils::get_first_capture;

#[derive(Debug, PartialEq)]
pub struct LsbRelease {
    distro: Option<String>,
    version: Option<String>,
}

impl TryInformation for LsbRelease {
    fn try_information() -> Option<OSInformation> {
        retrieve().map(parse).and_then(|r| {
            let distro = r.distro.unwrap_or("".to_string()).to_lowercase();
            match distro.as_str() {
                "arch" => OSInformation::some_new(OSType::Arch, r.version),
                "centos" => OSInformation::some_new(OSType::CentOS, r.version),
                "debian" => OSInformation::some_new(OSType::Debian, r.version),
                "fedora" => OSInformation::some_new(OSType::Fedora, r.version),
                "kali" => OSInformation::some_new(OSType::Kali, r.version),
                "manjarolinux" => OSInformation::some_new(OSType::Manjaro, r.version),
                "nixos" => OSInformation::some_new(OSType::NixOS, r.version),
                "opensuse" => OSInformation::some_new(OSType::OpenSUSE, r.version),
                "ubuntu" => OSInformation::some_new(OSType::Ubuntu, r.version),
                _ => None,
            }
        })
    }
}

pub fn retrieve() -> Option<String> {
    Command::new("lsb_release")
        .arg("-a")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        .ok()
}

pub fn parse<S: AsRef<str>>(file: S) -> LsbRelease {
    let distrib_regex = RegexBuilder::new(r"^Distributor ID:\s*(\w+)")
        .multi_line(true)
        .build()
        .unwrap();
    let version_regex = RegexBuilder::new(r"^Release:\s*([\w\.]+)")
        .multi_line(true)
        .build()
        .unwrap();

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
"#;

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
Codename:	wheezy"#;
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
"#;
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
"#;
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
"#;
        assert_eq!(
            parse(sample),
            LsbRelease {
                distro: Some("openSUSE".to_string()),
                version: Some("20170712".to_string()),
            }
        );
    }

    #[test]
    pub fn nixos_21_11() {
        let sample = r#"No LSB modules are available.
Distributor ID:	NixOS
Description:	NixOS 21.11 (Porcupine)
Release:	21.11
Codename:	porcupine
"#;
        assert_eq!(
            parse(sample),
            LsbRelease {
                distro: Some("NixOS".to_string()),
                version: Some("21.11".to_string()),
            }
        );
    }
}
