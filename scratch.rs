fn main() {
    string_lit();
}

fn string_lit() {
    let mut a = "hello";
    a.push_str(", world!");
    println!("{a}");
}
