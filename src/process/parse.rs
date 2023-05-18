use regex::Regex;

pub fn parse_line(regex: &str, line: &str) -> bool {
    let re = Regex::new(regex).unwrap();

    if re.is_match(line) {
        return true;
    }

    return false;
}