use stdin_readlines::*;

fn main() {
    println!("Hello, world!");
    let mut s = String::new();
    stdin_readlines(&mut s);
    println!("{}", s);
}
