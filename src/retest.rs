extern crate regex;

pub fn test(pattern: &str, subject: &str) {
    let re = regex::Regex::new(&pattern).unwrap();
    assert!(re.is_match(&subject));
}
