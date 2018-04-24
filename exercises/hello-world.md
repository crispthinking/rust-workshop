# Hello World in Rust

This tutorial is intended to help get you off the ground and
programming in Rust. It demonstrates simple console output in Rust,
and walks you through installing the Rust toolchain.

## Using this Tutorial

In this tutorial you will learn how to create a simple program using
the Rust toolchain. Throughout I will use $ foo to represent commands
which should be from the command line. To follow this tutorial you
should make sure you have access to:

 * A terminal or Console
 * A text editor
 * A web browser

You should be able to follow this tutorial on Windows, macOS, and Linux.

## Greetings

Lets get started by running some Rust code. The easiest way to do this
is to head to <https://play.rust-lang.org>. Here you can interactively
edit and run Rust right from your browser. Navigate there now and you
should be greeted by a text editor containing the following simple
rust program:

```
fn main() {
    println!("Hello, world!");
}
```

You can run this code by clicking "Run..." in the upper left
corner. You should be greeted with the following output:

    Hello, World!

Try changing the text that is printed to prove that this code really
is being compiled and run.

## Getting Functional

Just printing "Hello, World!" Isn't that interesting. It's been done
before in many other languages. Let's make it a bit more
Rusty. Instead of greeting world we can abstract away the target of
the greeting. Add the following function definition before `main`:

```
fn greet(greetee: &str) {
    println!("Hello, {}!", greetee);
}
```

And update `main` to call `greet`:

```
fn main() {
    greet("World");
}
```

If you run this agin you should see `Hello, World!` printed.

## Adding some Types

Just being able to greet strings doesn't seem like enought
though. Let's add a `Person` type which can be greeted too. Add the
following at the top:

```
struct Person {
    first: String,
    last: String,
}
```

This defines a structure to hold the information about a person. In
this example a `Person` has a `first` and `last` name. Both of these
are owned strings. Nex we'll add an `impl` block to define some
methods for `Person`. Below the `Person` declaration add:

```
impl Person {
    pub fn new(first: &str, last: &str) -> Self {
        Person {
            first: first.to_string(),
            last: last.to_string(),
        }
    }
    
    pub fn name(&self) -> String {
        format!("{} {}", self.first, self.last)
    }
}
```

This defines two methods, `new` and `name`. By convention `new` is a
static constructor and is used to create instances. It returns the
type `Self`, which is the type that the `impl` block is for.

To make an instance method `name` takes `&self` as the first
parameter. Instance methods can also be defined to take ownership of
self (`self`) and to recieve a mutable reference `&mut self`. These
are just syntax sugar for `self: &Self`, `self: Self`, and `self: &mut
Self` respectively.

Next up let's create a instance of a `Person` and `greet` them. Add
the following at the end of `main`:

```
let me = Person::new("<your>", "<name>");
greet(&me.name());
```

Now run the program again and you should see output something similar to this:

    Hello, World!
	Hello, Will Speak!

Note that we called the static method `new` with `::new`, and that we
have to add a `&` in the call to `greet` to turn the owned string it
returns into a borrow as `greet` expects. Try removing the `&` and see
how the compiler complains. Rust errors generally contain a lot of
information, often including suggestions of how to "fix" them. If you
want more information about the erorr try clickign the error number to
see more details.

