mod lexer;
mod error;
mod parser_v2;

use crate::lexer::LexerError;



fn main() {
    #[derive(Copy, Clone, Debug)]
    enum Value{
        A,
        B,
    }

    let mut a = Value::A;
    let b = a;
    a = Value::B;
    println!("{:?}", a);
    println!("{:?}", b);
}
