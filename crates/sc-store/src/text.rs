//! The canonical text form: a strict JSON subset with one way to write any value.
use core::fmt;
use std::collections::BTreeMap;

/// How deeply lists and objects may nest. Parsing recurses, so this bounds the stack.
const MAX_DEPTH: usize = 64;

/// One value in a canonical text file. Objects keep their keys in byte order, so the form cannot vary.
///
/// There is no floating point. A number with a fraction is stored as text in the format its field declares.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    /// `null`.
    Null,
    /// `true` or `false`.
    Bool(bool),
    /// A whole number.
    Int(i64),
    /// A string.
    Text(String),
    /// A list, in authored order.
    List(Vec<Value>),
    /// An object.
    Object(BTreeMap<String, Value>),
}

impl Value {
    /// The one canonical form: sorted keys, two-space indentation, LF line ends and a final newline.
    #[must_use]
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        write_value(self, 0, &mut out);
        out.push('\n');
        out
    }

    /// The string inside `Text`.
    #[must_use]
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }

    /// The number inside `Int`.
    #[must_use]
    pub const fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(number) => Some(*number),
            _ => None,
        }
    }

    /// The items inside `List`.
    #[must_use]
    pub fn as_list(&self) -> Option<&[Value]> {
        match self {
            Self::List(items) => Some(items),
            _ => None,
        }
    }

    /// The fields inside `Object`.
    #[must_use]
    pub const fn as_object(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Self::Object(fields) => Some(fields),
            _ => None,
        }
    }
}

fn write_value(value: &Value, depth: usize, out: &mut String) {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Int(number) => out.push_str(&number.to_string()),
        Value::Text(text) => write_string(text, out),
        Value::List(items) if items.is_empty() => out.push_str("[]"),
        Value::List(items) => {
            out.push_str("[\n");
            for (at, item) in items.iter().enumerate() {
                indent(depth + 1, out);
                write_value(item, depth + 1, out);
                out.push_str(if at + 1 == items.len() { "\n" } else { ",\n" });
            }
            indent(depth, out);
            out.push(']');
        }
        Value::Object(fields) if fields.is_empty() => out.push_str("{}"),
        Value::Object(fields) => {
            out.push_str("{\n");
            let last = fields.len();
            for (at, (key, item)) in fields.iter().enumerate() {
                indent(depth + 1, out);
                write_string(key, out);
                out.push_str(": ");
                write_value(item, depth + 1, out);
                out.push_str(if at + 1 == last { "\n" } else { ",\n" });
            }
            indent(depth, out);
            out.push('}');
        }
    }
}

fn indent(depth: usize, out: &mut String) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

fn write_string(text: &str, out: &mut String) {
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            other if u32::from(other) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", u32::from(other)));
            }
            other => out.push(other),
        }
    }
    out.push('"');
}

/// Why a file is not canonical text, with where the reader stopped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextError {
    /// 1-based line.
    pub line: usize,
    /// 1-based column, counted in characters.
    pub column: usize,
    /// What is wrong, in a sentence.
    pub message: String,
}

impl fmt::Display for TextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for TextError {}

/// Read one value. Whitespace and key order may vary, and the next save writes the canonical form.
///
/// # Errors
/// Refuses anything else: a fraction or exponent, a number outside 64 bits, a repeated key, a lone surrogate,
/// nesting past 64 levels, or text after the value.
pub fn parse(text: &str) -> Result<Value, TextError> {
    let mut reader = Reader {
        characters: text.chars().collect(),
        at: 0,
    };
    reader.skip_space();
    let value = reader.value(0)?;
    reader.skip_space();
    if reader.peek().is_some() {
        return Err(reader.error("there is text after the value"));
    }
    Ok(value)
}

struct Reader {
    characters: Vec<char>,
    at: usize,
}

impl Reader {
    fn peek(&self) -> Option<char> {
        self.characters.get(self.at).copied()
    }

    fn next(&mut self) -> Option<char> {
        let character = self.peek();
        if character.is_some() {
            self.at += 1;
        }
        character
    }

    fn error(&self, message: &str) -> TextError {
        let (mut line, mut column) = (1, 1);
        for character in self.characters.iter().take(self.at) {
            if *character == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        TextError {
            line,
            column,
            message: message.to_owned(),
        }
    }

    fn skip_space(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t' | '\n' | '\r')) {
            self.at += 1;
        }
    }

    fn value(&mut self, depth: usize) -> Result<Value, TextError> {
        if depth > MAX_DEPTH {
            return Err(self.error("lists and objects nest more than 64 levels"));
        }
        match self.peek() {
            Some('{') => self.object(depth),
            Some('[') => self.list(depth),
            Some('"') => self.string().map(Value::Text),
            Some('t') => self.word("true", Value::Bool(true)),
            Some('f') => self.word("false", Value::Bool(false)),
            Some('n') => self.word("null", Value::Null),
            Some('-' | '0'..='9') => self.number(),
            Some(_) => Err(self.error("a value must start here")),
            None => Err(self.error("the file ends where a value should start")),
        }
    }

    fn word(&mut self, word: &str, value: Value) -> Result<Value, TextError> {
        for expected in word.chars() {
            if self.peek() != Some(expected) {
                return Err(self.error("this is not a value"));
            }
            self.at += 1;
        }
        Ok(value)
    }

    fn number(&mut self) -> Result<Value, TextError> {
        let start = self.at;
        if self.peek() == Some('-') {
            self.at += 1;
        }
        let first_digit = self.at;
        while matches!(self.peek(), Some('0'..='9')) {
            self.at += 1;
        }
        if self.at == first_digit {
            return Err(self.error("a number needs a digit"));
        }
        if self.at - first_digit > 1 && self.characters.get(first_digit) == Some(&'0') {
            self.at = first_digit;
            return Err(self.error("a number cannot have a leading zero"));
        }
        if matches!(self.peek(), Some('.' | 'e' | 'E')) {
            return Err(self.error(
                "numbers here are whole; a fraction is stored as text in the format its field declares",
            ));
        }
        let digits: String = self
            .characters
            .iter()
            .skip(start)
            .take(self.at - start)
            .collect();
        if digits == "-0" {
            self.at = start;
            return Err(self.error("negative zero has no canonical form; write 0"));
        }
        digits.parse().map(Value::Int).map_err(|_| {
            self.at = start;
            self.error("this number does not fit in 64 bits")
        })
    }

    fn string(&mut self) -> Result<String, TextError> {
        self.at += 1;
        let mut out = String::new();
        loop {
            match self.next() {
                None => return Err(self.error("the file ends inside a string")),
                Some('"') => return Ok(out),
                Some('\\') => out.push(self.escape()?),
                Some(character) if u32::from(character) < 0x20 => {
                    self.at -= 1;
                    return Err(self.error("a control character in a string must be escaped"));
                }
                Some(character) => out.push(character),
            }
        }
    }

    fn escape(&mut self) -> Result<char, TextError> {
        match self.next() {
            Some('"') => Ok('"'),
            Some('\\') => Ok('\\'),
            Some('/') => Ok('/'),
            Some('n') => Ok('\n'),
            Some('r') => Ok('\r'),
            Some('t') => Ok('\t'),
            Some('b') => Ok('\u{8}'),
            Some('f') => Ok('\u{c}'),
            Some('u') => self.unicode_escape(),
            _ => Err(self.error("this is not a string escape")),
        }
    }

    fn unicode_escape(&mut self) -> Result<char, TextError> {
        let first = self.four_hex()?;
        let lone = "a surrogate must be half of a pair";
        if (0xDC00..0xE000).contains(&first) {
            return Err(self.error(lone));
        }
        let scalar = if (0xD800..0xDC00).contains(&first) {
            if self.next() != Some('\\') || self.next() != Some('u') {
                return Err(self.error(lone));
            }
            let second = self.four_hex()?;
            if !(0xDC00..0xE000).contains(&second) {
                return Err(self.error(lone));
            }
            0x1_0000 + ((first - 0xD800) << 10) + (second - 0xDC00)
        } else {
            first
        };
        char::from_u32(scalar).ok_or_else(|| self.error(lone))
    }

    fn four_hex(&mut self) -> Result<u32, TextError> {
        let mut value = 0;
        for _ in 0..4 {
            let digit = self
                .next()
                .and_then(|character| character.to_digit(16))
                .ok_or_else(|| self.error("\\u needs four hexadecimal digits"))?;
            value = value * 16 + digit;
        }
        Ok(value)
    }

    fn list(&mut self, depth: usize) -> Result<Value, TextError> {
        self.at += 1;
        let mut items = Vec::new();
        self.skip_space();
        if self.peek() == Some(']') {
            self.at += 1;
            return Ok(Value::List(items));
        }
        loop {
            self.skip_space();
            items.push(self.value(depth + 1)?);
            self.skip_space();
            match self.next() {
                Some(',') => {}
                Some(']') => return Ok(Value::List(items)),
                _ => return Err(self.error("a list continues with a comma or ends with ]")),
            }
        }
    }

    fn object(&mut self, depth: usize) -> Result<Value, TextError> {
        self.at += 1;
        let mut fields = BTreeMap::new();
        self.skip_space();
        if self.peek() == Some('}') {
            self.at += 1;
            return Ok(Value::Object(fields));
        }
        loop {
            self.skip_space();
            if self.peek() != Some('"') {
                return Err(self.error("an object key must be a string"));
            }
            let at_key = self.at;
            let key = self.string()?;
            if fields.contains_key(&key) {
                self.at = at_key;
                return Err(self.error("this key appears twice in the object"));
            }
            self.skip_space();
            if self.next() != Some(':') {
                return Err(self.error("an object key is followed by a colon"));
            }
            self.skip_space();
            let value = self.value(depth + 1)?;
            fields.insert(key, value);
            self.skip_space();
            match self.next() {
                Some(',') => {}
                Some('}') => return Ok(Value::Object(fields)),
                _ => return Err(self.error("an object continues with a comma or ends with }")),
            }
        }
    }
}
