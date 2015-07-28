extern crate getopts;
extern crate regex;
extern crate term;
mod retest;

use getopts::Options;
use std::env;
use std::io;
use std::io::Read;

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
    options.optopt("s", "subject", "Specify a subject to match against", "TEXT");

    let matches = match options.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // If the help flag is present or the user forgot to specify a pattern, show
    // the usage message.
    if matches.opt_present("h") || matches.free.is_empty() {
        print_usage(options);
        return;
    }

    // Get the regular expression pattern from the argument list.
    let pattern = matches.free[0].clone();

    // Get the subject to test on. If the -s option is present, get the subject
    // from the argument list, otherwise get the subject from stdin.
    // Get the input text from stdin.
    let mut subject: String;

    if matches.opt_present("s") {
        subject = matches.opt_str("s").unwrap();
    } else {
        subject = String::new();
        let mut stdin = io::stdin();
        stdin.read_to_string(&mut subject).unwrap();
    }

    // Print out the highlighted subject.
    retest::print_matches(&pattern, &subject);
}
