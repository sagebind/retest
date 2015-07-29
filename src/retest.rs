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
        let pos = captures.pos(i).unwrap();

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
