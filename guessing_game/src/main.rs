use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /**
         * The match expression is used to handle the two possible outcomes of the parse() method. 
         * 
         * If the input is successfully parsed into an unsigned 32-bit integer, 
         * the Ok variant of the Result enum is returned with the parsed integer as its value. 
         * This value is then assigned to the guess variable of type u32. 
         * If the input cannot be parsed into an unsigned 32-bit integer, the Err variant of the Result enum is returned. 
         * In this case, the continue keyword is used to skip the rest of the loop iteration and move on to the next one.
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
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