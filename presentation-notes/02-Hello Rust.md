# Hello Rust

[TOC]

So, What is Rust?

## Systems Programming

Rust is a "systems programming" language. But what does that mean?
It's a bit of a fuzzy term which programming langauge authors use to
mean that their lanaguage is 'low level' in some way.

In rust "systems programming" means that you can run Rust on an
embedded microcontroller, you can use Rust to build a browser, you can
use rust to create a command line tool. It means you can use Rust
where your only choices otherwise might have been C or C++.

## Safe, Concurrent, Fast - pick three!

Rust's semi-official motto is "safe, concurrent, fast - pick
three". This summs up the ethos behind rust quite well. Rust aims to
be a powerful, modern, high-level programming language. It is built
around the core idea of providing 'safety' and using it to enable
users to do more not less.

In Rust "safety" means memory safety, data-race safety and type
safety. Rust uses these as building blocks to enable high-leve code to
compile down to fast machine code. You can write complex chains of
fluent iterator adapters and, the compiler is able to optimise it down
to the same fast & efficent machine code that a hand-optimised
`for`-loop would produce. This is because some of the safety
guarantees built into Rust's type and ownership system give the
compiler far more information to perform optimisations with than
traditional C or C++ compilers have.

## Hello Rust

So, what does rust look like?

```
fn main() {
    println!("Hello, world!");
}
```

This is the canonical "Hello world" example written in rust. It shows
off a few of the features of the language straight away. Rust code
favours shorter keywords like `fn`, `impl` and `let` over longer ones;
the language is block-scoped with blocks delimited by curly braces;
and `println!` is a macro. Macros are often used in Rust to make complex
or repetitive code simpler to understand and manage. Don't worry
though, macros in Rust are hygienic and nothing like the macros you
might have experience with in C-land.

## Ownership

The core safety construct in Rust is ownership. Let's say you have
this code which allocates a new book. I can do things with the book
because I own it.

```
let book = Book::new("Excession");
println!("My book: {}", book);
```

If we create a second variable we can transfer ownership with the
assignment operator:

```
let my_book = Book::new("Excession");
let your_book = my_book;
println!("Your book: {}", your_book);
// !!ERROR - This won't work.
println!("My book: {}", my_book);
```

[Playground](https://play.rust-lang.org/?gist=75efc5c72f1fa97864e17aef192f10ff&version=stable)

```
error[E0382]: use of moved value: `my_book`
 --> src/main.rs:5:31
  |
3 |     let your_book = my_book;
  |         --------- value moved here
4 |     println!("Your book: {:?}", your_book);
5 |     println!("My book: {:?}", my_book); // !!ERROR - This won't work.
  |                               ^^^^^^^ value used here after move
  |
  = note: move occurs because `my_book` has type `Book`, which does not implement the `Copy` trait
```

Once ownership is transferred from one variable to another the
original variable can no longer be used to access the value. A value
can only have a single owner at once. This is referred to as *move
semantics* because assignment _moves_ values.

The compiler tries to help us here by showing us where we move the
value, and suggesting that if the value was `Copy`able we wouldn't
encounter the error.

## Mutation

Values, by default, are read only.

```
let book = Book::new("Matter");
// !!ERROR - `book` isn't mutable
book.next_page();
```

[Playground](https://play.rust-lang.org/?gist=c8b9fd08cdf7b368912b3ca82a066670&version=stable)

In the previous examples it wouln't be possible to change the `Book`
after creation. Lets say we have a function `next_page` which updates
the page number in our book object.

```
error[E0596]: cannot borrow immutable local variable `book` as mutable
 --> src/main.rs:3:5
  |
2 |     let book = Book::new("Matter");
  |         ---- consider changing this to `mut book`
3 |     book.next_page(); // !!ERROR - `book` isn't mutable
  |     ^^^^ cannot borrow mutably
```

The compiler isn't happy about that.

```
let mut book = Book::new("Matter");
book.next_page();
```

To solve this we can use `let mut` bindings instead of `let`
bindings. By modelling mutablity this way Rust ensures that at most
one place could be mutating data at once. It also means that if data
could be mutated nothing else can be reading from it.

## Borrowing

Only allowing a single variable to own a value at once is quite
restrictve. Rust's solution to this is *borrowing*.

Borrows in rust are like pointers or references that have been souped
up a bit.

A borrow is introduced with the ampersand character.

```
let my_book = Book::new("Inversions");
let my_borrow = &my_book;
println!("Borrowed book: {:?}", my_borrow);
```

[Playground](https://play.rust-lang.org/?gist=dac1e2fdf69c5e13f3b7d16e3a04c098&version=stable)

The compiler makes sure ahead of time that no borrow lives longer than
the value it references. This means you should never find yourself
with a pointer to garbage memory. You can't accidentally return a
reference to a location on a stack, or free memory that others still
have a reference to.

```
let my_borrow: &Book;
{
    let my_book = Book::new("Inversions");
    my_borrow = &my_book;
}
println!("Borrowed book: {:?}", my_borrow);

```

[Playground](https://play.rust-lang.org/?gist=c8c200361fee7ae99c0918dcac4377b1&version=stable)

```
error[E0597]: `my_book` does not live long enough
 --> src/main.rs:5:22
  |
5 |         my_borrow = &my_book;
  |                      ^^^^^^^ borrowed value does not live long enough
6 |     }
  |     - `my_book` dropped here while still borrowed
7 |     println!("Borrowed book: {:?}", my_borrow);
8 | }
  | - borrowed value needs to live until here
```

Here the compiler tells us that `my_book` goes out of scope before
`my_borrow` and prevents us assigning a reference to the book into the
borrow. In rust when an object goes out of scope, or is "dropped" in
rust-lingo, the resources associated with it are recoved. This is
similar to RAII in modern C++.

```
let mut my_book = Book::new(“Inversions");
let my_borrow: &mut Book = &my_book;
my_book.next_page();
```

In addition to a standard borrow you can also borrow mutably. Once
again the compiler will ensure that only a single mutable borrow to a
value exists at once.
