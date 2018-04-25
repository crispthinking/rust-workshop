extern crate regex;

use regex::*;

fn main() {
    let badwords = ["c", r"c\++", "ada"];
    let regex_set = RegexSet::new(&badwords).unwrap();

    println!("Created set: {:?}", regex_set);
}
