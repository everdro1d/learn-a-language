use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number between 1-100\nType 'quit' to end the game.");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Enter a number: "); // print needs to flush stdout manually
        let _ = io::stdout().flush(); // 'let _ =' ignores the Result

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failure when reading user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) if (guess.to_lowercase().contains("quit")) => break,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

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
