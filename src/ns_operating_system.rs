use super::{OSInformation, TryInformation};

#[derive(Debug, PartialEq)]
pub struct NSOperatingSystem {}

impl TryInformation for NSOperatingSystem {
    fn try_information() -> Option<OSInformation> {
        #[cfg(target_os = "macos")]
        {
            use super::OSType;
            use cocoa_foundation::base::nil;
            use cocoa_foundation::foundation::NSProcessInfo;

            let os_version = unsafe { NSProcessInfo::processInfo(nil).operatingSystemVersion() };
            let version = format!(
                "{}.{}.{}",
                os_version.majorVersion, os_version.minorVersion, os_version.patchVersion
            );
            if os_version.majorVersion <= 10 {
                Some(OSInformation::new(OSType::OSX, version))
            } else {
                Some(OSInformation::new(OSType::MacOS, version))
            }
        }
        #[cfg(not(target_os = "macos"))]
        None
    }
}
