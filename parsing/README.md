# Simple Parsing in Rust

This tutorial demonstrates creating a simple parser in Rust. It
introduces using modules for organising code and shows how to create a
simple state machine using `enum`s.

## Using this Tutorial

In this tutorial you will learn how to create state machines Rust
toolchain. Throughout I will use $ foo to represent commands which
should be from the command line. To follow this tutorial you should
make sure you have access to:

 * A terminal or Console. This should be the *Developer Command Prompt
   for VS* on Windows.
 * A text editor

You should be able to follow this tutorial on Windows, macOS, and Linux.

## Creating a Project

From a command line run `$ cargo new --bin parsing`. This should
create a new folder called `parsing/` with an empty rust project
inside. Change into this folder and check everything is working with
`$ cargo build`.

## Adding Modules

We'll begin by creating the outline of the project. This will consist
of three modules:

 * The main module, which contains the `main` method
   * A `token` module which defines the `Token` type we will be producing.
   * A `parser` module which defines the code to parse our strings.
   
Begin by replacing the contents of `main.rs` with the following:

```rust
mod token;
mod parser;

fn main() {
    let tokens = parser::parse("hello 123 world!");
    println!("Parsed: {:?}", tokens);
}
```

The first two lines define the submodules. In rust a submodule can be
either a file or a folder. Given the module `token` we could create
either `token.rs` or `token/mod.rs`. For this example we will just use
files. Create a new file called `token.rs` and add the following
definition for the `Token` `enum`:

```rust
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
```

Here we have defined an `enum` with three values defining the
different things will will look for in our strings. Note that comments
beginning with `///` provide documentation for the following item. By
adding the atrribute `#[derive(Debug)]` we are telling the compiler
that we want a standard implementation of the `Debug` trait so we can
print this out to the console with `:?`.

Next create another new file `parser.rs`. To get things working we'll
just define the outline of our parser so we can get things compiling:

```rust
use token::Token;

pub fn parse(input: &str) -> Vec<Token> {
	vec![Token::Number(42)]
}
```

You should now be able to compile and run with `$ cargo run` and see
something similar to the following:

```
   Compiling parsing v0.1.0 (file:///Users/willspeak/Repositories/rust-workshop/parsing)
    Finished dev [unoptimized + debuginfo] target(s) in 1.39 secs
     Running `target/debug/parsing`
Parsed: [Number(42)]
```

## Defining the Parser State

To keep track of where we are when parsing we will define an
`enum`. Add the following to the top of the file, just below the `use`
line:

```rust
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
```

This defines the current state of our state machine. Note how we are
storing values with some states to keep track of where we are when
parsing. The first line brings all of the enum items into scope so
instead of typing `ParserState::Boundary` we can just use `Boundary`.

## Match and More

Next up we will define a helper funcntion which chooses the state when
we first start parsing a token. To avoid duplicaiton it's handy to
pull this code out into a function. Functions without the `pub`
keyword are private to a given module so feel free to use as many of
them as you like!.

Add the following after the `ParserState` declaration:

```rust
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
```

Here we are choosing the next state with a `match` expression. This
demonstrates a few useful kinds of patterns that can be used with
`match`.

The `'0'...'9'` matches on a range of values, inclusive. We're using
it to recognise characters which are base-10 digits.

The first pattern matches any character, but filters some out using a
predicate. This is useful if you can't express your restrictions just
by the shape of the data alone.

The last pattern is a common one to end a `match`. It catches all
other values. Matches in Rust have to be complete, so this is often
needed to handlel default or fallback cases.

## State Machines

Now all that is left is to define the main state machine
transitions. Begin by replacing the body of the `parse` function with
the following skeleton:

```rust

pub fn parse(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut state = ParserState::Boundary;

    for c in input.chars() {

        state = match state {
            // TODO: Add state machine transitions here
		}
    }

    tokens
}

```

This sets up the vector we will use to store tokens in, and
initialises the state to a default value. The for loop iterates
through each character in the input string.

All that's left is to add a `match` arm for each state defining the
transitions. The first one is easy:

```rust
Boundary => next_from_boundary(c),
```

This uses the helper we defined earlier. Onwards to the case when we
are parsing words:

```rust
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
```

If we are looking at another word character we add it to the buffered
up word. Otherwise we emit the token and use our helper function to
find the correct first state. The number case is similar:

```rust
InNumber(num) => match c {
    '0'...'9' => {
        InNumber((num * 10) + c.to_digit(10).unwrap())
    },
    _ => {
        tokens.push(Token::Number(num));
        next_from_boundary(c)
    }
},
```

Again we recognise if we are pointing at another digit and add it to
our buffered value. If not we emit a token and move on.

The last case is similar too, however the challenge is to match the
_inverse_ of the other states. We are able to use `is_alphanumeric` as
it covers the alphabetic characters from words and digits from
numbers.

```rust
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
```

Give it a go. Compile and run again with `$ cargo run` and you should
see something similar to the following:

```
   Compiling parsing v0.1.0 (file:///Users/willspeak/Repositories/rust-workshop/parsing)
    Finished dev [unoptimized + debuginfo] target(s) in 1.39 secs
     Running `target/debug/parsing`
Parsed: [Word("hello"), Punct(" "), Number(123), Punct(" "), Word("world")]
```

## Source Code

The final source code for this exercise can be found [next to this
file on
GitHub](./).

## Extra Credit

 * Update the `main` method to parse strings from standard input rather than a fixed value.
 * Try adding parsing for more token types.
