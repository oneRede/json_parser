use std::collections::HashMap;

use crate::lexer::Token;
use crate::lexer::TokenType;

#[allow(unused)]
#[derive(Eq, Hash, PartialEq)]
enum Symbol {
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
    // J
    J,
    // A
    A,
    // K
    K,
    // V
    V,
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
#[derive(Eq, Hash, PartialEq, Debug)]
#[derive(Copy, Clone)]
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
    ACC,
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
#[derive(Clone)]
#[derive(Debug, PartialEq)]
struct Stack {
    data: Vec<State>,
    head: usize,
}

#[allow(unused)]
impl Stack {
    fn new() -> Self {
        Self {
            data: vec![],
            head: 0,
        }
    }

    fn push(&mut self, state: State) {
        self.data.push(state);
        self.head += 1;
    }

    fn pop(&mut self) -> Option<State> {
        self.head -= 1;
        let state = self.data.pop();
        state
    }

    fn get_head_state(&mut self) -> Option<State> {
        return Some(self.data[self.head].clone());
    }
}

#[allow(unused)]
struct Parser {
    action: HashMap<State, HashMap<TokenType, (Action, State, NonTerminal)>>,
    goto: HashMap<State, HashMap<NonTerminal, State>>,
    tokens: Vec<Token>,
    head_token: usize,
    state_stack: Stack,
    symbol: Vec<Symbol>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        let mut action: HashMap<State, HashMap<TokenType, (Action, State, NonTerminal)>> =
            HashMap::new();

        let mut i0: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i0.insert(
            TokenType::LeftBigBracket,
            (Action::Shift, State::I2, NonTerminal::Null),
        );
        action.insert(State::I0, i0);

        let mut i1: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i1.insert(
            TokenType::End,
            (Action::Reduction, State::Null, NonTerminal::ACC),
        );
        action.insert(State::I0, i1);

        let mut i2: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i2.insert(
            TokenType::String,
            (Action::Shift, State::I5, NonTerminal::Null),
        );
        action.insert(State::I0, i2);

        let mut i3: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i3.insert(
            TokenType::RightBigBracket,
            (Action::Shift, State::I6, NonTerminal::Null),
        );
        i3.insert(
            TokenType::Comma,
            (Action::Shift, State::I7, NonTerminal::Null),
        );
        action.insert(State::I0, i3);

        let mut i4: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i4.insert(
            TokenType::Colon,
            (Action::Shift, State::I8, NonTerminal::Null),
        );
        action.insert(State::I0, i4);

        let mut i5: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i5.insert(
            TokenType::Colon,
            (Action::Reduction, State::Null, NonTerminal::K),
        );
        action.insert(State::I0, i5);

        let mut i6: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i6.insert(
            TokenType::End,
            (Action::Reduction, State::Null, NonTerminal::J),
        );
        action.insert(State::I0, i6);

        let mut i7: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i7.insert(
            TokenType::String,
            (Action::Shift, State::I5, NonTerminal::Null),
        );
        action.insert(State::I0, i7);

        let mut i8: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i8.insert(
            TokenType::Int,
            (Action::Shift, State::I11, NonTerminal::Null),
        );
        action.insert(State::I0, i8);

        let mut i9: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i9.insert(
            TokenType::RightBigBracket,
            (Action::Reduction, State::Null, NonTerminal::A),
        );
        i9.insert(
            TokenType::Comma,
            (Action::Shift, State::I7, NonTerminal::Null),
        );
        action.insert(State::I0, i9);

        let mut i10: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i10.insert(
            TokenType::RightBigBracket,
            (Action::Reduction, State::Null, NonTerminal::A),
        );
        i10.insert(
            TokenType::Comma,
            (Action::Reduction, State::Null, NonTerminal::A),
        );
        action.insert(State::I0, i10);

        let mut i11: HashMap<TokenType, (Action, State, NonTerminal)> = HashMap::new();
        i11.insert(
            TokenType::RightBigBracket,
            (Action::Reduction, State::Null, NonTerminal::V),
        );
        i11.insert(
            TokenType::Comma,
            (Action::Reduction, State::Null, NonTerminal::V),
        );
        action.insert(State::I0, i11);

        let mut goto: HashMap<State, HashMap<NonTerminal, State>> = HashMap::new();

        let mut i0: HashMap<NonTerminal, State> = HashMap::new();
        i0.insert(NonTerminal::J, State::I1);
        goto.insert(State::I0, i0);

        let mut i2: HashMap<NonTerminal, State> = HashMap::new();
        i2.insert(NonTerminal::A, State::I3);
        i2.insert(NonTerminal::K, State::I4);
        goto.insert(State::I0, i2);

        let mut i7: HashMap<NonTerminal, State> = HashMap::new();
        i7.insert(NonTerminal::A, State::I9);
        i7.insert(NonTerminal::K, State::I4);
        goto.insert(State::I0, i7);

        let mut i8: HashMap<NonTerminal, State> = HashMap::new();
        i8.insert(NonTerminal::V, State::I10);
        goto.insert(State::I0, i8);

        Self {
            action: action,
            goto: goto,
            tokens: tokens,
            head_token: 0,
            state_stack: Stack::new(),
            symbol: vec![],
        }
    }

    fn parse(&mut self) -> Result<String, String> {
        loop {
            if self.state_stack.head == 0 {
                self.state_stack.push(State::I0);
                
            }
            let token = self.tokens.get(self.head_token).unwrap();
            let state = self.state_stack.get_head_state().unwrap();
            let (action, state, non_terminal) = self.action(&state, &token.token_type).unwrap();
            if state.eq(&State::ACC){
                return Ok("ok and over!!".to_string())
            }
            match action {
                Action::Shift => {
                    self.state_stack.push(*state);
                    continue;
                }
                Action::Reduction => {
                    self.state_stack.pop();
                    let head_state = self.state_stack.get_head_state().unwrap();
                    // let next_state = self.goto(head_state, non_terminal);
                    // self.state_stack.push(next_state);
                    continue
                }
                _ => {
                    print!("")
                }
            }
            return Ok("Ok".to_string());
        }
    }

    fn action(
        &self,
        state: &State,
        token_type: &TokenType,
    ) -> Option<&(Action, State, NonTerminal)> {
        if self.action.contains_key(&state) {
            if self.action[&state].contains_key(token_type) {
                return Some(&self.action[&state][token_type]);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn goto(&self, state: &State, non_terminal: &NonTerminal) -> Option<&State> {
        if self.goto.contains_key(&state) {
            if self.goto[&state].contains_key(non_terminal) {
                return Some(&self.goto[&state][non_terminal]);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn get_symbol_lens(&self, non_terminal: NonTerminal) -> Option<usize> {
        match non_terminal {
            NonTerminal::J => Some(3),
            NonTerminal::A => Some(3),
            NonTerminal::K => Some(1),
            NonTerminal::V => Some(1),
            _ => None,
        }
    }
}
