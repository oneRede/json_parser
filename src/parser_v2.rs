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

impl TokenType {
    fn all_value() -> [TokenType; 8] {
        [
            TokenType::String,
            TokenType::Int,
            TokenType::Colon,
            TokenType::Comma,
            TokenType::LeftBigBracket,
            TokenType::RightBigBracket,
            TokenType::WhiteSpace,
            TokenType::End,
        ]
    }
}

#[allow(unused)]
#[derive(Eq, Hash, PartialEq)]
enum NonTerminal {
    J,
    A,
    K,
    V,
    ACC,
    Null,
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
    Null,
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

impl State {
    fn all_value() -> [State; 12] {
        [
            State::I0,
            State::I1,
            State::I2,
            State::I3,
            State::I4,
            State::I5,
            State::I6,
            State::I7,
            State::I8,
            State::I9,
            State::I10,
            State::I11,
        ]
    }
}

#[allow(unused)]
struct Parser {
    action: HashMap<State, HashMap<TokenType, (Action, State, NonTerminal)>>,
    goto: HashMap<State, HashMap<NonTerminal, State>>,
}

impl Parser {
    fn new() -> Self {
        let mut action: HashMap<State, HashMap<TokenType, (Action, State, NonTerminal)>> = HashMap::new();

        let mut i0: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i0.insert(TokenType::LeftBigBracket, (Action::Shift, State::I2, NonTerminal::Null));
        action.insert(State::I0, i0);

        let mut i1: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i1.insert(TokenType::End, (Action::Reduction, State::Null, NonTerminal::ACC));
        action.insert(State::I0, i1);

        let mut i2: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i2.insert(TokenType::String, (Action::Shift, State::I5, NonTerminal::Null));
        action.insert(State::I0, i2);

        let mut i3: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i3.insert(TokenType::RightBigBracket, (Action::Shift, State::I6, NonTerminal::Null));
        i3.insert(TokenType::Comma, (Action::Shift, State::I7, NonTerminal::Null));
        action.insert(State::I0, i3);

        let mut i4: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i4.insert(TokenType::Colon, (Action::Shift, State::I8, NonTerminal::Null));
        action.insert(State::I0, i4);

        let mut i5: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i5.insert(TokenType::Colon, (Action::Reduction, State::Null, NonTerminal::K));
        action.insert(State::I0, i5);

        let mut i6: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i6.insert(TokenType::End, (Action::Reduction, State::Null, NonTerminal::J));
        action.insert(State::I0, i6);

        let mut i7: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i7.insert(TokenType::String, (Action::Shift, State::I5, NonTerminal::Null));
        action.insert(State::I0, i7);

        let mut i8: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i8.insert(TokenType::Int, (Action::Shift, State::I11, NonTerminal::Null));
        action.insert(State::I0, i8);

        let mut i9: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i9.insert(TokenType::RightBigBracket, (Action::Reduction, State::Null, NonTerminal::A));
        i9.insert(TokenType::Comma, (Action::Shift, State::I7, NonTerminal::Null));
        action.insert(State::I0, i9);

        let mut i10: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i10.insert(TokenType::RightBigBracket, (Action::Reduction, State::Null, NonTerminal::A));
        i10.insert(TokenType::Comma, (Action::Reduction, State::Null, NonTerminal::A));
        action.insert(State::I0, i10);

        let mut i11: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i11.insert(TokenType::RightBigBracket, (Action::Reduction, State::Null, NonTerminal::V));
        i11.insert(TokenType::Comma, (Action::Reduction, State::Null, NonTerminal::V));
        action.insert(State::I0, i11);

        Self {
            action: action,
            goto: HashMap::new(),
        }
    }
}
