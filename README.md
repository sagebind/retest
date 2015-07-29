# retest
Retest is a regular expression tester that can be used in the terminal. It is meant to be a simple and quick utility.

## Compiling
You can install dependencies and compile all at once using `cargo`:

    $ cargo build

## Installing
A simple `Makefile` is provided for easily installing and uninstalling retest. To install, run

    $ sudo make install

This will complile retest and place a globally available executable into `/usr/local/bin`. Similarly, you can uninstall retest with

    $ sudo make uninstall


## Usage
Retest accepts one regular expression as an argument, and then matches it against a subject string. The subject will be printed back out, with all matches found highlighted. For example:

    $ echo 'fee fi fo fum' | retest '\w+\s'

By default, the subject is read from standard input. If you'd like to specify the subject as an argument, you can use the `--subject` (or `-s`) option:

    $ retest '\w+\s' --subject 'fee fi fo fum'

For more details, check the help message from the program:

    $ retest --help
