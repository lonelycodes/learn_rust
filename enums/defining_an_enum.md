# Defining an Enum

Enums provide a way to say that a value is _one of a possible set_ of values.

## Enum Values

```rust
enum IpAddrKind {
  V4,
  V6
}

let four = IpAddrKind::V4
let six = IpAddrKind::V6

fn route(kind: IpAddrKind) {  }

// both are valid because both values are of the same type
route(four);
route(six);
```

There's even more benefits when combining with structs:

```rust
enum IpAddrKind {
  V4,
  V6
}

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

let home = IpAddr {
  kind: IpAddrKind::V4,
  address: String::from("127.0.0.1"),

let loopback = IpAddr {
  kind: IpAddrKind::V6,
  address: String::from("::1"),
}
```

We can even express the same concept with just an `enum`

```rust
enum IpAddr {
  V4(String),
  V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

But now both V4 and V6 are just `String`. Nothing prevents us from doing it wrong. Enums allow for different types:

```rust
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

Check out how the std lib of rust does it:

```rust
struct Ipv4Addr { ... }
struct Ipv6Addr { ... }

enum IpAddr {
  V4(Ipv4Addr),
  V6(Ipv6Addr),
}
```

## Enums vs Structs

Consider the following enum:

```rust
enum Message {
  Quit, // no data associated
  Move {x: i32, y: i32}, // named fields like a struct
  Write(String), // a single String
  ChangeColor(i32, i32, i32), // Three i32 values
}
```

We could do something similar with structs:

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
  x: i32,
  y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

With the enum we can also make `impl` blocks:

```rust
impl Message {
  fn call(&self) {
    // ...
  }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## The Option Enum (and why it's better than Null Values)

`Option` encodes the scenario where a value could be _something_ or it could be _nothing_.
  
Rust does not have Tony Hoare's "Billion Dollar Mistake": the _Null value_.

```rust
enum Option<T> {
  None,
  Some(T),
}

let some_number = Some(5);
let some_char = Some('c');
let no_number: Option<i32> = None;
```

The `<T>` syntax is new and specifies generic type parameters (more on them later).

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let s = x + y; // compiler error: mismatched types
```

We'll see how to access the _inner_ `T` in the next chapter on `match`.

