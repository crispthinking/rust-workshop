# Fireflowers and Unsafe

There are some things which can't be achieved within the restrictions
of safe Rust. Rust tends to take a pragmatic approach here. If you
want to allow multiple borrows to share nodes in a tree for example
and, every now and again, mutate one you can't. If you want to build a
doubly linked-list you can't. What would the lifetimes of the borrows
be for the next and previous links.

```
struct ListNode<T, 'a> {
	data: T,
	next: &'a ListNode<T, 'a>,
	prev: &'a ListNode<T, 'a>,
}
```

## Breaking Out

There are many techniques to work around these problems, many of them
safe Rust. If that fails however you can break out the `unsafe`
keyword. Unsafe rust has the same syntax as safe Rust, but it's up to
you as the programmer to make sure the safety guarantees are
upheld. You can use *raw pointers*, `null`, break ownership and
lifetime guarantees, perform casts, and call out to C APIs.

```
struct ListNode<T> {
	data: T,
	next: *mut ListNode<T>,
	prev: *mut ListNode<T>,
}
```

The idiom in rust is to use `unsafe` as sparingly as possible and
build simple safe abstractions with it. There are many useful things
in the standard library which are built with `unsafe` so you don't
have to use it:

 * `RefCell` so you can share mutable state (although only one thing
   can actually mutate it at once.)
 * `Mutex` so you can share data between threads


## Become Fire Mario

Rust has an impressive ecosystem. There are thousands of published
packages, or crates, produced by the community of rust enthusiasts; or
Rustaceans as we like to call ourselves. Many people who use Rust end
up raving about it. It's easy to think that people like Rust because
of "memory safety without GC", the awesome iterator adapter APIs, or
the cute little Ferris mascot.

These are just the tools though. The metaphore used in the Rust
community is that as a programmer you are Mario. Rust is a Fire
Flower. When you use Rust you become Fire Mario. You're bigger, better,
and can shoot fireballs. Awesome right? You shouldn't use rust because
of any one of its features. You should use rust because it makes you
Fire Mario.
