mod lexer;
mod error;

use crate::lexer::LexerError;
fn main() {
    let _n: i32 = 3;
    println!("{:?}", LexerError::IntError);
}
