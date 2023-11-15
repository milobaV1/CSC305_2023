pub fn main(){
  let mut x = 5i32;
  println!("X was {}",x);
  x = 7i32;
  println!("x is now {}", x);

  //shadowing
  let y = "   ";
  let y = y.len();

  println!("y is {}", y);
}