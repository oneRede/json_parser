use chrono::Local;
use std::fmt;

use crate::lexer::LexerError;


#[allow(unused)]
#[derive(Debug)]
enum Error {
    LexerError(LexerError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(unused)]
fn fatal_error(des: &'static str, error: Error) {
    let fmt_datetime = "%Y-%m-%d %H:%M:%S";
    let datetime = Local::now().format(fmt_datetime);

    println!("{} Fatal Error({}): {}", datetime, error, des);
    // process::exit(1);
}

#[test]
fn test_error_print() {
    let error = Error::LexerError(LexerError::IntError);
    let des = "not a int fmt";
    fatal_error(des, error);
}
