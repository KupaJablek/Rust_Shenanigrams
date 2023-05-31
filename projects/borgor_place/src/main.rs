// Welcome to my burger place
// this program will allow for user input to choose meal option and give price to user
//
// purpose: to learn basic input output in the crab language
use std::io; 

const BORGOR_PRICE:f32 = 2.25;

fn main() {
    println!("Welcome to the Borgor Place!");

    let total_cost = take_order();

    println!("The cost of your meal is ${total_cost}");
}

fn take_order() -> f32 {
    println!("How many borgor would you like: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Inocrrect Input");

    let borgor_count: f32 = input.trim().parse().expect("please enter a number");

    return borgor_count * BORGOR_PRICE;
}
