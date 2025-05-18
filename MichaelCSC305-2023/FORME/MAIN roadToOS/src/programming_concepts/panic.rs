pub fn main() {
    let number = -5;

    if number >= 0 {
        // Everything is fine, let's keep going
        println!("Positive number: {}", number);
    } else {
        // Uh-oh, we didn't expect a negative number!
        panic!("Encountered a negative number: {}", number);
    }
}
