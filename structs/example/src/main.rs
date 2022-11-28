/* Initial Version */

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("Area of rectangle: {} square pixels.", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}


/* Refactoring with Tuples */

fn main() {
    let rect = (30, 50);
    println!("Area of rectangle: {} square pixels.", area(rect));
}

fn area(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

/* Refactoring with Structs */

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area of rectangle: {} square pixels.", area(&rect));
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}

/* Tips for output */

println!("rect is {r}"); // compile error Rectangle does not implement std::fmt::Display

println!("rect is {:?}", r); // compile error Rectangle does not implement Debug trait

#[derive(Debug)] // need this to be able to use :? and :#? in format string
struct Rectangle {
    width: u32,
    height: u32,
}

dbg!(&r); // dbg macro can also be used when we derive the Debug trait
