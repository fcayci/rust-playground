use std::io;

fn main() {
    println!("Generate nth Fibonacci number");

    let mut limit = String::new();

    println!("Enter the sequence number to calculate:");

    io::stdin()
        .read_line(&mut limit)
        .expect("Failed to read.");

    let limit : u64 = match limit.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Cannot convert to number");
            return;
        },
    };

    let mut x : usize = 0;
    let mut y : usize = 1;
    for _ in (1..limit).rev() {
        (x, y) = (y, x + y);
    }

    println!("{limit}th Fibonacci number is {y}!");
}

