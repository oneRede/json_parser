use chrono::{Datelike, Duration, Local, TimeZone, Timelike};

use crate::lexer::LexerError;

enum Error {
    LexerError(LexerError)
}

fn fatal_error(des: &'static str, error: Error){
    let fmt_datetime = "%Y年%m月%d日 %H:%M:%S";
    let datetime = Local::now().format(fmt_datetime);

    println!("Fatal Error: {}--{}--{}", datetime, error, des);
    process::exit(1);
}