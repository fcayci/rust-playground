// Guess digits game
use std::io;
use rand::Rng;

fn main() {
    println!("Guess digits game.");
    println!("+2 / -2 stuff...");

    let number = rand::thread_rng().gen_range(1000..10000);
    let _number: String = number.to_string();
    let mut steps = 0;
    println!("My number is {number}");

    loop {

        println!("Enter 4 digit number:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read stdin");

        let _guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut pres = 0;
        let mut nres = 0;
        for (i, c) in guess.chars().enumerate() {
            for (j, nc) in _number.chars().enumerate() {
                if c == nc {
                    if i == j {
                        pres += 1;
                    } else {
                        nres += 1;
                    }
                }
            }
        }

        if pres == 4 {
            break;
        }
        println!("+{pres} / -{nres}");
    }

    println!("Congrats! you completed the game in {steps} steps!");
}
