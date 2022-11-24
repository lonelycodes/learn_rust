# Control Flow

## IF-Expression

```rust
let a = 2;

if a < 3 { // condition _must_ be a bool. no falsy truthy weirdness in rust :)
  // ...
} else if a > 5 {
  // ...
} else {
  // ...
}
```

You can use `if` in `let`-statements:

```rust
let condition = true;
let a = if condition { 1 } else { 2 }; // a is 1
```

## Loops

Endless loop:

```rust
loop {
  // ...
}
```

Break out of it:

```rust
let mut cnt = 0;

let x = loop {
  counter += 1;
  if counter == 10 {
    break counter;
  }
}; // x is 10
```

`continue` and `break` like in most other languages. Applies to _innermost_ loop.

Can `continue` and `break` to some outer loop too:

```rust
'outer: loop {
  loop {
    break 'outer;
  }
}
```

### While

```rust
let mut n = 0;
while n < 10 {
  n += 1;
}
```

### For

```rust
for i in [1, 2 , 3] {
  // ...
}

for i in (1..4).rev() { // Ranges
  // ...
}
```


