fn main () {
    for n in 0..10 {
        let f = fib(n);
        println!("Fib #{n} is {f}.");
    }
}

fn fib(n: i32) -> i32 {
    let mut first;
    let mut second = 0;
    let mut current = 1;

    for _ in 0..n {
        first = second;
        second = current;
        current = first + second;
    }
    current
}

fn fib_rec(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 || n == 2 {
        return 1;
    }
    fib_rec(n-1) + fib_rec(n-2)
}
