use regex::{Captures, Regex};
use term::{self, color, Terminal};

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
    let mut terminal = term::stdout().unwrap();
    let color_cycle = [color::BLUE, color::GREEN, color::MAGENTA, color::YELLOW];

    // Get the string of the entire match. We will use this to print substrings
    // as we traverse the capture tree.
    let string = captures.at(0).unwrap();
    let offsets = captures.pos(0).unwrap();

    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push(offsets);

    let mut left_bound = 0;

    for i in 1..captures.len() {
        let pos = captures.pos(i).unwrap();

        if pos.1 <= stack.last().unwrap().1 {
            // Inside parent
            terminal.bg(color_cycle[(stack.len() % 4) - 1]).unwrap();
            print!("{}", &string[stack.last().unwrap().0 - offsets.0 .. pos.0 - offsets.0]);
            stack.push(pos);
        } else {
            left_bound = stack.last().unwrap().0;
            let right_bound = stack.last().unwrap().1;

            // Unwind stack until we find the correct parent.
            while pos.1 > stack.last().unwrap().1 {
                stack.pop().unwrap();
                terminal.bg(color_cycle[stack.len() % 4]).unwrap();
                print!("{}", &string[left_bound - offsets.0 .. right_bound - offsets.0]);
                left_bound = right_bound;
            }

            terminal.bg(color_cycle[(stack.len() % 4) - 1]).unwrap();
            print!("{}", &string[right_bound - offsets.0 .. pos.0 - offsets.0]);
            stack.push(pos);
            left_bound = pos.0;
        }
    }

    terminal.bg(color_cycle[0]).unwrap();
    print!("{}", &string[left_bound - offsets.0 .. offsets.1]);

    // Reset coloring to normal.
    terminal.reset().unwrap();
}
