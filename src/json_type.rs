trait JsonValue {
    fn value(self) -> JsonType;
}

enum JsonType {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Null(()),
    Array(Vec<Box<dyn JsonValue>>),
    JsonObject(JsonObject)
}

struct Int {
    inner: i64,
}

impl Int {
    fn new(v: i64) -> Self {
        Self { inner: v }
    }
}

impl JsonValue for Int {
    fn value(self) -> JsonType {
        JsonType::Int(self.inner)
    }
}

struct Float {
    inner: f64,
}

impl Float {
    fn new(f: f64) -> Self {
        Self { inner: f }
    }
}

impl JsonValue for Float {
    fn value(self) -> JsonType {
        JsonType::Float(self.inner)
    }
}

struct Str {
    inner: String,
}

impl Str {
    fn new(s: String) -> Self {
        Self { inner: s }
    }
}

impl JsonValue for Str {
    fn value(self) -> JsonType {
        JsonType::Str(self.inner)
    }
}

struct Bool {
    inner: bool,
}

impl Bool {
    fn new(b: bool) -> Self {
        Self { inner: b }
    }
}

impl JsonValue for Bool {
    fn value(self) -> JsonType {
        JsonType::Bool(self.inner)
    }
}

struct Null {
    inner: (),
}

impl Null {
    fn new(k: ()) -> Self {
        Self { inner: () }
    }
}

impl JsonValue for Null {
    fn value(self) -> JsonType {
        JsonType::Null(())
    }
}

struct Array {
    inner: Vec<Box<dyn JsonValue>>,
}

impl Array {
    fn new(v: Vec<Box<dyn JsonValue>>) -> Self {
        Self { inner: v }
    }
}

impl JsonValue for Array {
    fn value(self) -> JsonType {
        JsonType::Array(self.inner)
    }
}

struct JsonObject {
    key: Vec<String>,
    value: Vec<Box<dyn JsonValue>>,
}

impl JsonObject {
    fn new() -> Self {
        Self {
            key: vec![],
            value: vec![],
        }
    }
}

impl JsonValue for JsonObject {
    fn value(self) -> JsonType{
        JsonType::JsonObject(self)
    }
}
