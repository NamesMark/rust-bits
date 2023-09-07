use std::io;
use rand::prelude::*; // 0.8.5

pub fn main() {
    println!("Higher or lower game!");
    let secret: u8 = thread_rng().gen_range(1..101);
    let mut buffer = String::new();
    let mut number: u8  = 0; 
    let mut count : u32 = 0;
    println!("Your guess: {number}");
    while number != secret {
        count += 1;
        println!("Attempt {count}. Guess a number between 1 and 100:");
        io::stdin().read_line(&mut buffer);
        number = buffer.trim().parse::<u8>().unwrap();
        buffer.clear();
        if number < 1 || number > 100 {
            println!("Bad input lol");
            continue;
        } else if number == secret {
            println!("Congrats, indeed it is {secret}. You won in {count} turns.");
        } else if number < secret {
            println!("No, {number} is smaller than the secret number!");
        } else {
            println!("No, {number} is larger than the secret number!");
        } 
    }

}