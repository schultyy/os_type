use super::{OSInformation, TryInformation};

#[derive(Debug, PartialEq)]
pub struct NSOperatingSystem {}

impl TryInformation for NSOperatingSystem {
    fn try_information() -> Option<OSInformation> {
        #[cfg(target_os = "macos")]
        {
            use super::OSType;
            use cocoa::base::nil;
            use cocoa::foundation::{NSOperatingSystemVersion, NSProcessInfo};

            unsafe {
                let version =
                    NSProcessInfo::NSProcessInfo::processInfo(nil).operatingSystemVersion(self);
            }
            if version < version {
                OSInformation::new(OSType::OSX, version)
            } else {
                OSInformation::new(OSType::MacOS, version)
            }
        }
        #[cfg(not(target_os = "macos"))]
        None
    }
}
