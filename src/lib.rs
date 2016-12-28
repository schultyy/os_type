use std::process::Command;
use std::fs;
use std::convert::AsRef;
use std::path::Path;
use std::collections::HashSet;

mod lsb_release;
mod windows_ver;

/// A list of supported operating system types
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum OSType {
    Unknown,
    Redhat,
    Windows,
    OSX,
    Distro(&'static str),
}

fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    let metadata = fs::metadata(path);

    match metadata {
        Ok(md) => md.is_dir() || md.is_file(),
        Err(_) => false,
    }
}

fn is_windows() -> bool {
    if cfg!(target_os = "windows") {
        return true;
    } else {
        return false;
    }
}

fn is_os_x() -> bool {
    match Command::new("sw_vers").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn lsb_release() -> OSType {
    // Some distro'name begin with a lowercase letter, refer to the official website. example is 'openSUSE'.
    let distros_strvec: Vec<&'static str> = vec!["openSUSE",
                                                 "Ubuntu",
                                                 "Debian",
                                                 "Arch",
                                                 "Mint",
                                                 "Manjaro",
                                                 "elementary",
                                                 "Fedora",
                                                 "Zorin",
                                                 "deepin"];
    let distros: HashSet<&'static str> = distros_strvec.into_iter().collect();
    match lsb_release::retrieve() {
        Some(release) => {
            let mut os_type = OSType::Unknown;
            for osname in distros.iter() {
                if release.distro == Some(osname.to_string()) {
                    os_type = OSType::Distro(osname);
                    break;
                }
            }
            os_type
        }
        None => OSType::Unknown,
    }

}

/// Returns the current operating system type
///
/// #Example
///
/// ```
/// use os_type;
/// let os = os_type::current_platform();
/// ```
pub fn current_platform() -> OSType {
    if is_os_x() {
        OSType::OSX
    } else if is_windows() {
        OSType::Windows
    } else if lsb_release::is_available() {
        lsb_release()
    } else if file_exists("/etc/redhat-release") || file_exists("/etc/centos-release") {
        OSType::Redhat
    } else {
        OSType::Unknown
    }
}
