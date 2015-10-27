/*
 * Mac OS X related checks
 */
use std::process::Command;
extern crate regex;
use self::regex::Regex;

pub struct SwVers {
    pub product_name: String,
    pub product_version: String,
    pub build_version: String
}

fn extract_from_regex(stdout: &String, regex: Regex, group: usize) -> Option<String> {
    match regex.captures_iter(&stdout).next() {
        Some(m) => {
            match m.at(group) {
                Some(s) => {
                    Some(s.to_string())
                },
                None => None
            }
        },
        None => None
    }
}

pub fn is_os_x() -> bool {
    match Command::new("sw_vers").output() {
        Ok(output) => output.status.success(),
        Err(_) => false
    }
}

pub fn retrieve() -> Option<SwVers> {
    None
}

pub fn parse(version_str: String) -> SwVers {
    let product_name_regex = Regex::new(r"ProductName:\s([\w\s]+)\n").unwrap();
    let product_version_regex = Regex::new(r"ProductVersion:\s(\w+\.\w+\.\w+)").unwrap();
    let build_number_regex = Regex::new(r"BuildVersion:\s(\w+)").unwrap();

    SwVers {
        product_name: extract_from_regex(&version_str, product_name_regex, 1).unwrap(),
        product_version: extract_from_regex(&version_str, product_version_regex, 1).unwrap(),
        build_version: extract_from_regex(&version_str, build_number_regex, 1).unwrap(),
    }
}
