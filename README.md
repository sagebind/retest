# retest
Retest is a regular expression tester that can be used in the terminal. It is meant to be a simple and quick utility to aid you in developing regular expressions. Like [regular expressions 101](https://regex101.com), but in the terminal.

Would you like to suggest new features? [Make an issue on GitHub](https://github.com/coderstephen/retest/issues/new) describing the feature, or [shoot me an email](mailto:me@stephencoakley.com) to ask me directly.

## Install from release
Binaries for Linux are provided for each release and available on the [releases page](https://github.com/coderstephen/retest/releases). Just download a "retest" binary and place it somewhere that is in your $PATH (like `/usr/local/bin`), and you're ready to go.

## Compiling and installing from source
First, get the source code by cloning from GitHub:

    $ git clone https://github.com/coderstephen/retest.git
    $ cd retest

You can install dependencies and compile all at once using `cargo`:

    $ cargo build

A simple `Makefile` is also provided for easily installing and uninstalling retest. To install globally, you can follow the traditional Linux steps:

    $ make
    $ sudo make install

This will compile retest and place a globally available executable into `/usr/local/bin`. Similarly, you can uninstall retest with

    $ sudo make uninstall

## Usage
Retest accepts one regular expression as an argument, and then matches it against a subject string. The subject will be printed back out, with all matches found highlighted. For example:

    $ echo 'fee fi fo fum' | retest 'f\w\w'

will output something like the following (text in square brackets would be highlighted):

    [fee] fi fo [fum]

By default, the subject is read from standard input. If you'd like to specify the subject as an argument, you can use the `--subject` (or `-s`) option:

    $ retest '\w+\s' --subject 'fee fi fo fum'

For more options and tricks, check the help message from the program:

    $ retest --help

## License
All documentation and source code is licensed under the Apache License, Version 2.0 (Apache-2.0). See the [LICENSE](LICENSE) file for details.
