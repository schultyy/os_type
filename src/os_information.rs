use std::fmt;

///Supported operating system types
#[derive(Debug)]
#[derive(PartialEq)]
pub enum OSType {
    Unknown,
    Redhat,
    OSX,
    Ubuntu,
    Debian
}

impl fmt::Display for OSType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

///Information about the operating system such as type and version
#[derive(Debug)]
#[derive(PartialEq)]
pub struct OSInformation {
    pub operating_system: OSType,
    pub version: String
}
