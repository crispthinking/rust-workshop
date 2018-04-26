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

## Source Code

The final source code for this exercise can be found [next to this
file on
GitHub](./).
