/*
 * Mac OS X related checks
 */
use std::process::Command;

use super::{OSInformation, OSType, TryInformation};

#[derive(Debug, PartialEq)]
pub struct SwVers {
    pub product_version: Option<String>,
}

impl TryInformation for SwVers {
    fn try_information() -> Option<OSInformation> {
        Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
            .ok()
            .map(|version| {
                if version.starts_with("10") {
                    OSInformation::new(OSType::OSX, version)
                } else {
                    OSInformation::new(OSType::MacOS, version)
                }
            })
    }
}
