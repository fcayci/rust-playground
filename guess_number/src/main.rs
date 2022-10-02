use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main()
{
    println!("Guessing game (almost) straight from the rust-lang book.");
    let number = rand::thread_rng().gen_range(1..=100);
    let mut steps = 0;

    loop {
        println!("Guess a number [0-100]:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read.");

        // Will exit
        // let guess: u32 = guess.trim().parse()
        //     .expect("Should be a number");

        // Better alternative
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Funny... Try again");
                continue;
            },
        };

        match guess.cmp(&100) {
            Ordering::Greater => {
                println!("Out of range");
                continue;
            },
            _ => {},
        }

        steps += 1;
        match guess.cmp(&number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Foind it!");
                break;
            }
        }
    }

    println!("Congrats! Found the number in {steps} steps.");

}
