use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new(); 
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read your number 😢");
    
        let guess: u32 = guess.trim().parse().expect("Could not read your number 😢");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small 😕"),
            Ordering::Greater => println!("Too big 😕"),
            Ordering::Equal => {
                println!("You won 🥳");
                break;
            }        
        }
    }
}
