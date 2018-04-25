extern crate regex;

use regex::Regex;

fn main() {
    let r = Regex::new("world").unwrap();
    println!("{}", r.is_match("Hello, world!"));
}
