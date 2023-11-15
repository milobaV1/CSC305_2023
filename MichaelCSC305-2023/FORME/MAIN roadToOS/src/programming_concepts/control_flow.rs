pub fn main(){
  let num = 5;
  if true {
    println!("{} is equal to 5",num);
  }

  let condition = false;
  let num1 = if condition {5} else {7};
  println!("{}",num1);

  for n in (1..=10).rev(){
    println!("{}",n);
  }
  
}