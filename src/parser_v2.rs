use std::collections::HashMap;

#[allow(unused)]
#[derive(Eq, Hash, PartialEq)]
enum TokenType {
    // lexical like "string"
    String,
    // lexical like 123456
    Int,
    // lexical ":"
    Colon,
    // lexical ","
    Comma,
    // lexical "{"
    LeftBigBracket,
    // lexical "}"
    RightBigBracket,
    // whitespace
    WhiteSpace,
    //End
    End,
}

#[allow(unused)]
#[derive(Eq, Hash, PartialEq)]
enum NonTerminal {
    J,
    A,
    K,
    V,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum Action {
    Shift,
    Reduction,
}

#[allow(unused)]
#[derive(Eq, Hash, PartialEq)]
enum State {
    I0,
    I1,
    I2,
    I3,
    I4,
    I5,
    I6,
    I7,
    I8,
    I9,
    I10,
    I11,
}

#[allow(unused)]
struct Parser {
    action: HashMap<State, HashMap<TokenType, (Action, State)>>,
    goto: HashMap<State, HashMap<NonTerminal, State>>,
}
