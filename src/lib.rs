extern crate regex;

use std::process::Command;
mod lsb_release;
mod windows_ver;
mod rhel_release;
mod sw_vers;
mod utils;
mod android_release;

///A list of supported operating system types
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum OSType {
    Unknown,
    Redhat,
    OSX,
    Ubuntu,
    Debian,
    Arch,
    CentOS,
    Android,
    Windows
}

/// Holds information about Operating System type and its version
/// If the version could not be fetched it defaults to `0.0.0`
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct OSInformation {
    pub os_type: self::OSType,
    pub version: String
}

fn lsb_release() -> OSInformation {
    match lsb_release::retrieve() {
        Some(release) => {
            if release.distro == Some("Ubuntu".to_string()) {
                OSInformation {
                    os_type: OSType::Ubuntu,
                    version: release.version.unwrap_or(default_version())
                }
            }
                else if release.distro == Some("Debian".to_string()) {
                    OSInformation {
                        os_type: OSType::Debian,
                        version: release.version.unwrap_or(default_version())
                    }
                } else if release.distro == Some("Arch".to_string()) {
                    OSInformation {
                        os_type: OSType::Arch,
                        version: release.version.unwrap_or(default_version())
                    }
                }
                    else if release.distro == Some("CentOS".to_string()){
                        OSInformation {
                            os_type: OSType::CentOS,
                            version: release.version.unwrap_or(default_version())
                        }
                    }
                        else {
                            unknown_os()
                        }
        },
        None => unknown_os()
    }
}

fn rhel_release() -> OSInformation {
    match rhel_release::retrieve() {
        Some(release) => {
            if release.distro == Some("CentOS".to_string()) {
                OSInformation {
                    os_type: OSType::CentOS,
                    version: release.version.unwrap_or(default_version())
                }
            } else {
                OSInformation {
                    os_type: OSType::Redhat,
                    version: release.version.unwrap_or(default_version())
                }
            }
        },
        None => unknown_os()
    }
}

fn default_version() -> String {
    "0.0.0".into()
}

fn unknown_os() -> OSInformation {
    OSInformation {
        os_type: OSType::Unknown,
        version: default_version()
    }
}


/// Get the current software version (if MacOS)
#[cfg(target_os = "macos")]
fn _current_platform() -> OSInformation {
    if let Some(osx_info) = sw_vers::mac_os::retrieve() {
        OSInformation {
            os_type: OSType::OSX,
            version: osx_info.product_version.unwrap_or(default_version())
        }
    } else {
        unknown_os()
    }
}

/// Get the release (if running of Android)
#[cfg(target_os = "android")]
fn _current_platform() -> OSInformation {
    let version = android_release::android_release::get_android_version();
    OSInformation {
        os_type: self::OSType::Android,
        version: version.unwrap_or(default_version())
    }
}

#[cfg(target_os = "windows")]
fn _current_platform() -> OSInformation {
    let version : String = match windows_ver::retrieve() {
        Some(v) => v.version,
        None => default_version()

    };

    OSInformation {
        os_type: self::OSType::Windows,
        version: version
    }
}

#[cfg(not(any(target_os ="android", target_os ="macos", target_os = "windows")))]
fn _current_platform() -> OSInformation {
    if lsb_release::is_available() {
        lsb_release()
    }
    else if utils::file_exists("/etc/redhat-release") || utils::file_exists("/etc/centos-release") {
        rhel_release()
    }
    else {
        unknown_os()
    }
}

///Returns the current operating system type
///
///#Example
///
///```
///use os_type;
///let os = os_type::current_platform();
///println!("Type: {:?}", os.os_type);
///println!("Version: {}", os.version);
///```
pub fn current_platform() -> OSInformation {
    _current_platform()
}
