use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the secret number is: {secret_number}");
    loop {
    println!("Please input your guees.");
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // var declared with "mut" so that it is mutable.
    let mut guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    }

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => {
            println!("You Win!");
            break;
        },
    }
    }
}
