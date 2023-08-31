use std::fs::File;
use std::io::{Error, Read};

use regex::Regex;

pub fn read_file(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_first_capture(regex: &Regex, file: &str) -> Option<String> {
    regex
        .captures(file)
        .and_then(|capture| capture.get(1).map(|match_| match_.as_str().to_owned()))
}
