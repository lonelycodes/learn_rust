# Variables

Declare & define an _immutable_ variable:

```rust
let x = 0;
```

Declare & define a _mutable_  variable:

```rust
let x = 0;
```

Declare & define a _mutable_  variable:

```rust
let mut x = 0;
```

Shadowing a varialbe:

```rust
let x = 2;
let x = x + 1; // -> x is 3 in outer scope
{
  let x = x * 2; // -> x is 6 in inner scope
}
// -> x is again 3 in outer scope
```

Shadowing != `mut`

```rust
let s = "ABC";
let s = s.len(); // ->  s is 3

let mut t = "ABC";
t = t.len(); // -> compiler error (mismatched types)
```

