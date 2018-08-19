extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
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
    let player_name = player_name.trim();
    println!("Welcome to the guessing game {}", player_name);

    println!("And what is your age?");
    let mut player_age = String::new();
    io::stdin()
        .read_line(&mut player_age)
        .expect("Failed to read in player's age");

    let player_age: u32 = player_age
        .trim()
        .parse()
        .expect("Please type in a number for your age");

    println!("Ok, your age is {}", player_age);

    let mut guess_count = 0;

    if player_age > 18 {
        println!("Guess the number!");
        let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
        // println!("The secret number is: {}", secret_number);
        loop {
            println!("Make a new guess: ");
            guess_count += 1;
            // in Rust, variables are immutable by default
            let mut guess = String::new(); // this is a mutable variable, new instance of a string
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("You guessed: {} (guess count: {})", guess, guess_count);
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small...\n"),
                Ordering::Greater => println!("Too large...\n"),
                Ordering::Equal => {
                    println!("You got it! You win!");
                    break;
                }
            }
        }
    } else {
        println!("You are too young to play...");
    }
}
