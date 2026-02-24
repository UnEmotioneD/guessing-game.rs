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

// TODO: when the guess is out of range
fn next_best_guess(left_edge: u32, right_edge: u32, guess: u32, is_too_big: bool) -> (u32, u32) {
    let mut new_left = left_edge;
    let mut new_right = right_edge;

    let next_best;
    if is_too_big {
        next_best = (left_edge + guess) / 2;
        new_right = guess;
    } else {
        next_best = (right_edge + guess) / 2;
        new_left = guess;
    }

    let msg = format!("Next best guess is: {next_best}");
    println!("{msg}");

    (new_left, new_right)
}

fn main() {
    println!("\n==============");
    println!("Guessing game!");
    println!("==============\n");

    let mut left_edge: u32 = 1;
    let mut right_edge: u32 = 100;

    let secret_number = rand::rng().random_range(left_edge..=right_edge);

    let limit: u32 = get_number_input("Set limit of attempt: ");
    println!("Attempt limit: {limit}");

    let mut cnt: u32 = 0;

    let mut is_too_big;

    loop {
        cnt += 1;
        let msg = format!("\nplease input your guess({cnt}/{limit}): ");

        let guess: u32 = get_number_input(&msg);
        println!("Your guess: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                is_too_big = false;
            }
            Ordering::Greater => {
                println!("Too big!");
                is_too_big = true;
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };

        (left_edge, right_edge) = next_best_guess(left_edge, right_edge, guess, is_too_big);

        if limit == cnt {
            println!("You have reached the limit!");
            println!("The secret number was: {secret_number}");
            break;
        }
    }
}
