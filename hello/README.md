# Hello World in Rust

This tutorial is intended to help get you off the ground and
programming in Rust. It demonstrates simple console output in Rust,
and walks you through installing the Rust toolchain.

## Using this Tutorial

In this tutorial you will learn how to create a simple program using
the Rust toolchain. Throughout I will use $ foo to represent commands
which should be from the command line. To follow this tutorial you
should make sure you have access to:

 * A terminal or Console. This should be the *Developer Command Prompt
   for VS* on Windows.
 * A text editor
 * A web browser

You should be able to follow this tutorial on Windows, macOS, and Linux.

## Greetings

Lets get started by running some Rust code. The easiest way to do this
is to head to <https://play.rust-lang.org>. Here you can interactively
edit and run Rust right from your browser. Navigate there now and you
should be greeted by a text editor containing the following simple
rust program:

```rust
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

```rust
fn greet(greetee: &str) {
    println!("Hello, {}!", greetee);
}
```

And update `main` to call `greet`:

```rust
fn main() {
    greet("World");
}
```

If you run this agin you should see `Hello, World!` printed.

## Adding some Types

Just being able to greet strings doesn't seem like enought
though. Let's add a `Person` type which can be greeted too. Add the
following at the top:

```rust
struct Person {
    first: String,
    last: String,
}
```

This defines a structure to hold the information about a person. In
this example a `Person` has a `first` and `last` name. Both of these
are owned strings. Next we'll add an `impl` block to define some
methods for `Person`. Below the `Person` declaration add:

```rust
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

```rust
let me = Person::new("<your>", "<name>");
greet(&me.name());
```

Now run the program again and you should see output something similar to this:

    Hello, World!
	Hello, Joe Bloggs!

Note that we called the static method `new` with `::new`, and that we
have to add a `&` in the call to `greet` to turn the owned string it
returns into a borrow as `greet` expects. Try removing the `&` and see
how the compiler complains. Rust errors generally contain a lot of
information, often including suggestions of how to "fix" them. If you
want more information about the erorr try clickign the error number to
see more details.

## Adding Traits

Having to explicitly get the `name` from a `Person` instance to greet
them seems a bit of a faff. Let's introduce a trait to encompas what
it means to be "Greetable". Traits in Rust are similar to interfaces
in other languages, however they are more powerful ins some
ways. First add the trait definition to the beginning of our code:

```rust
trait Greetable {
    fn name(&self) -> String;
}
```

Next lets move our `name` method from the `impl Person` block to a new `impl` block:

```rust
impl Greetable for Person {

    fn name(&self) -> String {
        /// ... as before ...
    }
}
```

Note that the function no longer needs the `pub` modifier.

This block must contain a definition for each method in the trait. For
our `Greetable` trait there is just one: `name`.

We can now update our `greet` method to accept any type which is `Greetable`:

```rust
fn greet<T>(greetee: &T)
    where T: Greetable + ?Sized
{
    println!("Hello, {}!", greetee.name());
}
```

Here the type bound on T allows `greet` to accept anything which is
Greetable. The extra `?Sized` is needed to allow us to accept types
like `str` which don't have a known size at compile time. Usually this
isn't something you have to worry about.

As well as `Person` we'll need to implement `Greetable` for `str`:

```rust
impl Greetable for str {
    fn name(&self) -> String {
        self.to_string()
    }
}
```

The last step is to change `main` to greet the `Person` directly:

```rust
fn main() {
    greet("World");
    let me = Person::new("Joe", "Bloggs");
    greet(&me);
}
```

If all goes well you should be able to run the example again and see
the same output:

    Hello, World!
	Hello, Joe Bloggs!

## Installing the Toolchain

Now we have our example greeter working in
<https://play.rust-lang.org> it's time to run it locally. The easiest
way to manage installed Rust toolchains is with Rustup. This is a
command line program which allows you to have mulitple toolchains
installed (stable, beta, web asssembly etc.) and keep them all up to
date. To get starated head to <https://rustup.rs>.

On macOS and Linux copy the command line to a console and run it. On
Windows there is an installer to download and run. If you are
installing on Windows you'll need to make sure you have the [Build
Tolls for Visual Studio][build-tools] installed. Both should end up
installing toolchain and adding it to your path. To check open a new
console and run the following commands:

    $ cargo --version
    cargo 0.26.0 (41480f5cc 2018-02-26)
    $ rustc --version
    rustc 1.25.0 (84203cac6 2018-03-25)

You should see similar output. If you see errors about not finding the
commands make sure it is a new termial window rather than the one you
used to download and install the toolchain. On Windows you'll need to
make sure you're using the *Developer Command Prompt for VS* rather
than a plain old command window.

## Creating a Project with Cargo

Cargo is the rust package manager and build tool. It is what you will
usually interact with when working with Rust. In your console navigate
to a folder where you want to create your project (e.g. `cd
/Users/joe/Repositories`). Cargo has support for scaffolding simple
projects. We'll use this to create a template to get us off the
ground:

    $ cargo new --bin hello

This will create a new folder `hello/` under the current directory
with a layout similar to this:

```
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

First lets check that it compiles. Change into `hello/` and run the following:

```
$ cargo build
   Compiling hello v0.1.0 (file:///Users/joe/Repositories/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 8.62 secs
```

If all goes well you should see similar output. Now lets run the
program. We can either do this directly by running `$
./target/debug/hello` or with `$ cargo run`. The latter will make sure
the compiled program is up to date before running it.

Now it's time to copy our example code from the playpen at
<https://play.rust-lang.org>. Open up `src/main` in your chosen text
editor and replace the contents with your final working code from the
playpen. Save the file and re-run it with `$ cargo run`. You should
see the same output that the playpen displayed in your console.

That's it! If you've been following along you've just written your
first Rust program. You're now a Rustacean.

## Source Code

The final source code for this exercise can be found [next to this
file on
GitHub](./).

## Extra Credit

* Try creating other `Greetable` types, such as `Company` or `Group`.
* Instead of always greeting with "Hello" try making the greeting
  customisable. Maybe create a `Greeter` trait which can greet any
  `Greetable`.

 [build-tools]: https://www.visualstudio.com/downloads/#build-tools-for-visual-studio-2017
