use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game!!");
    println!("Please enter your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fail to read the input");
    // converting the input to integer
    let guess: u32 = guess.trim().parse().expect("Please enter a number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("You have entered your guess as: {guess}");
}
