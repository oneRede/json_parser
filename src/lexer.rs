use std::ops::Range;

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    range: Range<usize>,
}

impl Token {
    fn new(token_type: TokenType, head: usize, tail: usize) -> Self {
        Self {
            token_type: token_type,
            range: head..tail,
        }
    }
}
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
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
#[derive(Debug)]
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
                    '0'..='9' => {
                        println!("int");
                        self.state = State::Int;
                        self.now += 1;
                    }
                    '"' => {
                        self.state = State::String;
                        self.now += 1;
                    }
                    't' => {
                        self.state = State::True;
                        self.now += 1;
                    }
                    'f' => {
                        self.state = State::False;
                        self.now += 1;
                    }
                    'n' => {
                        self.state = State::Null;
                        self.now += 1;
                    }
                    _ => {
                        println!("init error");
                        return ();
                    }
                },
                State::Int => match chars.next().unwrap() {
                    '0'..='9' => {
                        println!("goon int");
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
                    _ => {
                        println!("state {:?}", self.state);
                        println!("int error");
                        return ();
                    }
                },
                _ => println!("others"),
            }
        }
    }
}

#[test]
fn test_empty_josn() {
    let s = "{12345678}";
    let mut lexer = Lexer::new(s);
    lexer.token();
    println!("{:?}", lexer.token);
    println!("{:?}", &s[0..1]);
}
