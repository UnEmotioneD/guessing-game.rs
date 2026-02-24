use rand::prelude::*;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn get_number_input(msg: &str) -> u32 {
    loop {
        print!("{msg}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(num) => break num,
            Err(_) => println!("Not a number"),
        };
    }
}

fn main() {
    println!("\n==============");
    println!("Guessing game!");
    println!("==============\n");

    let secret_number = rand::rng().random_range(1..=100);

    let limit: u32 = get_number_input("Set limit of attempt: ");
    println!("Attempt limit: {limit}");

    let mut cnt: u32 = 0;

    loop {
        cnt += 1;
        let msg = format!("\nplease input your guess({cnt}/{limit}): ");

        let guess: u32 = get_number_input(&msg);
        println!("Your guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };

        if limit == cnt {
            println!("You have reached the limit!");
            println!("The secret number was: {secret_number}");
            break;
        }
    }
}
