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
