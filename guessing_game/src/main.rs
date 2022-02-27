use rand::Rng;

use std::io;
use std::cmp::Ordering;

fn main() {
    println!("=== Guess the number! ===");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!(">> The secret number is: {}", secret_number);

    // allow multiple guess
    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        // Tip: one long line is difficult to read, so it’s best to divide it.
        // It’s often wise to introduce a newline and other whitespace to help 
        // break up long lines when you call a method with the .method_name() syntax.
        io::stdin()
            .read_line(&mut guess)
            .expect("<< Failed to read line >>"); // used for error handling

        // handling invalid input
        let guess: u32 = guess.trim().parse() => {
            Ok(num) => num,
            Err(_) => continue
        };

        println!(">> You have guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => println!("You win!"),
            Ordering::Greater => {
                println!("Too big!")
                break  // if you win the game, break
            }
        }
    }
}
