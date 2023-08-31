extern crate regex;
#[cfg(target_os = "windows")]
extern crate winreg;

mod os_information;
mod utils;
pub use os_information::{OSInformation, OSType};

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
