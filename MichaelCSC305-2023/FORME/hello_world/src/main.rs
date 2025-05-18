fn lis(n: u8){
    n + n*n + n*n*n
}

fn main(){
    let a = lis(5);
    println!("{}", a)
}