extern crate regex;

use regex::*;
use std::io::{self, BufRead};
use std::iter;

fn main() {
    let badwords = ["c", r"c\++", "ada"];
    let regex_set = RegexSet::new(&badwords).unwrap();
    let regexes = badwords.iter()
        .map(|w| Regex::new(w))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    println!("Created set: {:?}", regex_set);

    let stdin = io::stdin();
    let ok_lines = stdin.lock()
        .lines()
        .filter_map(|l| l.ok());

    for mut line in ok_lines {
        let match_info = regex_set.matches(&line);
        if match_info.matched_any() {
            for m in match_info.into_iter() {
                line = regexes[m].replace(&line, |caps: &Captures| {
                    iter::repeat('*').take(caps[0].len()).collect()
                }).to_string();
            }
        }
        println!("{}", line);
    }
}
