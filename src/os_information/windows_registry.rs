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
            cur_ver
                .get_value("CurrentBuild")
                .map(|s| OSInformation::new(OSType::Windows, s))
                .ok()
        }
        #[cfg(not(target_os = "windows"))]
        None
    }
}
