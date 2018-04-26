#[derive(Debug)]
pub enum Token {
    Word(String),
    Number(u32),
    Punct(String),
}
