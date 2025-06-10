use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game!!");
    println!("Please enter your guess");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    io::stdin()
        .read_line(&mut guess)
        .expect("Fail to read the input");

    println!("You have entered your guess as: {guess}");
}
