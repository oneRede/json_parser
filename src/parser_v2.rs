use crate::lexer::Token;
struct Parser {
    tokens: Vec<Token>,
}

trait JsonValue {
    fn value(&self);
}

struct JsonObject {
    keys: Vec<String>,
    values: Vec<Box<dyn JsonValue>>
}