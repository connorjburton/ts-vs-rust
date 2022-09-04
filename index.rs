// https://doc.rust-lang.org/std/primitive.i32.html#method.saturating_sub - to handle under/overflows

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
            // we are borrowing guess here, not giving ownership to read_line
            .read_line(&mut guess)
            // this causes a panic and would not normally be in production code
            .expect("Failed to read line");
        
        // .parse() knows what type to parse it to as we have provided the new type
        // as the type annotation of this let
        // this also uses Option, this is an alternative to null
        // https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // https://doc.rust-lang.org/std/primitive.i32.html#impl-Eq
        // https://doc.rust-lang.org/std/cmp/trait.Eq.html - example of custom .cmp
        match guess.cmp(&secret_number) {
            // These are enums that we are comparing against
            // https://doc.rust-lang.org/std/cmp/enum.Ordering.html
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println("You win!");
                break;
            }
        }
    }
}