use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Welcome Game of Guessing!.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut max = 0;
    let mut left = 5;
    // println!("Secret Code is: {secret_number}");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter number only...");
                continue;
            }   
        };
        // .expect("Please type a number!");
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => { 
                println!("You win");
                break;
            }
        }
        if max >= 5{
            println!("You lost..! Max number to guess was 5");
            break;
            }
        max += 1; 
        left -= 1;
        println!("Chances left-----> {left}");
    }
}
