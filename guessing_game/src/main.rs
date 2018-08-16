extern crate rand;

use rand::Rng;
use std::io;

// entry point of the program
fn main() {
    // macro that prints content to the console
    println!("This is the Rust Guessing Game!");

    println!("What is your name?");
    // create and return a new instance of String
    let mut player_name = String::new();
    // read_line returns io::Result, Ok or Err
    io::stdin()
        .read_line(&mut player_name) // & indicates that this is a reference to something
        .expect("Failed to read in player's name");
    println!("Welcome to the guessing game {}", player_name);

    println!("And what is your age?");
    let mut player_age = String::new();
    io::stdin()
        .read_line(&mut player_age)
        .expect("Failed to read in player's age");
    println!("Ok, your age is {}", player_age);

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    println!("Make a new guess: ");
    // in Rust, variables are immutable by default
    let mut guess = String::new(); // this is a mutable variable, new instance of a string
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
