/* word counter (wc kind of) */

/* counter number of words in a given string */
fn word_counter(s: &String) -> usize {
    // convert to bytes
    let b = s.as_bytes(); 

    let mut word = 0;
    let mut begin = true;
    let mut end = true;
    let mut duplicate = false;

    for &item in b.iter() {
        if item == b' ' || item == b'\n' || item == b'\t' {
            if !begin && !duplicate {
                word += 1;
            }
            end = true;
            duplicate = true;
        } else {
           duplicate = false; 
           begin = false;
           end = false;
        }
    }

    if !end {
        word += 1;
    }

    word
}

fn main() {
    let s1 = String::from("The Rust Programming Language");
    let s2 = String::from("  The Rust Programming Language");
    let s3 = String::from("The Rust Programming Language    ");
    let s4 = String::from("   The Rust Programming Language    ");
    let s5 = String::from(" The   Rust   Programming   Language    ");

    println!("Number of words in {}: {}", s1, word_counter(&s1));
    println!("Number of words in {}: {}", s2, word_counter(&s2));
    println!("Number of words in {}: {}", s3, word_counter(&s3));
    println!("Number of words in {}: {}", s4, word_counter(&s4));
    println!("Number of words in {}: {}", s5, word_counter(&s5));
}
