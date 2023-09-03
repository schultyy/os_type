use super::{OSInformation, OSType, TryInformation};
use regex::RegexBuilder;
use std::fs::read_to_string;
use utils::get_first_capture;

#[derive(Debug, PartialEq)]
pub struct OsRelease {
    distro: Option<String>,
    version: Option<String>,
}

impl TryInformation for OsRelease {
    fn try_information() -> Option<OSInformation> {
        retrieve().map(parse).and_then(|r| {
            let distro = r.distro.unwrap_or("".to_string()).to_lowercase();
            match distro.as_str() {
                "alpine" => OSInformation::some_new(OSType::Alpine, r.version),
                "arch" => OSInformation::some_new(OSType::Arch, r.version),
                "centos" => OSInformation::some_new(OSType::CentOS, r.version),
                "debian" => OSInformation::some_new(OSType::Debian, r.version),
                "deepin" => OSInformation::some_new(OSType::Deepin, r.version),
                "fedora" => OSInformation::some_new(OSType::Fedora, r.version),
                "freebsd" => OSInformation::some_new(OSType::FreeBSD, r.version),
                "kali" => OSInformation::some_new(OSType::Kali, r.version),
                "nixos" => OSInformation::some_new(OSType::NixOS, r.version),
                "opensuse" => OSInformation::some_new(OSType::OpenSUSE, r.version),
                "pop" => OSInformation::some_new(OSType::PopOS, r.version),
                "rhel" => OSInformation::some_new(OSType::Redhat, r.version),
                "ubuntu" => OSInformation::some_new(OSType::Ubuntu, r.version),
                _ => None,
            }
        })
    }
}

fn retrieve() -> Option<String> {
    read_to_string("/etc/os-release")
        .or_else(|_| read_to_string("/usr/lib/os-release"))
        .ok()
}

fn parse<S: AsRef<str>>(file: S) -> OsRelease {
    let distrib_regex = RegexBuilder::new(r#"^ID="?(\w+)"#)
        .multi_line(true)
        .build()
        .unwrap();
    let version_regex = RegexBuilder::new(r#"^VERSION_ID="?([\w\.]+)"#)
        .multi_line(true)
        .build()
        .unwrap();

    let distro = get_first_capture(&distrib_regex, &file);
    let version = get_first_capture(&version_regex, &file);

    OsRelease { distro, version }
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
            OsRelease {
                distro: Some("ubuntu".to_string()),
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
            OsRelease {
                distro: Some("alpine".to_string()),
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
            OsRelease {
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
            OsRelease {
                distro: Some("nixos".to_string()),
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
            OsRelease {
                distro: Some("kali".to_string()),
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
            OsRelease {
                distro: Some("rhel".to_string()),
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
            OsRelease {
                distro: Some("pop".to_string()),
                version: Some("22.04".to_string()),
            }
        );
    }

    #[test]
    fn opensuse_leap_15_0() {
        let sample = r#"NAME="openSUSE Leap"
VERSION="15.0"
ID="opensuse-leap"
ID_LIKE="suse opensuse"
VERSION_ID="15.0"
PRETTY_NAME="openSUSE Leap 15.0"
ANSI_COLOR="0;32"
CPE_NAME="cpe:/o:opensuse:leap:15.0"
BUG_REPORT_URL="https://bugs.opensuse.org"
HOME_URL="https://www.opensuse.org/"
"#;
        assert_eq!(
            parse(sample),
            OsRelease {
                distro: Some("opensuse".to_string()),
                version: Some("15.0".to_string())
            }
        )
    }

    #[test]
    fn opensuse_tumbleweed_20180530() {
        let sample = r#"NAME="openSUSE Tumbleweed"
# VERSION="20180530"
ID="opensuse-tumbleweed"
ID_LIKE="suse opensuse"
VERSION_ID="20180530"
PRETTY_NAME="openSUSE Tumbleweed"
ANSI_COLOR="0;32"
CPE_NAME="cpe:/o:opensuse:tumbleweed:20180530"
BUG_REPORT_URL="https://bugs.opensuse.org"
HOME_URL="https://www.opensuse.org/"
"#;
        assert_eq!(
            parse(sample),
            OsRelease {
                distro: Some("opensuse".to_string()),
                version: Some("20180530".to_string())
            }
        )
    }

    #[test]
    fn freebsd_12_4() {
        let sample = r#"NAME=FreeBSD
VERSION="12.4-RELEASE"
VERSION_ID="12.4"
ID=freebsd
ANSI_COLOR="0;31"
PRETTY_NAME="FreeBSD 12.4-RELEASE"
CPE_NAME="cpe:/o:freebsd:freebsd:12.4"
HOME_URL="https://FreeBSD.org/"
BUG_REPORT_URL="https://bugs.FreeBSD.org/"
"#;
        assert_eq!(
            parse(sample),
            OsRelease {
                distro: Some("freebsd".to_string()),
                version: Some("12.4".to_string())
            }
        )
    }
}
