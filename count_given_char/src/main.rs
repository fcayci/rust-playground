/* similar to the first_word example in 4.3 */

/* count the number of given char in the string */
fn count_chars(s: &String, c: char) -> usize {
    let mut counter = 0;

    for ch in s.chars() {
        if ch == c {
            counter += 1;
        }
    }
    counter
}

fn main() {
    println!("Finding number of chars in the string:");

    let s = String::from("The Rust Programming Language Learning");
    println!("{}", s);

    let mut c = 'R';
    println!("Number of {} is: {}", c, count_chars(&s, c));

    c = 'L';
    println!("Number of {} is: {}", c, count_chars(&s, c));

    c = 'a';
    println!("Number of {} is: {}", c, count_chars(&s, c));

    c = 'y';
    println!("Number of {} is: {}", c, count_chars(&s, c));

    c = 'g';
    println!("Number of {} is: {}", c, count_chars(&s, c));

    c = 'm';
    println!("Number of {} is: {}", c, count_chars(&s, c));
}
