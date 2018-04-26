mod token;
mod parser;

fn main() {
    let tokens = parser::parse("hello 123 world!");
    println!("Parsed: {:?}", tokens);
}
