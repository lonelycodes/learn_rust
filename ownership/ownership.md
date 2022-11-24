# Ownership

A set of rules that govern how rust programs manage memory.
  
There is no garbage collection.
  
There is no manual `malloc()`/`free()`.
  
Rust defines ownership rules that are checked by the compiler.
  
Ownership does nothing at runtime, so no performance hit.
  
This really is what makes rust unique.

## Stack / Heap

Stack: LIFO.
  
Heap: allocator finds spot on heap that's big enough, marks it in use and returns a _pointer_ to it.
  
Pushing to stack faster than allocating on heap. Searching spot is just top of the stack.
  
Calling a function: params are pushed to the stack (even pointers to heap addresses).

## The Rules

* Each value has an owner.
* There can only be ine owner at a time.
* When the owner goes out of scope, the value is dropped.

## Variable Scope

```rust
// ... a not valid, hasn't been declared
{
  let a = "foobar"; // a is valid from here forward
  // ... things with a
} // scope over. from here, a is no longer valid
```

## The String Type

We need `String` because things like `char`, `f32`, `u8`, ... are of fixed size. They are simply pushed/poppet to/from the stack.
  
`String` could be of unknown size at compile time, can't always use string literals.

```rust
let mut s = String::from("hello"); // :: lets us namespace from function under String type
s.push_str(", world!"); // s is now hello, world!


let mut t = "hello";
t.push_str(", world!") // push_str not found for &str
```

## Memory & Allocation

String literal: known at compile-time -> value gets hardcoded in final executable.
  
`String` should be mutable, growable, shrinkable -> unknown size at compile-time. We need:
* Requesting memory from allocator at run-time
* Returning the memory to the allocator when we no longer need the `String`.

```rust
let s = String::from("hello!"); // Request from allocator -- done by us :)
```

Returning the momory often done by GC. But: difficult.
  
Rust just returns the memory when the variable goes out of scope.
  
Special function `drop` called by rust when variable goes out of scope.

### Variables & Data Interacting With Move

Multiple variables can interact with same data.
  
Example using simple integers:

```rust
let a = 1;
let b = a;
```

Note that we make a copy of `a` and store it in `b`.
  
Now with `String`:

```rust
let s = String::from("hello");
let t = s;
```

This did _not_ make a copy!
  
Looking under the hood of `String`:
  
s ptr=1234, len=5, capacity=5
  
t ptr=1234, len=5, capacity=5
  
Then at address 1234 and following we have
* 1234 -> 'h'
* 1235 -> 'e'
* 1236 -> 'l'
* 1237 -> 'l'
* 1238 -> 'o'
  
The data for "hello" is allocated only once, and both `s` and `t` have a pointer to it.
  
So what happpens when _both_ `s` _and_ `t` go out of scope? both try to `drop` -> double free error!
  
To mitigate, after `let t = s`, rust no longer considers `s` to be valid.

```rust
let s = String::from("hello");
let t = s;

println!("{s}"); // compiler error: borrow of moved value s
```

Similar to shallow copy in other languages with addition of invalidating `s`.
  
Design choice: rust will never automatically create az deep copy of data -> automatic copies can be assumed to be fast.

### Variables & Data Interacting With Clone
























































