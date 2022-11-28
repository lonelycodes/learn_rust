# The Match Control Flow Construct

`match` compares a value against a series of `patterns` and executes code based on which pattern matches.

```rust
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
```

In contrast, an `if` expression needs to return a _boolean_ value.
  
Can also execute more code in curly braces:

```rust
fn value_in_cents(coin: Coin) -> u8 {
match coin {
    Coin::Penny => {
      println!("Lucky penny!");
      1
    }
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
```

## Patterns that Bind to Values

```rust
#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
  ...
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quasrter from {:?}", state);
      25
    }
  }
}
```

## Matching With Option<T>

```rust
fn add_one(n: Option<i32>) -> Option<i32> {
  match n {
    None => None,
    Some(i) => Some(i + 1),
  }
}

let five = Some(5);
let six = add_one(five);
let none = add_one(None);
```

Note that matches are _exhaustive_ so we have to cover all cases:

```rust
fn add_one(n: Option<i32>) -> Option<i32> {
  match n {
    Some(i) => Some(i + 1),
  } // compiler error: pattern 'None' not covered
}
```

## Catch-all Patterns and the _-Placeholder

```rust
fn move_a() {}
fn move_b() {}
fn move_c(n: u8) {}
fn default() {}

let dice_roll = 3;

match dice_roll {
  1 => move_a(),
  3 => move_b(),
  other => move_c(other),
}

match dice_roll {
  1 => move_a(),
  3 => move_b(),
  _ => default(),
}

match dice_roll {
  1 => move_a(),
  3 => move_b(),
  _ => ()
}
```



















































