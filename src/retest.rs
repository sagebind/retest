use regex::{self, Captures, Regex};
use std::error;
use std::fmt;
use term::{self, color};

#[derive(Debug)]
pub enum Error {
    Regex(regex::Error)
}

/// Implement the standard error methods.
impl error::Error for Error {
    /// Gets the error description.
    fn description(&self) -> &str {
        match *self {
            Error::Regex(ref err) => err.description()
        }
    }

    /// Gets a previous error object, if any.
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Regex(ref err) => Some(err)
        }
    }
}

/// Implement display formatting for the error type.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Regex(ref err) => write!(f, "Regex error: {}", err)
        }
    }
}

/// Implement casting from other error types.
impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Error {
        Error::Regex(err)
    }
}

/// Finds all matches for the given pattern on the given subject and returns
/// the matches and their captures.
pub fn find_matches<'t>(pattern: &str, subject: &'t str) -> Result<Vec<Captures<'t>>, Error> {
    let regex = try!(Regex::new(&pattern));
    Ok(regex.captures_iter(subject).collect())
}

/// Prints out the given subject string, with the regions in the given matches
/// highlighted.
pub fn print_subject_highlighted<'t>(subject: &str, matches: &Vec<Captures<'t>>) {
    // Loop over each match in the subject and pretty-print the match as well as
    // any trailing, non-matching text preceding the match.
    let mut last_index = 0;

    for captures in matches.iter() {
        let positions = captures.get(0).unwrap();

        print!("{}", &subject[last_index .. positions.start()]);
        print_match(&subject, captures);

        last_index = positions.end();
    }

    // Print trailing unmatched text if there is any.
    if last_index < subject.len() {
        print!("{}", &subject[last_index ..]);
    }
    println!("");
}

/// Prints the given matches in a formatted list.
pub fn print_match_list<'t>(subject: &str, matches: &Vec<Captures<'t>>) {
    let mut match_id = 1;

    for captures in matches.iter() {
        let positions = captures.get(0).unwrap();

        print!("{:<4} {:<14} ", format!("{}.", match_id), format!("[{}-{}]", positions.start(), positions.end()));
        print_match(&subject, captures);
        println!("");
        match_id += 1;
    }
}

/// Prints out a match using color formatting.
///
/// The match is highlighted according to the captures that are matched. Each
/// capture is highlighted in a different color to distinguish it from other
/// captures. Because captures can be nested, this function uses a stack to keep
/// track of capture scope and display nested captures correctly.
///
/// The worst-case run time is O(2n log(n)), but the average run time should be
/// more like Θ(n + log(n)). The more nested the captures are, the worse the run
/// time is.
fn print_match(subject: &str, captures: &Captures) {
    let mut terminal = term::stdout().unwrap();
    let color_cycle = [color::BLUE, color::GREEN, color::MAGENTA, color::YELLOW];
    let mut color_index = 0;

    // To highlight the captures in a context-sensitive manner, set up a stack
    // to keep track of entering and exiting capture scopes. When examining a
    // capture, check if it fits inside the parent capture. If it doesn't, close
    // the scopes until we find a scope the capture does fit in.
    let mut stack: Vec<(usize, usize, u32)> = Vec::new();

    // Since we will print the captures as we go along, set up a cursor that
    // points to how much of the string we have printed out so far so we don't
    // accidentally print out the same regions of the strings more than once.
    let mut string_cursor = captures.get(0).unwrap().start();

    // Loop over each capture and find the scope it belongs to. Optional capture
    // groups that are not matched are ignored.
    for i in 0..captures.len() {
        let pos = match captures.get(i) {
            None => continue,
            Some(pos) => pos
        };

        // Unwind the stack until we find the correct parent.
        while !stack.is_empty() && pos.end() > stack.last().unwrap().1 {
            let scope = stack.pop().unwrap();
            terminal.bg(scope.2).unwrap();
            print!("{}", &subject[string_cursor .. scope.1]);
            string_cursor = scope.1;
        }

        // If the stack isn't empty, print out the head end of the parent scope
        // just before the current capture.
        if !stack.is_empty() {
            terminal.bg(stack.last().unwrap().2).unwrap();
            print!("{}", &subject[string_cursor .. pos.start()]);
            string_cursor = pos.start();
        }

        // Push the current capture onto the stack with a color selected from
        // the color cycle.
        stack.push((pos.start(), pos.end(), color_cycle[color_index]));
        color_index = (color_index + 1) % color_cycle.len();
    }

    // Finally, print out the tail of the match. The stack will still have
    // captures in it for the very last capture and its ancestors, so unwind the
    // stack one last time and print out each capture's tail.
    while !stack.is_empty() {
        let scope = stack.pop().unwrap();
        terminal.bg(scope.2).unwrap();
        print!("{}", &subject[string_cursor .. scope.1]);
        string_cursor = scope.1;
    }

    // Reset coloring to normal.
    terminal.reset().unwrap();
}
