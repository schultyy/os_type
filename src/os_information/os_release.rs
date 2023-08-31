use std::fs::read_to_string;

use regex::Regex;
use utils::*;

use super::{OSInformation, OSType, TryInformation};

#[derive(Debug, PartialEq)]
pub struct OSRelease {
    distro: Option<String>,
    version: Option<String>,
}

impl TryInformation for OSRelease {
    fn try_information() -> Option<OSInformation> {
        retrieve().and_then(|r| {
            let version = r.version.unwrap_or(OSInformation::default_version());
            let distro = r
                .distro
                .and_then(|d| d.split_whitespace().next().map(str::to_string))
                .unwrap_or("".to_string())
                .to_lowercase();
            match distro.as_str() {
                "alpine" => Some(OSInformation::new(OSType::Alpine, version)),
                "arch" => Some(OSInformation::new(OSType::Arch, version)),
                "centos" => Some(OSInformation::new(OSType::CentOS, version)),
                "debian" => Some(OSInformation::new(OSType::Debian, version)),
                "deepin" => Some(OSInformation::new(OSType::Deepin, version)),
                "fedora" => Some(OSInformation::new(OSType::Fedora, version)),
                "kali" => Some(OSInformation::new(OSType::Kali, version)),
                "nixos" => Some(OSInformation::new(OSType::NixOS, version)),
                "opensuse" => Some(OSInformation::new(OSType::OpenSUSE, version)),
                "pop" => Some(OSInformation::new(OSType::PopOS, version)),
                "red" => Some(OSInformation::new(OSType::Redhat, version)),
                "ubuntu" => Some(OSInformation::new(OSType::Ubuntu, version)),
                _ => None,
            }
        })
    }
}

fn retrieve() -> Option<OSRelease> {
    read_to_string("/etc/os-release")
        .or_else(|_| read_to_string("/usr/lib/os-release"))
        .map(parse)
        .ok()
}

fn parse<S: AsRef<str>>(file: S) -> OSRelease {
    let distrib_regex = Regex::new(r#"NAME="(\w+)"#).unwrap();
    let version_regex = Regex::new(r#"VERSION_ID="?([\w\.]+)"#).unwrap();

    let distro = get_first_capture(&distrib_regex, &file);
    let version = get_first_capture(&version_regex, &file);

    OSRelease { distro, version }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ubuntu_18_04() {
        let sample = r#"NAME="Ubuntu"
VERSION="18.04 LTS (Bionic Beaver)"
ID=ubuntu
ID_LIKE=debian
PRETTY_NAME="Ubuntu 18.04 LTS"
VERSION_ID="18.04"
HOME_URL="https://www.ubuntu.com/"
SUPPORT_URL="https://help.ubuntu.com/"
BUG_REPORT_URL="https://bugs.launchpad.net/ubuntu"
PRIVACY_POLICY_URL="https://www.ubuntu.com/legal/terms-and-policies/privacy-policy"
VERSION_CODENAME=bionic
UBUNTU_CODENAME=bionic
"#;

        assert_eq!(
            parse(sample),
            OSRelease {
                distro: Some("Ubuntu".to_string()),
                version: Some("18.04".to_string()),
            }
        );
    }

    #[test]
    fn alpine_3_9_5() {
        let sample = r#"NAME="Alpine Linux"
ID=alpine
VERSION_ID=3.9.5
PRETTY_NAME="Alpine Linux v3.9"
HOME_URL="https://alpinelinux.org/"
BUG_REPORT_URL="https://bugs.alpinelinux.org/"
"#;

        assert_eq!(
            parse(sample),
            OSRelease {
                distro: Some("Alpine".to_string()),
                version: Some("3.9.5".to_string()),
            }
        );
    }

    #[test]
    fn deepin_20_3() {
        let sample = r#"PRETTY_NAME="Deepin 20.3"
NAME="Deepin"
VERSION_ID="20.3"
VERSION="20.3"
ID=Deepin
HOME_URL="https://www.deepin.org/"
"#;

        assert_eq!(
            parse(sample),
            OSRelease {
                distro: Some("Deepin".to_string()),
                version: Some("20.3".to_string()),
            }
        );
    }

    #[test]
    fn nixos_21_11() {
        let sample = r#"NAME=NixOS
ID=nixos
VERSION="21.11 (Porcupine)"
VERSION_CODENAME=porcupine
VERSION_ID="21.11"
BUILD_ID="21.11.20220325.d89f18a"
PRETTY_NAME="NixOS 21.11 (Porcupine)"
LOGO="nix-snowflake"
HOME_URL="https://nixos.org/"
DOCUMENTATION_URL="https://nixos.org/learn.html"
SUPPORT_URL="https://nixos.org/community.html"
BUG_REPORT_URL="https://github.com/NixOS/nixpkgs/issues"
"#;

        assert_eq!(
            parse(sample),
            OSRelease {
                distro: Some("NixOS".to_string()),
                version: Some("21.11".to_string()),
            }
        );
    }
    #[test]
    fn kali_2021_4() {
        let sample = r#"PRETTY_NAME="Kali Linux GNU/Linux Rolling"
NAME="Kali"
ID=kali
VERSION="2021.4"
VERSION_ID="2021.4"
VERSION_CODENAME="kali-rolling"
ID_LIKE=debian
HOME_URL="https://www.kali.org/"
SUPPORT_URL="https://forums.kali.org/"
BUG_REPORT_URL="https://bugs.kali.org"
"#;

        assert_eq!(
            parse(sample),
            OSRelease {
                distro: Some("Kali".to_string()),
                version: Some("2021.4".to_string()),
            }
        );
    }

    #[test]
    fn redhat_9_2() {
        let sample = r#"NAME="Red Hat Enterprise Linux"
VERSION="9.2 (Plow)"
ID="rhel"
ID_LIKE="fedora"
VERSION_ID="9.2"
PLATFORM_ID="platform:el9"
PRETTY_NAME="Red Hat Enterprise Linux 9.2 (Plow)"
ANSI_COLOR="0;31"
LOGO="fedora-logo-icon"
CPE_NAME="cpe:/o:redhat:enterprise_linux:9::baseos"
HOME_URL="https://www.redhat.com/"
DOCUMENTATION_URL="https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/9"
BUG_REPORT_URL="https://bugzilla.redhat.com/"

REDHAT_BUGZILLA_PRODUCT="Red Hat Enterprise Linux 9"
REDHAT_BUGZILLA_PRODUCT_VERSION=9.2
REDHAT_SUPPORT_PRODUCT="Red Hat Enterprise Linux"
REDHAT_SUPPORT_PRODUCT_VERSION="9.2"
"#;

        assert_eq!(
            parse(sample),
            OSRelease {
                distro: Some("Red".to_string()),
                version: Some("9.2".to_string()),
            }
        );
    }

    #[test]
    fn pop_os_22_04() {
        let sample = r#"NAME="Pop!_OS"
VERSION="22.04 LTS"
ID=pop
ID_LIKE="ubuntu debian"
PRETTY_NAME="Pop!_OS 22.04 LTS"
VERSION_ID="22.04"
HOME_URL="https://pop.system76.com"
SUPPORT_URL="https://support.system76.com"
BUG_REPORT_URL="https://github.com/pop-os/pop/issues"
PRIVACY_POLICY_URL="https://system76.com/privacy"
VERSION_CODENAME=jammy
UBUNTU_CODENAME=jammy
LOGO=distributor-logo-pop-os
"#;

        assert_eq!(
            parse(sample),
            OSRelease {
                distro: Some("Pop".to_string()),
                version: Some("22.04".to_string()),
            }
        );
    }
}
