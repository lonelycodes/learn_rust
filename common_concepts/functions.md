# Functions

```rust
fn main() { 
  other();
}

fn other() {  }
```

## Parameters

```rust
fn main() {
  other(1, 'a');
}

fn other(a: i32, b: char) {  }
```

## Statements and Expressions

Statement performs action, returns no value. E.g., `let`.

```rust
let x = (let y = 0); // can't do this in rust! You could in C...
```


Expression evaluates to some returning value. E.g., `1+2`.

```rust
let x = {
  let x = 0;
  x + 1; 
}; // x is 1
```

## Return Values

```rust
fn one() -> i32 {
  1 // just the expression, no semicolon needed
}
```
