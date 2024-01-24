use std::cmp::Ordering;

// use std::io;
use rand::Rng;

pub fn main() {
    loop {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        let mut guess = String::new();

        println!("Please input your guess.");

        // std::io::stdin()
        //     .read_line(&mut guess)
        //     .expect("Failed to read line");

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim() == "quit" {
                    break;
                }

                println!("Invalid number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small. Secret number was: {secret_number}"),
            Ordering::Greater => println!("Too big. Secret number was: {secret_number}"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
