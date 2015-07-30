use regex::{self, Captures, Regex};
use std::error;
use std::fmt;
use term::{self, color, Terminal};

#[derive(Debug)]
pub enum Error {
    Regex(regex::Error)
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Regex(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Regex(ref err) => Some(err)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Regex(ref err) => write!(f, "Regex error: {}", err)
        }
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Error {
        Error::Regex(err)
    }
}

/// Returns an iterator for finding all matches of `pattern` in `subject`.
pub fn find_matches<'t>(pattern: &str, subject: &'t str) -> Result<Vec<Captures<'t>>, Error> {
    let regex = try!(Regex::new(&pattern));
    Ok(regex.captures_iter(subject).collect())
}

/// Finds all matches for the given pattern on the given subject and prints out
/// the subject with matches highlighted.
pub fn print_subject_highlighted<'t>(subject: &str, matches: &Vec<Captures<'t>>) {
    // Loop over each match in the subject and pretty-print the match as well as
    // any trailing, non-matching text preceding the match.
    let mut last_index = 0;

    for captures in matches.iter() {
        let positions = captures.pos(0).unwrap();

        print!("{}", &subject[last_index .. positions.0]);
        print_match(captures);

        last_index = positions.1;
    }

    // Print trailing unmatched text if there is any.
    if last_index < subject.len() {
        print!("{}", &subject[last_index ..]);
    }
    println!("");
}

/// Prints the matches in a list format.
pub fn print_match_list<'t>(matches: &Vec<Captures<'t>>) {
    let mut match_id = 1;

    for captures in matches.iter() {
        print!("{:<4} ", format!("{}.", match_id));
        print_match(captures);
        println!("");
        match_id += 1;
    }
}

/// Prints out a match using color formatting.
fn print_match(captures: &Captures) {
    let mut terminal = term::stdout().unwrap();
    let color_cycle = [color::BLUE, color::GREEN, color::MAGENTA, color::YELLOW];
    let mut color_index = 1;

    // Get the string of the entire match and the relative offset in the subject.
    let string = captures.at(0).unwrap();
    let offsets = captures.pos(0).unwrap();

    // To highlight sub-matches, divide the match string into a series of
    // resizeable regions that store their range and their color. As we loop
    // over each capture, find the parent region the current capture group fits
    // into, and split the region in two, with a new region for the current
    // capture between it. Worst-case time is O(3n^2), where n is the number of
    // subgroups in the regular expression.
    let mut regions: Vec<(usize, usize, u16)> = Vec::new();
    regions.push((offsets.0, offsets.1, color::BLUE));

    for i in 1..captures.len() {
        let pos = match captures.pos(i) {
            None => continue,
            Some(pos) => pos
        };

        for j in 0..regions.len() {
            if pos.0 >= regions[j].0 && pos.1 <= regions[j].1 {
                // Define two new regions that overlap the old one.
                let middle = (pos.0, pos.1, color_cycle[color_index]);
                let right = (pos.1, regions[j].1, regions[j].2);
                // Shrink the old region to be the leftmost one.
                regions[j].1 = pos.0;

                // Insert the new regions.
                regions.insert(j + 1, middle);
                regions.insert(j + 2, right);

                // Choose next color next time.
                color_index = (color_index + 1) % 4;
                break;
            }
        }
    }

    // Now print out the prepared regions; this part is pretty easy, since the
    // region parser already did all the work.
    for i in 0..regions.len() {
        terminal.bg(regions[i].2).unwrap();
        print!("{}", &string[regions[i].0 - offsets.0 .. regions[i].1 - offsets.0]);
    }

    // Reset coloring to normal.
    terminal.reset().unwrap();
}
