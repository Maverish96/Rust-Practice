use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;
use colored::Colorize;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("secret: {}", secret);

    loop {
        print!("Input your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut guess = "".to_string();
        io::stdin()
            .read_line(&mut guess)  
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Please input a number!");
                continue
            }
        };

        
        println!("your guess: {}", guess.to_string().blue());

        match guess.cmp(&secret) {
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Equal => {
                println!("{}", "Correct!".green());
                break;
            }
        }
    }
    
}
