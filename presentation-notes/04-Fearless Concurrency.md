# Fearless Concurrency

The same kinds of features that allow you to build safe
single-threaded code also provide the building blocks for creating
safe threaded code.

## Mutex Owns You

In rust a Mutex contains a value, but that value can't be accessed
directly instead you are forced to lock the mutext. When the lock goes
out of scope you no longer have access to the value.

```
let mutex = Mutex::new(10);

{
    let lock = mutex.lock();
    match lock {
        Ok(mut guard) => {
            *guard += 1;
            println!("Value: {0}", guard);
        }
        Err(_) => println!("Mutex poisoned")
    }
}

println!("Final value: {:?}", mutex.into_inner());
```

[Playground](https://play.rust-lang.org/?gist=06612c9159235c8f797ba0ef807a0419&version=stable)

## Rayon

A lot of rust code uses iterator adapters like `map`, `filter` and so
on. Because of the strict guarantees about what the lambdas passed to
each operation can do making code mukli-threaded is often as simple as
changing `iter()` to `par_iter()` with Rayon.

```
fn sum_of_squares(input: &[i32]) -> i32 {
    input.iter()
         .map(|&i| i * i)
         .sum()
}
```

```
use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter() // <-- just change that!
         .map(|&i| i * i)
         .sum()
}
```
