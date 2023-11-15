use std::io;
pub fn main(){
  let mut a = String::new();
  let mut b = String::new();  
  println!("Input your value for a");
  io::stdin().read_line(&mut a).expect("Failed to read line");
  println!("Input your value for b");
  io::stdin().read_line(&mut b).expect("Failed to read line");
  let a = a.trim().parse::<f32>().unwrap();
  let b = b.trim().parse::<f32>().unwrap();
  println!("The addition of {} and {} is {}", a,b,addition(a, b));
  
  
      //let x = plus_one(5);
      //println!("The value of x is: {x}");


  
}
fn addition(x:f32, y:f32) -> f32{
  let result = x+y;
  //println!("The addition of {} and {} is {}", x, y, result);
  result
  
}
fn plus_one(x: i32) -> i32 {
    x + 1
}