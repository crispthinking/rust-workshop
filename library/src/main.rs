mod book;

use book::Book;

fn main() {
    let book = Book::new("hello world");

    println!("Normal: {:?}", book);
    println!("Special: {:#?}", book);
}
