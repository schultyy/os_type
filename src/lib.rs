extern crate regex;
#[cfg(target_os = "windows")]
extern crate winreg;

mod lsb_release;
mod os_release;
mod rhel_release;
mod sw_vers;
mod uname;
mod utils;
mod windows_registry;

use self::{
    lsb_release::LsbRelease, os_release::OSRelease, rhel_release::RHELRelease, sw_vers::SwVers,
    uname::Uname, windows_registry::WindowsRegistry,
};
use std::fmt::Display;

/// A list of supported operating system types
#[derive(Debug, PartialEq, Clone)]
pub enum OSType {
    Unknown,

    // Windows
    Windows,
    Cygwin,

    // MacOS
    MacOS,
    OSX,

    // Linux
    GenericLinux,
    Alpine,
    Arch,
    CentOS,
    Debian,
    Deepin,
    Fedora,
    Kali,
    Manjaro,
    NixOS,
    OpenSUSE,
    PopOS,
    Redhat,
    Ubuntu,

    // BSD
    FreeBSD,
}

impl Display for OSType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OSType::Unknown => write!(f, "Unknown"),

            // Windows
            OSType::Windows => write!(f, "Windows"),
            OSType::Cygwin => write!(f, "Cygwin"),

            // Macos
            OSType::MacOS => write!(f, "macOS"),
            OSType::OSX => write!(f, "Mac OS X"),

            // Linux
            OSType::GenericLinux => write!(f, "Generic Linux"),
            OSType::Alpine => write!(f, "Alpine"),
            OSType::Arch => write!(f, "Arch"),
            OSType::CentOS => write!(f, "CentOS"),
            OSType::Debian => write!(f, "Debian"),
            OSType::Deepin => write!(f, "Deepin"),
            OSType::Fedora => write!(f, "Fedora"),
            OSType::Kali => write!(f, "Kali"),
            OSType::Manjaro => write!(f, "Manjaro"),
            OSType::NixOS => write!(f, "NixOS"),
            OSType::OpenSUSE => write!(f, "openSUSE"),
            OSType::PopOS => write!(f, "Pop!_OS"),
            OSType::Redhat => write!(f, "Red Hat"),
            OSType::Ubuntu => write!(f, "Ubuntu"),

            // BSD
            OSType::FreeBSD => write!(f, "FreeBSD"),
        }
    }
}

impl Default for OSType {
    fn default() -> Self {
        if cfg!(target_os = "windows") {
            Self::Windows
        } else if cfg!(target_os = "macos") {
            Self::MacOS
        } else if cfg!(target_os = "linux") {
            Self::GenericLinux
        } else if cfg!(target_os = "freebsd") {
            Self::FreeBSD
        } else {
            Self::Unknown
        }
    }
}

/// Holds information about Operating System type and its version
/// If the version could not be fetched it defaults to `0.0.0`
#[derive(Debug, Clone, PartialEq)]
pub struct OSInformation {
    pub os_type: self::OSType,
    pub version: String,
}

pub trait TryInformation {
    fn try_information() -> Option<OSInformation>;
}

impl Default for OSInformation {
    fn default() -> Self {
        Self {
            os_type: Default::default(),
            version: Self::default_version(),
        }
    }
}

impl OSInformation {
    #[inline]
    pub fn default_version() -> String {
        "0.0.0".to_string()
    }

    #[inline]
    pub(crate) fn new(os_type: OSType, version: String) -> Self {
        Self { os_type, version }
    }

    #[inline]
    pub(crate) fn some_new(os: OSType, version: Option<String>) -> Option<Self> {
        Some(Self::new(os, version.unwrap_or_else(Self::default_version)))
    }
}

impl OSInformation {
    pub fn current_platform() -> Self {
        Uname::try_information()
            .or_else(WindowsRegistry::try_information)
            .or_else(SwVers::try_information)
            .or_else(LsbRelease::try_information)
            .or_else(OSRelease::try_information)
            .or_else(RHELRelease::try_information)
            .unwrap_or_default()
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
    OSInformation::current_platform()
}
