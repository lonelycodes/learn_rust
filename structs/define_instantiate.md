# Defining and Instantiating Structs

Similar to tuples: can hold multiple related values.
  
In structs you give the values names to make clear what they _mean_.

```rust
struct User{
  username: String,
  email: String,
  active: bool,
  sign_in_count: u64,
}

fn main() {
  let my_user = User {
    username: String::from("foobar"),
    email: String::from("foo@bar.com"),
    active: true,
    sign_in_count: 1337,
  };

  let mut mut_user = User {
    username: String::from("foobar"),
    email: String::from("foo@bar.com"),
    active: true,
    sign_in_count: 1337,
  };
  mut_user.active = false;
}

fn build_user(email: String, username: String) -> User {
  User {
    username: username,
    email: email,
    active: true,
    sign_in_count: 1,
  }
}
```

## Field Init Shorthand

```rust
fn build_user(email: String, username: String) -> User {
  User {
    username,
    email,
    active: true,
    sign_in_count: 1,
  }
}
```

## Struct Update Syntax

```rust
fn main () {
  let u1 = User {
    username: "u1"
    email: "e1",
    active: true,
    sign_in_count: 1,
  };

  let u2 = User {
    username: "u2",
    email: "e2",
    ..u1
  };
}
```

## Tuple Structs

No named fields -> access with `white.0` etc.k

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let white = Color(255, 255, 255);
let home = Point(0,0,0);
```

`white` and `home` have different types.
  
# Unit-Like Structs

No fields at all.
  
Behave similar to `()`.
  
When we don't want to store data in type itself but define traits for it still.

```rust
struct MyUnit;

let u = MyUnit;
```

## Ownership of Struct Data

Note that in `User` example we used `String` (owned type) instead of &str `string slice type`.
  
To store references to data owned by someone else we need to use _lifetimes_.
  
Lifetimes ensure the data referenced is valid as long as the strut.

