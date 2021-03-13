use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

pub fn guessing_game() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..100);
    println!("The secret number is {}", secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("Too small!") }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => {println!("Too big!")}
        }
    }
}