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

        let next = match state {
            Boundary => Some(next_from_boundary(c)),
            InWord(mut word) => match c {
                c if c.is_alphabetic() => {
                    word.push(c);
                    Some(InWord(word))
                },
                _ => {
                    tokens.push(Token::Word(word));
                    None
                }
            },
            InNumber(num) => match c {
                '0'...'9' => {
                    Some(InNumber((num * 10) + c.to_digit(10).unwrap()))
                },
                _ => {
                    tokens.push(Token::Number(num));
                    None
                }
            },
            InPunctuation(mut punct) => match c {
                c if c.is_alphanumeric() => {
                    tokens.push(Token::Punct(punct));
                    None
                },
                c => {
                    punct.push(c);
                    Some(InPunctuation(punct))
                }
            },
        };
        
        if let Some(new_state) = next {
            state = new_state;
        } else {
            state = next_from_boundary(c);
        }
        
    }

    tokens
}
