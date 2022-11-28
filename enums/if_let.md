# Concise Control Flow with if let

`if let` can be used as a less verbose option to handle values that match one pattern while ignoring the rest.

```rust
let config_max = Some(3u8);
match config_max {
  Some(max) => println!("The max is {}", max),
  _ => (),
}
```

Can be written as:

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
  println!("The max is {}", max);
}
```

Another example: counting all non-quarter coins while announcing states:

```rust
let mut count = 0;
match coin {
  Coin::Quarter(state) => println!("Found state quarter from {:?}", state),
  _ => count += 1,
}
```

Can be written as:

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
  println!("Found state quarter from {:?}", state);
} else {
  count += 1;
}
```

