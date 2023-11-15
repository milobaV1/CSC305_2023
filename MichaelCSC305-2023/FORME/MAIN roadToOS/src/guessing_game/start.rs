use std::io;
use rand::Rng;
use std::cmp::Ordering;
pub fn main() {
    //println!("Number {} and {} in binary are {:b} and {:b}", 10, 20, 10, 20);
    //println!("{:?}", ("This is ben", 10, "and kevin", 11));

  //println!("Safe no");

  //let mut age = 17;

  //println!("My age is {}", age);

  //age = 18;


  /*let a:u8 = 1;
  let b:isize = 15;
  println!("a: {}", a);
  println!("b: {}", b);

  let arr:[isize ; 4] = [1,2,3,4];
  println!("First element of this array is {}", arr[0]);
  println!("While all the elements of this array are: {:?}", arr);
  */
  println!("Guess the number!");
  //GENERATE A RANDOM NUMBER
  let secret_number:i32 = rand::thread_rng()
    .gen_range(1..=20);
  loop{
    println!("\nInput your guess");
  //CREATE AN EMPTY VARIABLE THAT RECEIVES A STRING AS INPUT  
  let mut guess= String::new();
  //COLLECTING THE USER'S INPUT
  io::stdin()
    .read_line(&mut guess)
    .expect("Please input a value");

  print!("You have guessed {}",guess);
  //CONVERTING THE STRING TO A NUMBER
  let guess = guess.trim().parse::<i32>().unwrap();

  //COMPARING THE USER'S GUESS TO THE RANDOMLY GENERATED NUMBER
  match guess.cmp(&secret_number){
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big"),
    Ordering::Equal => {println!("Correct");
                       break;
                       }
  }
  }




}


