// 'Guess-the-number' game implemented in Rust.

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

// begins program execution.
fn main() {
	println!("I am thinking of a number.. can you guess it?");
	
    // generates a number between 1 and 100 (inclusive)
	let secret_number = rand::thread_rng().gen_range(1, 101); 
	
	loop {
		println!("Please input your guess.");
	
		let mut guess = String::new();
        // read user's input, and bind it to the 'guess' variable
        // print failure message, if an error occurs.
		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");
        // ensures user's guess is a number
        // if not, skip to the next iteration
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		println!("You guessed: {}", guess);
        // compares the user input to the number to guess
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}
