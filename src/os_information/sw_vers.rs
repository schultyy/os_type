/*
 * Mac OS X related checks
 */
use regex::Regex;
use std::process::Command;
use utils::*;

use super::{OSInformation, OSType, TryInformation};

#[derive(Debug, PartialEq)]
pub struct SwVers {
    pub product_version: Option<String>,
}

impl TryInformation for SwVers {
    fn try_information() -> Option<OSInformation> {
        retrieve().map(|r| {
            let version = r
                .product_version
                .unwrap_or(OSInformation::default_version());
            if version.starts_with("10") {
                OSInformation::new(OSType::OSX, version)
            } else {
                OSInformation::new(OSType::MacOS, version)
            }
        })
    }
}

fn retrieve() -> Option<SwVers> {
    Command::new("sw_vers")
        .arg("-a")
        .output()
        .map(|output| {
            let stdout = String::from_utf8_lossy(&output.stdout);
            parse(stdout.to_string())
        })
        .ok()
}

fn parse(version_str: String) -> SwVers {
    let product_version_regex = Regex::new(r"ProductVersion:\s(\w+\.\w+\.\w+)").unwrap();

    let product_version = get_first_capture(&product_version_regex, &version_str);

    SwVers { product_version }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn macos_12_6_7_21g651() {
        let sample = r#"ProductName:	macOS
ProductVersion:	12.6.7
BuildVersion:	21G651
"#
        .to_string();

        assert_eq!(
            parse(sample),
            SwVers {
                product_version: Some("12.6.7".to_string()),
            }
        );
    }

    #[test]
    fn macos_10_10_5_14f27() {
        let sample = r#"ProductName:	Mac OS X
ProductVersion:	10.10.5
BuildVersion:	14F27
"#
        .to_string();

        assert_eq!(
            parse(sample),
            SwVers {
                product_version: Some("10.10.5".to_string()),
            }
        );
    }
}
