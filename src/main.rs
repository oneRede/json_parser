mod lexer;
mod error;
mod parser_v2;

use crate::lexer::LexerError;


fn main() {
    const _GOTO: [i32;3] = [1,2,3];
    let _n: i32 = 3;
    println!("{:?}", LexerError::IntError);
}
