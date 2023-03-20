use std::ops::Range;

struct Token {
    token_type: TokenType,
    range: Range<usize>,
}

impl Token {
    fn new() -> Self {
        Self {
            token_type: TokenType::Int,
            range: 0..0,
        }
    }
}
#[derive(PartialEq)]

enum TokenState {
    Init,
    Recoginzing,
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

    // lexical like false or true
    Bool,

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

    // lexical " "
    WhiteSpace,
}

struct Lexer {
    str: &'static str,
    head: usize,
    state: TokenState,
}

impl Lexer {
    fn new(s: &'static str) -> Self {
        Self {
            str: s,
            head: 0,
            state: TokenState::Init,
        }
    }

    fn parse_colon(&mut self) -> Option<Token> {
        if self.state == TokenState::Init {
            self.head += 1;
            Some(Token::new())
        } else {
            None
        }
    }

    fn parse_comma(&mut self) -> Option<Token> {
        if self.state == TokenState::Init {
            self.head += 1;
            Some(Token::new())
        } else {
            None
        }
    }

    fn parse_big_left_bracket(&mut self) -> Option<Token> {
        if self.state == TokenState::Init {
            self.head += 1;
            Some(Token::new())
        } else {
            None
        }
    }

    fn parse_big_right_bracket(&mut self) -> Option<Token> {
        if self.state == TokenState::Init {
            self.head += 1;
            Some(Token::new())
        } else {
            None
        }
    }

    fn parse_middle_left_bracket(&mut self) -> Option<Token> {
        if self.state == TokenState::Init {
            self.head += 1;
            Some(Token::new())
        } else {
            None
        }
    }

    fn parse_middle_right_bracket(&mut self) -> Option<Token> {
        if self.state == TokenState::Init {
            self.head += 1;
            Some(Token::new())
        } else {
            None
        }
    }
}
