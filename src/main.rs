extern crate getopts;
extern crate regex;
extern crate term;
mod retest;

use getopts::Options;
use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::process;

const VERSION: &'static str = "0.2.1";

/// Prints the program usage to the console.
fn print_usage(options: Options) {
    let brief = "Usage: retest REGEX [options]

    Tests REGEX by matching it to the text given in stdin.";

    print!("{}", options.usage(brief));
}

/// Parses command-line options and runs retest.
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut options = Options::new();
    options.optflag("h", "help", "Print this help menu");
    options.optflag("v", "version", "Print the program version");
    options.optopt("f", "file", "Specify a file to match against", "FILE");
    options.optopt("s", "subject", "Specify a subject to match against", "TEXT");
    options.optflag("l", "list", "Print the matches as a list instead of inside the entire subject");
    options.optflag("i", "insensitive", "Case-insensitive matching");
    options.optflag("m", "multiline", "Enable multi-line mode: ^ and $ match begin/end of line");

    let opt_matches = match options.parse(&args[1..]) {
        Ok(matches) => { matches }
        Err(err) => {
            println!("ERROR: {}", err);
            process::exit(1);
        }
    };

    // If the version flag is present, display the version info and exit.
    if opt_matches.opt_present("v") {
        println!("Retest version {}.", VERSION);
        return;
    }

    // If the help flag is present or the user forgot to specify a pattern, show
    // the usage message.
    if opt_matches.opt_present("h") || opt_matches.free.is_empty() {
        print_usage(options);
        return;
    }

    // Get the regular expression pattern from the argument list and add modifiers
    // if given.
    let mut pattern = opt_matches.free[0].clone();
    if opt_matches.opt_present("i") {
        pattern = "(?i)".to_string() + &pattern;
    }
    if opt_matches.opt_present("m") {
        pattern = "(?m)".to_string() + &pattern;
    }

    // Get the subject to test on. If the -s option is present, get the subject
    // from the argument list. If the -f option is present, read the subject from
    // a file. Otherwise get the subject from stdin.
    let mut subject: String;

    if opt_matches.opt_present("s") {
        subject = opt_matches.opt_str("s").unwrap();
    } else {
        subject = String::new();

        if opt_matches.opt_present("f") {
            let mut file = match fs::File::open(opt_matches.opt_str("f").unwrap()) {
                Ok(file) => { file },
                Err(err) => {
                    println!("ERROR: {}", err);
                    process::exit(1);
                }
            };
            file.read_to_string(&mut subject).unwrap();
        } else {
            let mut stdin = io::stdin();
            stdin.read_to_string(&mut subject).unwrap();
        }
    }
    let subject = subject;

    // Find all matches in the subject.
    let matches = match retest::find_matches(&pattern, &subject) {
        Ok(result) => { result },
        Err(err) => {
            println!("ERROR: {}", err);
            process::exit(1);
        }
    };

    // If the -l flag is given, print out the matches as a list instead.
    if opt_matches.opt_present("l") {
        retest::print_match_list(&subject, &matches);
    } else {
        retest::print_subject_highlighted(&subject, &matches);
    }
}
