use std::ops::Range;

struct Token {
    token_type: TokenType,
    range: Range<usize>,
}

impl Token {
    fn new(token_type: TokenType, head: usize, tail: usize) -> Self {
        Self {
            token_type: TokenType::Int,
            range: head..tail,
        }
    }
}
#[derive(PartialEq)]
enum State {
    Init,
    String,
    Int,
    Float,
    True,
    False,
    Null,
    Err,
}

#[derive(PartialEq)]
enum TokenType {
    // lexical like "string"
    String,

    // lexical like 123456
    Int,

    // lexical like 12.3
    Float,

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
}

struct Lexer {
    str: &'static str,
    head: usize,
    now: usize,
    state: State,
    token: Vec<Token>,
}

impl Lexer {
    fn new(s: &'static str) -> Self {
        Self {
            str: s,
            head: 0,
            now: 0,
            state: State::Init,
            token: vec![],
        }
    }

    fn token(&mut self) {
        loop {
            match self.state {
                State::Init => match &self.str.chars().next().unwrap() {
                    '{' => {
                        let t = Token::new(TokenType::LeftBigBracket, self.head, self.now);
                        self.token.push(t);
                        self.head += 1;
                        self.now += 1;
                    },
                    '0'..='9' => {
                        self.state = State::Int;
                        self.now += 1;
                    },
                    '"' => {
                        self.state = State::String;
                        self.now += 1;
                    },
                    't' => {
                        self.state = State::True;
                        self.now += 1;
                    },
                    'f' => {
                        self.state = State::False;
                        self.now += 1;
                    },
                    'n' => {
                        self.state = State::Null;
                        self.now += 1;
                    },
                    _ => println!("tt"),
                }
                _ => println!("others"),
            }
        }
    }
}
