# Intro to Types

As well as lifetimes and borrowing another key feature in Rust is the
strong static type system. As well as simple types like integers,
strings, and arrays Rust has tuple types and user-defined `struct`s
and `enum`s.

[TOC]

## Structs

Struct types allow you to group bits of data together. In our book
example we could define the title and page number like this:

```
struct Book {
	title: String,
	page: i32,
}
```

The title of the book is a heap-allocated owned string and the page is
an integer.

## Enums

Enums are a form of discrimiated union. They allow you to store one of
a given set of options. Continuing our library-based theme we could
create an enum to model the operations for a library book:

```
enum BookOperation {
	Inspect,
	Return(Book),
	Lend(String),
}
```

You can only access the values within an enum if you check that it is
of the right type. This is done with a `match` or an `if let`.

```
match operation {
	Return(book) => println!("Returning {}", book.title),
	Lend(title)	=> println!("Lending {}", title),
	_ => ()
}
```

## Null, an Exceptionally Bad Idea

The first place where this type system comes into its own is with
`null`. There is no `null` in safe Rust. Instead values that may have
value, or may not have the type `Option`. Option is an enum with two
variants: `Some` and `None`.

Lets say we wanted to write a function that gets someone's
surname. Not everyone has a last name. In other languages you might
throw an exception or return `null`. In Rust we can use `Option` to
force the person consuming the API to check if there was a last name.

```
pub struct Person {
	forename: String,
	surname: Option<String>,
}

fn print_surname(person: Person) {
	if let Some(surname) = person.surname {
		println!("{}'s surname is {}", person.forename, surname);
	} else {
		println!("{} has no surname");
	}
}
```

## There is no Try

Similar to `null` Rust doesn't have exceptions. Instead functions
return a `Result` which is an `enum` of `Ok` or `Err`. In the same way
that `Option` forces you to check for the existence of something
`Result` forces you to check for an error. The question mark operator
allows erorrs to be chained more easily.

```
pub fn hello_to_file(path: &Path) -> Result<(), Error> {
    let mut file = File::create(&path)?;
    write!(file, "hello world")?;
    Ok(())
}
```

[Playground](https://play.rust-lang.org/?gist=5bcc6d4b28b2f7e1d44c54d8cbedf213&version=stable)

## Further Typing

You can use this to build things like type-safe state machines where
the context is held in the value for each enum variant. Enums can also
be used in place of boolean values. Often bools on their own as
function parameters can be confusing:

```
render_to_path("hello/world/", true);
```

```
render_to_path("hello/world/", PathMode::IgnoreMissing);
render_to_path("hello/world/", PathMode::Create);
```
