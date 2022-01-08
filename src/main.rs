use clap::{App, Arg};
use regex::Regex;

fn main() {
    let args = App::new("gremlin")
        .version("0.1")
        .about("Grep-like searcher for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("Pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // .unwrap(), unwraps a Result (Some, None)
    let pattern = args.value_of("pattern").unwrap();
    let regxr = Regex::new(pattern).unwrap();

    let quote = "\
      Every face, every shop, bedroom window, public-house, and 
      dark square is a picture feverishly turned--in search of what?
      It is the same with books.
      What do we seek through millions of pages?";

    for line in quote.lines() {
        match regxr.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
