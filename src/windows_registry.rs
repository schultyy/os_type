use super::{OSInformation, TryInformation};

pub struct WindowsRegistry {}

impl TryInformation for WindowsRegistry {
    fn try_information() -> Option<OSInformation> {
        #[cfg(target_os = "windows")]
        {
            use super::OSType;
            use winreg::enums::*;
            use winreg::RegKey;

            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let cur_ver = hklm
                .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
                .ok()?;
            let major_ver: u32 = cur_ver.get_value("CurrentMajorVersionNumber").unwrap_or(0);
            let minor_ver: u32 = cur_ver.get_value("CurrentMinorVersionNumber").unwrap_or(0);
            let build_ver: String = cur_ver
                .get_value("CurrentBuildNumber")
                .unwrap_or("0".to_string());
            let version = format!("{major_ver}.{minor_ver}.{build_ver}");
            Some(OSInformation::new(OSType::Windows, version))
        }
        #[cfg(not(target_os = "windows"))]
        None
    }
}
