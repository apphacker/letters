#[macro_use]
extern crate clap;
extern crate term_painter;

mod display;
mod log;
mod letter;

use clap::App;
use log::*;

#[derive(Debug)]
pub struct Options {
    verbose: bool,
    very_verbose: bool,
    colors: bool,
}

fn main() {
    println!("Hello, world!");
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let options = Options {
        verbose: matches.is_present("verbose"),
        very_verbose: matches.occurrences_of("verbose") > 1,
        colors: matches.is_present("colors"),
    };


    // Unwrap in pattern is safe, clap guarantees it.
    let letters = match matches.value_of("letters") {
        Some(p) => p,
        _ => {
            v(format!("Found nothing to do with options: {:?}", options), &options);
            return;
        },
    };
    v(format!("Printing letters with options: {:?}", options), &options);
    display::letters(letters, &options);
}
