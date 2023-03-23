use chrono::{Datelike, Duration, Local, TimeZone, Timelike};
use std::{fmt, process};

use crate::lexer::LexerError;

#[derive(Debug)]
enum Error {
    LexerError(LexerError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
