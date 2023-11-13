use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

loop{
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
//VERSIONS WITH PROBLEMS
/*
1. let guess_int:u32 = guess.trim().parse();
2. let guess_int:u32 = guess.trim().parse().expect("Please type a number");
3. let guess_int = guess.trim().parse::<u32>();
4. let guess_int = guess.trim().parse::<u32>().expect("Please type a number");
     */
    let guess_int:u32 = guess
        .trim()
        .parse()
        .unwrap();
        

    println!("You guessed: {}", guess);

    match guess_int.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
    }
    }
    
}
}
