use regex::Regex;

pub fn get_first_capture<S: AsRef<str>>(regex: &Regex, file: S) -> Option<String> {
    regex
        .captures(file.as_ref())
        .and_then(|capture| capture.get(1).map(|match_| match_.as_str().to_owned()))
}
