//! The canonical text form has one way to write any value, and refuses what it cannot keep exactly.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::cast_possible_truncation,
    clippy::indexing_slicing
)]
use sc_store::{parse, Value};
use std::collections::BTreeMap;

fn object(pairs: Vec<(&str, Value)>) -> Value {
    Value::Object(
        pairs
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect(),
    )
}

#[test]
fn the_canonical_form_is_exact() {
    let value = object(vec![
        ("zebra", Value::Int(-7)),
        (
            "apple",
            Value::List(vec![
                Value::Bool(true),
                Value::Null,
                Value::Text("a\"b\\c\n\u{1}é".to_owned()),
            ]),
        ),
        ("empty_list", Value::List(vec![])),
        ("empty_object", object(vec![])),
    ]);
    assert_eq!(
        value.to_text(),
        "{\n  \"apple\": [\n    true,\n    null,\n    \"a\\\"b\\\\c\\n\\u0001é\"\n  ],\n  \"empty_list\": [],\n  \"empty_object\": {},\n  \"zebra\": -7\n}\n"
    );
}

#[test]
fn key_order_and_spacing_do_not_change_the_saved_form() {
    let first = parse("{\"b\":1,\"a\":[1,2]}").expect("valid");
    let second = parse(" {\n \"a\" : [ 1 , 2 ] ,\r\n\t\"b\" : 1 } ").expect("valid");
    assert_eq!(first, second);
    assert_eq!(first.to_text(), second.to_text());
    assert_eq!(
        first.to_text(),
        "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": 1\n}\n"
    );
}

#[test]
fn escapes_read_back_as_their_characters() {
    let value = parse(r#""\"\\\/\b\f\n\r\t\u0041\u00e9\ud83d\ude00""#).expect("valid");
    assert_eq!(value, Value::Text("\"\\/\u{8}\u{c}\n\r\tAé😀".to_owned()));
    assert_eq!(parse(&value.to_text()).expect("its own form"), value);
}

#[test]
fn what_cannot_be_kept_exactly_is_refused_with_where_and_why() {
    for (text, words, line, column) in [
        ("1.5", "numbers here are whole", 1, 2),
        ("1e3", "numbers here are whole", 1, 2),
        ("[\n  2.0\n]", "numbers here are whole", 2, 4),
        ("9223372036854775808", "does not fit in 64 bits", 1, 1),
        ("-0", "negative zero", 1, 1),
        ("01", "leading zero", 1, 1),
        ("-", "needs a digit", 1, 2),
        ("{\"a\":1,\"a\":2}", "appears twice", 1, 8),
        ("\"\\ud800\"", "half of a pair", 1, 9),
        ("\"\\udc00\"", "half of a pair", 1, 8),
        ("\"\\ud800x\"", "half of a pair", 1, 9),
        ("\"\\ud800\\u0041\"", "half of a pair", 1, 14),
        ("\"\\q\"", "not a string escape", 1, 4),
        ("\"\\u12\"", "four hexadecimal", 1, 7),
        ("\"a\nb\"", "must be escaped", 1, 3),
        ("\"abc", "ends inside a string", 1, 5),
        ("1 2", "text after the value", 1, 3),
        ("", "ends where a value", 1, 1),
        ("[1,]", "a value must start", 1, 4),
        ("[1 2]", "ends with ]", 1, 5),
        ("{\"a\" 1}", "followed by a colon", 1, 7),
        ("{a:1}", "key must be a string", 1, 2),
        ("{\"a\":1 \"b\":2}", "ends with }", 1, 9),
        ("nul", "not a value", 1, 4),
        ("tru", "not a value", 1, 4),
    ] {
        let Err(error) = parse(text) else {
            panic!("{text:?} must be refused")
        };
        assert!(error.message.contains(words), "{text:?}: {error}");
        assert_eq!(
            (error.line, error.column),
            (line, column),
            "{text:?}: {error}"
        );
    }
}

#[test]
fn the_extremes_of_a_number_survive() {
    for text in ["9223372036854775807", "-9223372036854775808", "0"] {
        assert_eq!(
            parse(text).expect("in range").to_text(),
            format!("{text}\n")
        );
    }
}

#[test]
fn nesting_past_the_limit_is_refused_rather_than_overflowing_the_stack() {
    let deep = |levels: usize| format!("{}1{}", "[".repeat(levels), "]".repeat(levels));
    assert!(parse(&deep(64)).is_ok());
    let error = parse(&deep(65)).expect_err("too deep");
    assert!(error.message.contains("nest more than 64"), "{error}");
    // Far past the limit, as a hostile file would be.
    assert!(parse(&"[".repeat(2_000_000)).is_err());
}

struct Random(u64);
impl Random {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
}

fn random_text(random: &mut Random) -> String {
    let alphabet: Vec<char> = "ab\"\\/\n\r\t\u{0}\u{1f}\u{7f} é€😀\u{2028}\u{fffd}"
        .chars()
        .collect();
    (0..random.below(8))
        .map(|_| alphabet[random.below(alphabet.len() as u64) as usize])
        .collect()
}

fn random_value(random: &mut Random, depth: u32) -> Value {
    match random.below(if depth == 0 { 4 } else { 6 }) {
        0 => Value::Null,
        1 => Value::Bool(random.below(2) == 0),
        2 => Value::Int(random.next() as i64),
        3 => Value::Text(random_text(random)),
        4 => Value::List(
            (0..random.below(4))
                .map(|_| random_value(random, depth - 1))
                .collect(),
        ),
        _ => {
            let mut fields = BTreeMap::new();
            for _ in 0..random.below(4) {
                fields.insert(random_text(random), random_value(random, depth - 1));
            }
            Value::Object(fields)
        }
    }
}

#[test]
fn every_value_survives_a_round_trip_and_its_form_is_stable() {
    let mut random = Random(1);
    for seed in 0..5_000 {
        let value = random_value(&mut random, 4);
        let text = value.to_text();
        let back = parse(&text).unwrap_or_else(|error| panic!("seed {seed}: {error}\n{text}"));
        assert_eq!(back, value, "seed {seed}");
        assert_eq!(back.to_text(), text, "seed {seed}");
    }
}
