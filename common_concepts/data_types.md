# Data Types

Rust is statically typed, i.e., types of all variables must be known at compile-time.
  
Compiler can do type inference.
  

If multiple types are possible we need to annotate:

```rust
let guess = "1337".parse().expect("NaN!"); // -> compiler error "type annotations needed"
let guess: u32 = "1337".parse().expect("NaN!"); // -> guess is now 1337
```

## Scalar Types

I.e., singe values.
  
Integers, floating point numbers, booleans and characters.

### Integer Types
can be signed `i*` or unsigned `u*`
  
Can have different lengths: 8-128 bit, e.g., `i32`, `u8`, `u16`, ...
  
`isize` and `usize` depends on the arch we run on x86_64 -> 64-bit
  
Literals:

* Decimal: `13_337`
* Hex: `0xff`
* Oct: `0o77`
* Binary: `0b1111_0000`
* Byte(`u8`): `b'A'`
  
  
Which one to use: depends. If unsure, set to `i32`.
  
`isize` and `usize` are mostyly used when indexing collections.
  
When integer overflows: in debug mode just panics. In `--release`: two's complement wrapping!
  
Can use `wrapping_add` etc to explicitly handle overflows.

### Floating-Point Types

There are `f32` and `f64`.

```rust
let a = 2.0; // f64 by default (almost as fast as f32 on modern machines)
let b: f32 = 3.0; // f32
```

### Numeric Operations

As expected

### Boolean Type

```rust
let a = true;
let b: bool = false;
```

### Character Type

```rust
let c = 'a';
let z: char = 'ℤ';
let poop = '💩';
```

Have to be single quotes. Strings have to be double quotes.
  
Chars in rust are 4 bytes, representing unicode values.

## Compund Types

Grouping multiple values into one type.

### Tuple Type

```rust
let tup: (i32, char) = (1, 'a');
let tup2 = (100, 1.2, 'a'); // can infer the types
let (x,y,z) = tup2; // can destructure
let c = tup.2; // c is 'a'
let u = (); // special 'unit' case aka empty value
```

### Array Type

```rust
let a: [i32, 3] = [1, 2, 3]; // can infer the types
let b = [1, 2, 3];
let c = [2; 5]; // c is [2, 2, 2, 2, 2]
let d = a[0]; // d is 1
```

Keeps data on `stack` vs on `heap`.
  
Fixed number of elements. Not flexible like vector type.
  
Vectors are also in stdlib and are allowed to grow and shrink in size.
  
When in doubt: use vector.
  
Invalid access panicks.











































