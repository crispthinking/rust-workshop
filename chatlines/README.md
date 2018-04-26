# Filtering User Input with `RegexSet`

In this tutorial we will be creating a program which uses the
`RegexSet` to filter lines of text and remove 'bad' words. The
`RegexSet` provides an optimised way of finding which of a large group
of expressions match in a given piece of text.

## Using this Tutorial

In this tutorial I'll walk you through creating a simple program that
uses regular expressions using the rust toolchain. In this tutorial `$
foo` represents commands which should be typed at the command line. To
follow the tutorial you should make sure you have access to:

 * A terminal or console from which you can run `cargo` and `rustc`.
 * A text editor (Visual Studio Code is recommended if you have to choose)

# Creating a Project

Let's begin by creating a new empty console application with
`cargo`. Open your terminal and run:

```
$ cargo new --bin chatlines
```

This should create a new directory called `chatlines/` with a layout
similar to this:

```
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

Change into that directory and make sure everything is working by running:

```
$ cargo run
```

If all goes well you should see "Hello, World!" printed out.

## Adding the Regex Crate

The [`regex` crate][regex-crate], or library, is available on
<https://crates.io>. To use it we must first add a reference to our
`Cargo.toml`. Open up the file in a text editor and add the following
to the `dependencies` section:

```toml
[dependencies]
regex = "0.2"
```

This tells `cargo` to find the latest 0.2.x version of `regex` and
make it available when compiling. To check that's working update
`src/main.rs` and replace the contents with the following simple
program:

```rust
extern crate regex;

use regex::Regex;

fn main() {
    let r = Regex::new("world").unwrap();
    println!("{}", r.is_match("Hello, world!"));
}
```

There's a few interesting things going on here; let's go through it
line-by-line.

The `extern crate` declaration pulls the contents of the `regex` crate
into scope and allows code to reference it. The following `use`
declaration pulls the `Regex` type out of the `regex` module into the
current scope so that code can access it without using the full path
(`::regex::Regex`).

In the body of the `main` method we first create a new `Regex`
instance. Seen as this can fail the `Regex::new` method returns a
`Result`. Here we use `unwrap` on the result to directly access the
`Ok` part of it, or `panic!` and exit the program if there was an
error. Although `Option` and `Result` allow you to handle failure
properly sometimes when prototyping code like this it is useful not to
have to worry about the error cases. As well as `Result` there is also
an `unwrap` on `Option`.

Now we have the `Regex` instance the last line checks if the pattern
matches a given string and prints out the result.

Go ahead and run this with `$ cargo run`. you should see something
similar to the following at your console:

```
   Compiling chatlines v0.1.0 (file:///Users/willspeak/Repositories/rust-workshop/chatlines)
    Finished dev [unoptimized + debuginfo] target(s) in 1.10 secs
     Running `target/debug/chatlines`
true
```

## Creating the `RegexSet`

For this example we will be filtering out the competing programming languages: `C`, `C++`, and `Ada`. Replace the body of the `main` method with the following declaration:

```rust
    let badwords = ["c", r"c\++", "ada"];
    let regex_set = RegexSet::new(&badwords).unwrap();

    println!("Created set: {:?}", regex_set);
```

Because we want to use the `RegexSet` type we need to update our `use` line too. We could switch it to `use regex::RegexSet`, however we will be using quite a bit from this module. Instead we can be lazy and replace it with:

```rust
use regex::*;
```

To pull all the public API of the `regex` crate into scope.

Go ahead and run the program again with `$ cargo run`. You should now see your `RegexSet` printed out to the console. Here we've use the `:?` print format. This uses the `Debug` representation of the object. Not all objects can be printed directly. If you see the compiler complaining that `X does not implement Display` try replacing the format with `:?`.

## Silencing Lines

Now we have our regex set let's use it! For this we are going to need to read lines from standard input. Lets begin by updating the `use` lines to the beginning of the file:

```rust
use regex::*;
use std::io::{self, BufRead};
use std::iter;
```

We will be using the `std::io` module to read lines from the console, and the `std::iter` module for some useful iterator extensions. Next we'll add a loop to the bottom of the `main` method:

```rust
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
```

Here we lock the standard input stream, so that we can read from it, and loop over each line that can be read. Inside the loop we check to see if any of the regular expressions in our `regex_set` match, and if so we print out some `*` characters in place of the real line. Note the use of the iterator adapters [`filter_map`] to check for lines which were read successfully; and [`take`] and [`collect`] to convert an endless repeating iterator of `*` characters into the string to print out.

To find out more about these functions you can visit <https://doc.rust-lang.org/> and click on "API Documentation" for information about the standard library or head to <https://docs.rs> to search third-party crate documentation.

Go ahead and try this out by running `$ cargo run` and type some text in. Each time you press enter the line should be echoed back to you. Try typing lines which do and don't contain bad words.

## Replacing just the Bad Words

Filtering out the entire line if someone uses a bad word seems a bit over the top. Lets update our program so that only the bad words themselves are removed.

First, we will need a regex for each pattern in `badwords`. Add the following just after your `regex_set` is created:

```rust
let regexes = badwords.iter()
    .map(|w| Regex::new(w))
    .collect::<Result<Vec<_>, _>>()
    .unwrap();
```

Replace the for loop with the following:

```rust
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
```

This switches from using `is_match`, which checks if a pattern matches anywhere to `matches`. The `matches` method returns more metadata about the match, including which patterns matched. We then loop through each pattern and replace the match using `Regex::replace`. The replacement function can accept anything which implements the [`Replacer`] trait. In this case we've given it a closure which creates a string of `*` characters. You can also use a plain string too or write your own replacer type if needed.

Go ahead and run the program again. Try typing in some text and check that just the `badwords` are being filtered.

That's it! You've created a chat filtering application using Rust! Now all you need to do to compete with Crisp is to make it scale...

## Source Code

The final source code for this exercise can be found [next to this file on GitHub](./).

## Extra Credit

What happens if you have more than one match for a given pattern in the string?

Try creating a custom [`Replacer`] that creates the `*` substitutions. Maybe make the replacement character configurable.

 [regex-crate]: https://crates.io/crates/regex
 [`filter_map`]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.filter_map
 [`collect`]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.collect
 [`take`]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.take
 [`Replacer`]: https://docs.rs/regex/0.2.10/regex/trait.Replacer.html
