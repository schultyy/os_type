extern crate regex;
use std::process::Command;
use std::fs;
use std::convert::AsRef;
use std::path::Path;
mod lsb_release;
pub mod os_information;
use os_information::{OSType, OSInformation};
mod sw_vers;

fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    let metadata = fs::metadata(path);

    match metadata {
        Ok(md) => md.is_dir() || md.is_file(),
        Err(_) => false
    }
}

fn is_os_x() -> bool {
    match Command::new("sw_vers").output() {
        Ok(output) => output.status.success(),
        Err(_) => false
    }
}

fn lsb_release() -> Option<OSInformation> {
    match lsb_release::retrieve() {
        Some(release) => {
            if release.distro == Some("Ubuntu".to_string()) {
                Some(OSInformation {
                    operating_system: OSType::Ubuntu,
                    version: release.version.unwrap_or("0.0.0".into())
                })
            }
            else if release.distro == Some("Debian".to_string()) {
                Some(OSInformation {
                    operating_system: OSType::Debian,
                    version: release.version.unwrap_or("0.0.0".into())
                })
            }
            else {
                None
            }
        },
        None => None
    }

}

///Returns the current operating system type
///
///#Example
///
///```
///use os_type;
///let os = os_type::current_platform();
///println!("Operating system {}", os.operating_system);
///println!("Operating system version {}", os.version);
///```
pub fn current_platform() -> OSInformation {

    let unknown_os = OSInformation {
        operating_system: OSType::Unknown,
        version: "0.0.0".into()
    };

    if is_os_x() {
        let osx_info = match sw_vers::retrieve() {
            Some(osx) => osx,
            None => return unknown_os
        };

        OSInformation {
            operating_system: OSType::OSX,
            version: osx_info.product_version.unwrap_or("0.0.0".into())
        }
    }
    else if lsb_release::is_available() {
        lsb_release().unwrap_or(unknown_os)
    }
    else if file_exists("/etc/redhat-release") || file_exists("/etc/centos-release") {
        OSInformation {
            operating_system: OSType::Redhat,
            version: "0.0.0".into()
        }
    }
    else {
        unknown_os
    }
}
