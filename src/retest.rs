use regex::Regex;
use term;

pub fn print_matches(pattern: &str, subject: &str) {
    // Attempt to compile the given regex pattern.
    let regex = match Regex::new(&pattern) {
        Ok(result) => { result },
        Err(_) => {
            println!("Invalid regular expression pattern '{}'.", pattern);
            return;
        }
    };

    // Grab a terminal object to write formatted output with.
    let mut term = term::stdout().unwrap();

    // Loop over each match in the subject and pretty-print the match as well as
    // any trailing, non-matching text preceding the match.
    let mut last_index = 0;
    for captures in regex.captures_iter(subject) {
        let positions = captures.pos(0).unwrap();

        print!("{}", &subject[last_index .. positions.0]);
        term.bg(term::color::BLUE).unwrap();
        print!("{}", captures.at(0).unwrap());
        term.reset().unwrap();

        last_index = positions.1;
    }

    // Print trailing unmatched text if there is any.
    if last_index < subject.len() {
        print!("{}", &subject[last_index ..]);
    }
}
