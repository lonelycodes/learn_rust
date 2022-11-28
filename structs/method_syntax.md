# Method Syntax

Same as functions but defined in context of struct, enum or trait.
  
First parameter to a method is always `self` (instance of the struct we call on).

## Defining Methods

Defining area _on_ rectangle:

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}
```

Rust does not implement getters automatically. We can do:

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn width(&self) -> u32 {
    self.width
  }
}

r.width(); // getter readonly access
```

## No -> operator

In C we have

```C
object = &Something; // object is a pointer

// these are similar
object->stuff();
(*object).stuff();
```

Rust has a feature _automatic referencing & dereferencing_ when calling _methods_.
  
This is possible because methods have a determined receiver (the type of `self`).
  
So rust can determine if the method is reading(`&self`), mutating(`&mut self`) or consuming(`self`)

## Methods with more parameters

```rust
impl Rectangle {
  fn area(&self) { ... }
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}
```

## Associated Functions

All functions and methods within `impl` are associated functions
  
Can also have functions without `self` as first param (thus not methods).
  
One example is the String::from() function, note that it's not called on an instance of `String`.

```rust
impl Rectangle {
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

Rectangle::square(10);
```

A common use for associated functions is to create new instances, e.g., with `Rectangle::new();`.

## Multiple impl Blocks

No problem to have multiple `impl` blocks for the same `struct`.

















































