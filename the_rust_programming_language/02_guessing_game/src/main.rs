use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess.");
    loop {
        let mut guess_str = String::new();
        io::stdin().read_line(&mut guess_str)
            .expect("Failed to read line");
        
        match guess_str.trim().parse::<u32>() {
            Ok(_) => {
            },
            Err(_) => {
                match guess_str.trim() {
                    "quit" => {
                        println!("Goodbye!");
                        break;
                    },
                    _ => {
                        println!("{} is not a number", guess_str.trim());
                        continue;
                    }
                };
            },
        };

        let guess: u32 = guess_str.trim().parse()
            .expect("This should not happen.");

        println!("You guessed: {}", guess);
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
