use std::io;
// Not using Ordering due to the new rand crate returning an int instead of a String

use rand::random_range; // Importing what I just need

fn main() {
    /*  
        From now on, I will use the 4 spaces indentation style
        I'm not used to it but whatever, you gotta adapt sometimes!
    */

    println!("Guess the number!");

    let secret_number = random_range(1..=100);
    /*
            In the book they use rand::thread_rng().gen_range(1..=100), 
            but it's deprecated in the latest version, so I just opened the docs
            (cargo --doc open) and found the new way to do it
        */

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read the line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // Comment example (works the same as JS)
        println!("You guessed: {guess}");

        if guess < secret_number {
            println!("Too small!");
        } else if guess > secret_number {
            println!("Too big!");
        } else if guess == secret_number {
            println!("You win!");
            break;
        }
    }
}
