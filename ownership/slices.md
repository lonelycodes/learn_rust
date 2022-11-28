# Slices

Let you access a contiguous sequence of elements in a collection instead of the whole collection.
  
A slice is a kind of reference, so it does not have ownership of the data.
  
Illustrative problem: Find first word (separated by spaces) in a string.

```rust
fn first_word(s: & String) -> ???
```

Could just return the index to the end of the first word (as `usize`):

```rust
fn first_word(s: & String) -> usize {
  let bytes = s.as_bytes(); // get an array of bytes
  for (i, &item) in bytes.iter().enumerate() { // create iterator and return tuples of index/ref to element
  if item == b' ' {
    return i;
  }
  s.len()
}
```

Problem here: we return a `usize` that only makes sense in the context of the `String` we put in.
  
For example the following code makes no sense now:

```rust
fn main() {
  let s = String::from("Hello world");
  let first = first_word(&s); // returns 5
  s.clear(); // first is no longer at 5 but at 0
}
```

## String Slices

A string slice is a reference to a _part_ of a string and looks like this:

```rust
let s = String::from("hello, world");
let hello = &s[0..5];
let world = &s[6..11];
```

