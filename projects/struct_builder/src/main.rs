// build a struct
use std::io;

#[derive(Debug)]
enum MaterialQuality {
    Premium,
    Standard,
    Bargin,
}

#[derive(Debug)]
struct Bear {
    fur_colour: String,
    name: String,
    quality: MaterialQuality,
    price: f32,
}

fn main() {
    println!("Welcome to build a bear!");

    let mut fur_colour = String::new();
    let mut name = String::new();

    println!("Enter the name of your bear: ");
    io::stdin().read_line(&mut name).expect("failed to readline");
    name.pop(); // pop() removed the newline or /r at the end of the input

    println!("Enter the colour of your bear");
    io::stdin().read_line(&mut fur_colour).expect("failed to readline");
    fur_colour.pop();

    println!("Select and option for the material quality");
    println!("1. Bargin");
    println!("2. Standard");
    println!("3. Premium");

    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).expect("failed to readline");
    let the_choice: i32 = user_choice.trim_end().parse::<i32>().unwrap(); // similar to "pop",
                                                                          // removes white space
                                                                          // and \n\r characters

    let the_result = match the_choice {
       1 => MaterialQuality::Bargin,
       2 => MaterialQuality::Standard,
       3 => MaterialQuality::Premium,
       _ => MaterialQuality::Bargin
    };

    let new_bear = Bear {
        fur_colour: fur_colour,
        name: name,
        quality: the_result,
        price:  45.45,
    };

    println!("You have created: ");
    println!("{:?}", new_bear);
}
