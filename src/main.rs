mod lexer;
mod error;
mod parser_v2;

use crate::lexer::LexerError;
fn main() {
    let _n: i32 = 3;
    println!("{:?}", LexerError::IntError);
}
