use std::{ops::Range, str::Chars, fmt};

#[allow(unused)]
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub range: Range<usize>,
}

impl Token {
    #[allow(unused)]
    fn new(token_type: TokenType, head: usize, tail: usize) -> Self {
        Self {
            token_type: token_type,
            range: head..tail,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum State {
    Init,
    String,
    Int,
    IntWithE,
    Float,
    FloatWithE,
    True,
    False,
    Null,
    Err,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
#[derive(Eq, Hash)]
pub enum TokenType {
    // lexical like "string"
    String,
    // lexical like 123456
    Int,
    // scientific notation int 1234e123
    IntWithE,
    // lexical like 12.3
    Float,
    // scientific notation float 1234e123
    FloatWithE,
    // lexical like false true
    True,
    // lexical like false false
    False,
    // lexical Null
    Null,
    // lexical ":"
    Colon,
    // lexical ","
    Comma,
    // lexical "{"
    LeftBigBracket,
    // lexical "}"
    RightBigBracket,
    // lexical "["
    LeftMiddleBracket,
    // lexical "]"
    RightMiddleBracket,
    // whitespace
    WhiteSpace, 
    //end
    End,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum _TokenType {
    Bool(Bool),
    Numeric(Numeric),
    String,
    Mark(Mark),
    Null,
    WhiteSpace,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum Bool {
    True,
    False,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum Numeric {
    Int(Int),
    Float(Float),
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum Int {
    Normal,
    Scientific,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum Float{
    Normal,
    Scientific,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum Mark {
    Colon,
    Comma,
    LeftBigBracket,
    RightBigBracket,
    LeftMiddleBracket,
    RightMiddleBracket,
}

#[allow(unused)]
#[derive(Debug)]
struct Cursor {
    chars: Chars<'static>,
    head: usize,
    now: usize,
    len: usize,
}

#[allow(unused)]
impl Cursor {
    fn new(s: &'static str) -> Self {
        Self {
            chars: s.chars(),
            head: 0,
            now: 0,
            len: s.len(),
        }
    }

    fn get_chars(&mut self, n: usize) -> Vec<char> {
        let mut chars:Vec<char> = vec![];
        for _ in 0..n{
            chars.push(self.chars.next().unwrap());
            self.now += 1;
        }
        chars
    }

    fn get_char(&mut self) -> Option<char> {
        self.now += 1;
        self.chars.next()
        
    }

    fn init(&mut self) {
        self.head = self.now;
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum LexerError {
    IntError,
    FloatError,
    StringError,
    ValueError,
}

impl fmt::Display for LexerError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(unused)]
#[derive(Debug)]
struct Lexer {
    str: &'static str,
    head: usize,
    now: usize,
    state: State,
    token: Vec<Token>,
}

impl Lexer {

    #[allow(unused)]
    fn new(s: &'static str) -> Self {
        Self {
            str: s,
            head: 0,
            now: 0,
            state: State::Init,
            token: vec![],
        }
    }

    #[allow(unused)]
    fn token(&mut self) {
        let chars = &mut self.str.chars();
        loop {
            if self.now > self.str.len() - 1 {
                break;
            }
            match self.state {
                State::Init => match chars.next().unwrap() {
                    '{' => {
                        self.now += 1;
                        let t = Token::new(TokenType::LeftBigBracket, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                    }
                    '}' => {
                        self.now += 1;
                        let t = Token::new(TokenType::RightBigBracket, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                    }
                    '[' => {
                        self.now += 1;
                        let t = Token::new(TokenType::LeftMiddleBracket, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                    }
                    ']' => {
                        self.now += 1;
                        let t = Token::new(TokenType::RightMiddleBracket, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                    }
                    ':' => {
                        self.now += 1;
                        let t = Token::new(TokenType::Colon, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                    }
                    ',' => {
                        self.now += 1;
                        let t = Token::new(TokenType::Comma, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                    }
                    ' ' => {
                        self.now += 1;
                        let t = Token::new(TokenType::WhiteSpace, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                    }
                    '0'..='9' => {
                        self.state = State::Int;
                        self.now += 1;
                    }
                    '"' => {
                        self.state = State::String;
                        self.now += 1;
                    }
                    't' => {
                        if chars.next().unwrap() != 'r' {
                            println!("error");
                            return ();
                        }
                        if chars.next().unwrap() != 'u' {
                            println!("error");
                            return ();
                        }
                        if chars.next().unwrap() != 'e' {
                            println!("error");
                            return ();
                        }
                        self.now += 4;
                        let t = Token::new(TokenType::True, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.state = State::Init;
                    }
                    'f' => {
                        if chars.next().unwrap() != 'a' {
                            println!("error");
                            return ();
                        }
                        if chars.next().unwrap() != 's' {
                            println!("error");
                            return ();
                        }
                        if chars.next().unwrap() != 'l' {
                            println!("error");
                            return ();
                        }
                        if chars.next().unwrap() != 'e' {
                            println!("error");
                            return ();
                        }
                        self.now += 5;
                        let t = Token::new(TokenType::False, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.state = State::Init;
                    }
                    'n' => {
                        if chars.next().unwrap() != 'u' {
                            println!("error");
                            return ();
                        }
                        if chars.next().unwrap() != 'l' {
                            println!("error");
                            return ();
                        }
                        if chars.next().unwrap() != 'l' {
                            println!("error");
                            return ();
                        }
                        self.now += 4;
                        let t = Token::new(TokenType::False, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.state = State::Init;
                    }
                    _ => {
                        println!("init error");
                        return ();
                    }
                },
                State::Int => match chars.next().unwrap() {
                    '0'..='9' => {
                        self.now += 1;
                    }
                    ',' => {
                        let t = Token::new(TokenType::Int, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                        self.now += 1;
                        let t = Token::new(TokenType::Comma, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                        self.state = State::Init;
                    }
                    ']' => {
                        let t = Token::new(TokenType::Int, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                        self.now += 1;
                        let t = Token::new(TokenType::RightMiddleBracket, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.state = State::Init;
                    }
                    '}' => {
                        let t = Token::new(TokenType::Int, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.now += 1;
                        let t = Token::new(TokenType::RightMiddleBracket, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.state = State::Init;
                    }
                    'e' | 'E' => {
                        self.now += 1;
                        self.state = State::IntWithE;
                    }
                    '.' => {
                        self.now += 1;
                        self.state = State::Float;
                    }
                    _ => {
                        println!("int error");
                        return ();
                    }
                },
                State::String => match chars.next().unwrap() {
                    ' '..='!' | '#'..='[' | ']'..='}' => {
                        self.now += 1;
                    }
                    '\\' => match chars.next().unwrap() {
                        'b' | 'f' | 'n' | 'r' | 't' | 'v' | '\\' | '\"' => {
                            self.now += 1;
                        }
                        _ => println!("error"),
                    },
                    '"' => {
                        self.now += 1;
                        let t = Token::new(TokenType::String, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.state = State::Init;
                    }
                    _ => println!("error"),
                },
                State::IntWithE => match chars.next().unwrap() {
                    '0'..='9' => {
                        self.now += 1;
                    }
                    ',' => {
                        let t = Token::new(TokenType::IntWithE, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                        self.now += 1;
                        let t = Token::new(TokenType::Comma, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                        self.state = State::Init;
                    }
                    ']' => {
                        let t = Token::new(TokenType::IntWithE, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                        self.now += 1;
                        let t = Token::new(TokenType::RightMiddleBracket, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.state = State::Init;
                    }
                    '}' => {
                        let t = Token::new(TokenType::IntWithE, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.now += 1;
                        let t = Token::new(TokenType::RightMiddleBracket, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.state = State::Init;
                    }
                    _ => {
                        println!("error");
                        return ();
                    }
                },
                State::Float => match chars.next().unwrap() {
                    '0'..='9' => {
                        self.now += 1;
                    }
                    ',' => {
                        let t = Token::new(TokenType::Float, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                        self.now += 1;
                        let t = Token::new(TokenType::Comma, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                        self.state = State::Init;
                    }
                    ']' => {
                        let t = Token::new(TokenType::Float, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                        self.now += 1;
                        let t = Token::new(TokenType::RightMiddleBracket, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.state = State::Init;
                    }
                    '}' => {
                        let t = Token::new(TokenType::Float, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.now += 1;
                        let t = Token::new(TokenType::RightMiddleBracket, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.state = State::Init;
                    }
                    'e' | 'E' => {
                        self.now += 1;
                        self.state = State::FloatWithE;
                    }
                    '.' => {
                        self.now += 1;
                        self.state = State::Float;
                    }
                    _ => {
                        println!("error");
                        return ();
                    }
                },
                State::FloatWithE => match chars.next().unwrap() {
                    '0'..='9' => {
                        self.now += 1;
                    }
                    ',' => {
                        let t = Token::new(TokenType::FloatWithE, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                        self.now += 1;
                        let t = Token::new(TokenType::Comma, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                        self.state = State::Init;
                    }
                    ']' => {
                        let t = Token::new(TokenType::FloatWithE, self.head, self.now);
                        self.token.push(t);

                        self.head = self.now;
                        self.now += 1;
                        let t = Token::new(TokenType::RightMiddleBracket, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.state = State::Init;
                    }
                    '}' => {
                        let t = Token::new(TokenType::FloatWithE, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.now += 1;
                        let t = Token::new(TokenType::RightMiddleBracket, self.head, self.now);
                        self.token.push(t);
                        self.head = self.now;
                        self.state = State::Init;
                    }
                    _ => {
                        println!("error");
                        return ();
                    }
                },
                _ => println!("others"),
            }
        }
    }
}

#[test]
fn test_lex_empty_json() {
    let s = "{}";
    let mut lexer = Lexer::new(s);
    lexer.token();
    println!("{:?}", lexer.token);
}

#[test]
fn test_lex_full_json() {
    let s = "{\"bool\":true,\"num\":1992,\"string\":\"abc\"}";
    let mut lexer = Lexer::new(s);
    lexer.token();
    println!("{:?}", lexer.token);
}
#[test]
fn test_lex_full_json_with_whitespace() {
    let s = "{\"bool\":true, \"num\":1992,\"string\":\"abc\"}";
    let mut lexer = Lexer::new(s);
    lexer.token();
    println!("{:?}", lexer.token);
}