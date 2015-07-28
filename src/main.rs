extern crate getopts;
use getopts::Options;
use std::env;
use std::io;
use std::io::Read;
mod retest;

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

    let matches = match options.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") || matches.free.is_empty() {
        print_usage(options);
        return;
    }

    let regex = matches.free[0].clone();

    // Get the input text from stdin.
    let mut input = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut input).unwrap();

    // Test
    retest::test(&regex, &input);
}
