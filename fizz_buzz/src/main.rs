// Fizz Buzz Rust

fn main() {
    println!("Fizz buzz thingy..");

    for num in 1..=100 {
        match (num % 3, num % 5) {
            (0, 0) => println!("{num:3}: FizzBuzz!"),
            (0, _) => println!("{num:3}: Fizz"),
            (_, 0) => println!("{num:3}: Buzz"),
                 _ => println!("{num:3}"),
        }
    }
}
