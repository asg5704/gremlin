use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = App::new("gremlin")
        .version("0.1.0")
        .about("👹 A grep-like pattern searcher")
        .author("Alexander Garcia <https://alexandergarcia.me/contact>")
        .arg(
            Arg::new("pattern")
                .help("Pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("input")
                .help("File to search [optional]")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    // Creates input if file passed or default stdin
    let input = args.value_of("input").unwrap_or("-");

    // Creates the pattern for regexr
    let pattern = args.value_of("pattern").unwrap();

    // Creates new Regex from pattern
    let regxr = Regex::new(pattern).unwrap();

    // Determines if file_name passed or stdin
    if input != "-" {
        // Creates search_file if file passed
        let search_file = File::open(input).unwrap();

        // Creates the BufferReader for search term
        let reader = BufReader::new(search_file);

        process_lines(reader, regxr);
    } else {
        // Takes input from stdin
        let stdin = io::stdin();

        // Creates a reader from stdin
        let reader = stdin.lock();

        process_lines(reader, regxr);
    }
}
