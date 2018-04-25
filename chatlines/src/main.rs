extern crate regex;

use regex::*;
use std::io::{self, BufRead};
use std::iter;

fn main() {
    let badwords = ["c", r"c\++", "ada"];
    let regex_set = RegexSet::new(&badwords).unwrap();

    println!("Created set: {:?}", regex_set);

    let stdin = io::stdin();
    let ok_lines = stdin.lock()
        .lines()
        .filter_map(|l| l.ok());

    for line in ok_lines {
        if regex_set.is_match(&line) {
            println!(
                "{}",
                iter::repeat('*')
                    .take(line.len())
                    .collect::<String>()
            );
        } else {
            println!("{}", line);
        }
    }
}
