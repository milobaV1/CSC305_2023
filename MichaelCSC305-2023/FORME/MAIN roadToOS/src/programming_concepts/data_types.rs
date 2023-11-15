pub fn scalar(){
let x = 1005i16;
  println!("x: {x}");
}
pub fn compound(){
  //TUPLES
  let tup = (500i16, 6.4f32, true);
  //DESTRUCTURING
  let (a,b,c) = tup;
  println!("The value of a is {a}\nThe value of b is {b}\nThe value of c is {c}");
  //ACCESSING THE TUPLE USING . OPERATOR
  println!("{},{},{}",tup.0,tup.1,tup.2);
  //ARRAYS
  let arr = [4i8;5];
  println!("First value is {}", arr[0]);
}