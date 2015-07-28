use regex::{Captures, Regex};
use term::{self, Terminal};

/// Finds all matches for the given pattern on the given subject and prints out
/// the subject with matches highlighted.
pub fn print_matches(pattern: &str, subject: &str) {
    // Attempt to compile the given regex pattern.
    let regex = match Regex::new(&pattern) {
        Ok(result) => { result },
        Err(_) => {
            println!("Invalid regular expression pattern '{}'.", pattern);
            return;
        }
    };

    // Loop over each match in the subject and pretty-print the match as well as
    // any trailing, non-matching text preceding the match.
    let mut last_index = 0;
    for captures in regex.captures_iter(subject) {
        let positions = captures.pos(0).unwrap();

        print!("{}", &subject[last_index .. positions.0]);
        print_captures(captures);
        last_index = positions.1;
    }

    // Print trailing unmatched text if there is any.
    if last_index < subject.len() {
        print!("{}", &subject[last_index ..]);
    }
}

/// Prints out a capture group using color formatting.
fn print_captures(captures: Captures) {
    // Grab a terminal object to write formatted output with.
    let mut terminal = term::stdout().unwrap();

    terminal.bg(term::color::BLUE).unwrap();
    print!("{}", captures.at(0).unwrap());
    terminal.reset().unwrap();
}
