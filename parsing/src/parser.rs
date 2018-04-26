use token::Token;

use self::ParserState::*;

/// Current Parser State
enum ParserState {
    /// At the boundary between two tokens
    Boundary,
    /// Parsing Word characters
    InWord(String),
    /// Parsing Number
    InNumber(u32),
    /// Parsing Punctuation
    InPunctuation(String),
}

/// Next From Token Boundary
///
/// Returns the next parser state for a given character if there is no
/// context. This is used when a token has just been emitted to find
/// the next state without any context.
fn next_from_boundary(c: char) -> ParserState {
    match c {
        c if c.is_alphabetic() => InWord(c.to_string()),
        '0'...'9' => InNumber(c.to_digit(10).unwrap()),
        _ => InPunctuation(c.to_string())
    } 
}

pub fn parse(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut state = ParserState::Boundary;

    for c in input.chars() {

        state = match state {
            Boundary => next_from_boundary(c),
            InWord(mut word) => match c {
                c if c.is_alphabetic() => {
                    word.push(c);
                    InWord(word)
                },
                _ => {
                    tokens.push(Token::Word(word));
                    next_from_boundary(c)
                }
            },
            InNumber(num) => match c {
                '0'...'9' => {
                    InNumber((num * 10) + c.to_digit(10).unwrap())
                },
                _ => {
                    tokens.push(Token::Number(num));
                    next_from_boundary(c)
                }
            },
            InPunctuation(mut punct) => match c {
                c if c.is_alphanumeric() => {
                    tokens.push(Token::Punct(punct));
                    next_from_boundary(c)
                },
                c => {
                    punct.push(c);
                    InPunctuation(punct)
                }
            },
        };
    }

    tokens
}
