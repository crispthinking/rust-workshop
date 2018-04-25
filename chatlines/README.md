# Filtering User Input with `RegexSet`

In this tutorial we will be creating a program which uses the
`RegexSet` to filter lines of text and remove 'bad' words. The
`RegexSet` provides an optimised way of finding which of a large group
of expressions match in a given piece of text.

## Using this Tutorial

In this tuturial I'll walk you through creating a simple program that
uses regular expressions using the rust toolchain. In this tutorial `$
foo` represents commands which should be typed at the command line. To
follow the tutorial you should make sure you have access to:

 * A terminal or console from which you can run `cargo` and `rustc`.
 * A text editor (Visual Studio Code is recommemnded if you have to choose)

# Creating a Project

Let's begin by creating a new empty console applicaiton with
`cargo`. Open your terminal and run:

```
$ cargo new --bin chatlines
```

This shoudl create a new directory called `chatlines/` with a layout
similar to this:

```
.
├── #README.md#
├── Cargo.toml
└── src
    └── main.rs

1 directory, 3 files
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

The `extern crate` delcaration pulls the contents of the `regex` crate
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
have to worry about the erorr cases. As well as `Result` there is also
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

 [regex-crate]: https://crates.io/crates/regex
