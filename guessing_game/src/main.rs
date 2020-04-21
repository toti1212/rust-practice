use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    
    // Generating the secret number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // Guessing...
    loop {
        println!("Please, enter your number:");

        // Create a new string variable to read the input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the number");
    
        // Parse the input into u32 for campare
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        
        // Compare the number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your number is too small"),
            Ordering::Greater => println!("Your number is too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
