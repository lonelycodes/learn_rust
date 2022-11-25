# References & Bolrrowing

Remember the problem: When we pass a value to a function (aka move it into it), we need to move it back (aka return it) if we want to use it again.
  
If we don't want to return it, we can pass a _reference_ to the variable to the function.
  
Like a pointer, references are addresses we can follow to access data (which is owned by some other variable).
  
Unlike a pointer, references are guaranteed to point to a _valid_ value of particular type for the whole life of the reference. Aka no nullPtr BS.

```rust
fn main() {
  let s_hello = String::from("hello");
  let l = do_stuff(&s);
  println("{s_hello}: {l}");
}

fn do_stuff(s: &String) -> usize {
  s.len()
}
```

We have s -> s_hello -> data.
  
The `&` allows us to reference a value without taking ownership of it.
  
Creating a reference to a value is called _borrowing_ (because we don't take ownership).
  
What happens when we try to modify borrowed values?

```rust
fn main() {
  let s= String::from("hello");
  do_stuff(&s);
  println("{s}");
}

fn do_stuff(s: &String) {
  s.push_str(", world!") // compiler error: cannot borrow s as mutable
}
```

## Mutable References

We can alwo get a _mutable reference_ to a borrowed value like so:

```rust
fn main() {
  let s= String::from("hello");
  do_stuff(&mut s);
  println("{s}");
}

fn do_stuff(s: &mut String) {
  s.push_str(", world!") // no problem
}
```

Restriction: if you have a mutable reference to a variable, no other references to that variable must exist.

```rust
let mut s = String::from("hello");

let t = &mut s;
let u = &mut s; // compile error: can't borrow s as mutable more than once
println!("{s}, {t}, {u}");
```

This means we can mutate borrowed values, but not always.
  
This means yhat rust can prevent _data races_ at compile-time!
  
Data race occurs when the following are true:
* 2 (or more) pointers access the same data at the same time
* At least one of those pointers writes to the data
* There's no sync-mechanism to sync access to the data
  
There's a similar rule for mutable and immutable references:

```rust
let mut s = String::from("hello");

let t = &s; // no problem
let u = &s; // no problem
let v = &mut s; // compile-error: can't borrow s as mutable because it's also borrowed asa immutable
```

Note: a reference's scope starts where it's introduced and ends where it is lasr used. So this works:

```rust
let mut s = String::from("hello");
let t = &s;
let u = &s;

println!("{t}, {u}"); // last use of references to s in t and u

let v = &mut s; // no problem
```

## Dangling References

Dangling pointer = pointer to memory that has been given to someone else.
  
Freeing memory but keeping the pointer to that memory.
  
In rust, the compiler guarantees that data never goes out of scope before the reference to it goes out of scope.
  
I.e., no dangling references. Example:

```rust
fn main() {
  let ref_to_nothing = dangle();
}

fn dangle() -> &String { // compile error: expected lifetime operator
  let s = String::from("hello");
  &s
}
```

Rust even says

> this function's return type contains a borrowed value, but there is no value
> for it to be borrowed from



























