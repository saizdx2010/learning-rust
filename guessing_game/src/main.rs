use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    let random_number = rand::thread_rng().gen_range(0..=100).to_string();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    if guess == random_number {
        println!(
            "Correct!, you guessed: {} and the random number was {}",
            guess, random_number
        );
    }

    if guess != random_number {
        println!(
            "Incorrect!, you guessed: {} and the random number was {}",
            guess, random_number
        );
    }

    match guess.cmp(&random_number) {
        Ordering::Less => println!("Too small, try again next time"),
        Ordering::Equal => println!("Jackpot, you got it"),
        Ordering::Greater => println!("Too big, try again next time"),
    }
}
