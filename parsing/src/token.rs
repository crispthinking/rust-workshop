/// Token Types
#[derive(Debug)]
pub enum Token {
    /// A word
    ///
    /// Made up of consecutive alphabetic characters
    Word(String),

    /// A numeric value
    Number(u32),

    /// Punctuation
    ///
    /// Anything that's not alphanumeric
    Punct(String),
}
