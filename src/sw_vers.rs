use super::{OSInformation, OSType, TryInformation};
use std::process::Command;

#[derive(Debug, PartialEq)]
pub struct SwVers {}

impl TryInformation for SwVers {
    fn try_information() -> Option<OSInformation> {
        Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
            .map(|version| {
                if version.starts_with("10") {
                    OSInformation::new(OSType::OSX, version)
                } else {
                    OSInformation::new(OSType::MacOS, version)
                }
            })
            .ok()
    }
}
