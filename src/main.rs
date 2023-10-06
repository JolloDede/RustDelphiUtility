use crate::lexer::tokenize;

mod lexer;

fn main() {
    println!("Hello");
    let _ = tokenize("test");
}
