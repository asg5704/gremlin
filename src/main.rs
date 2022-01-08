use regex::Regex;

fn main() {
  // .unwrap(), unwraps a Result (Some, None)
  let regxr = Regex::new("picture").unwrap();
  let quote = "\
      Every face, every shop, bedroom window, public-house, and 
      dark square is a picture feverishly turned--in search of what?
      It is the same with books.
      What do we seek through millions of pages?";
  
  for line in quote.lines() {
    let contains_substr = regxr.find(line);
    match contains_substr {
      Some(_) => println!("{}", line),
      None => ()
    }
  }
}
