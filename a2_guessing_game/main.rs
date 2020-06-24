use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("===========Guess Game===========");
    println!("Guess number from 1-100");
    println!("================================");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please guess a number>> ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your input not a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win the game!");
                break;
            }
        }
    }
}
