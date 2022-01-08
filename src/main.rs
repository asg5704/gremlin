use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args = App::new("gremlin")
        .version("0.1")
        .about("Grep-like searcher for patterns")
        .arg(
            Arg::new("pattern")
                .help("Pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("input")
                .help("File to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // .unwrap(), unwraps a Result (Some, None)
    let pattern = args.value_of("pattern").unwrap();
    let file_name = args.value_of("input").unwrap();
    let regxr = Regex::new(pattern).unwrap();
    let search_file = File::open(file_name).unwrap();
    let reader = BufReader::new(search_file);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        match regxr.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
