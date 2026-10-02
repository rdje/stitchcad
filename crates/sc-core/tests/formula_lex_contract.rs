//! Contract oracles for borrowed machine-form lexing, independent of expression evaluation.
use sc_core::name::MachineToken;
use sc_core::recipe::{FormulaLexemeKind as K, FormulaLexer, FormulaLexicalRule as R};

fn scan(source: &str) -> Vec<(K, &str, usize, usize)> {
    FormulaLexer::new(source)
        .filter_map(|item| {
            assert!(item.is_ok(), "assertion: machine lexical form: {item:?}");
            let token = item.ok()?;
            Some((
                token.kind(),
                token.text(),
                token.span().start(),
                token.span().end(),
            ))
        })
        .collect()
}

#[test]
fn binding_keeps_exact_roles_text_and_byte_positions() {
    assert_eq!(
        scan("let x: length = - 01.50 cm"),
        vec![
            (K::Let, "let", 0, 3),
            (K::Identifier, "x", 4, 5),
            (K::Colon, ":", 5, 6),
            (K::Identifier, "length", 7, 13),
            (K::Assign, "=", 14, 15),
            (K::Minus, "-", 16, 17),
            (K::Number, "01.50", 18, 23),
            (K::Identifier, "cm", 24, 26),
        ]
    );
}

#[test]
fn paired_comparisons_use_longest_match() {
    let actual: Vec<_> = scan("= == != < <= > >= + - * / ^ : , ( )")
        .into_iter()
        .map(|(kind, _, _, _)| kind)
        .collect();
    assert_eq!(
        actual,
        vec![
            K::Assign,
            K::Equal,
            K::NotEqual,
            K::Less,
            K::LessEqual,
            K::Greater,
            K::GreaterEqual,
            K::Plus,
            K::Minus,
            K::Multiply,
            K::Divide,
            K::Power,
            K::Colon,
            K::Comma,
            K::LeftParen,
            K::RightParen
        ]
    );
}

#[test]
fn only_three_keywords_are_reserved_and_spelling_matches_machine_tokens() {
    for (word, kind) in [("let", K::Let), ("assert", K::Assert), ("if", K::If)] {
        assert_eq!(scan(word).first().expect("one word").0, kind);
        assert!(MachineToken::new(word).is_err());
    }
    for word in [
        "let_down",
        "assertion",
        "if_else",
        "length",
        "hypot",
        "cm",
        "eps_num",
        "size_index",
        "is_base_size",
        "a_2",
        "true",
    ] {
        assert!(MachineToken::new(word).is_ok());
        assert_eq!(scan(word).first().expect("one word").0, K::Identifier);
    }
    for word in ["A", "waistGirth", "_a", "a_", "a__b"] {
        assert!(MachineToken::new(word).is_err());
        let result = FormulaLexer::new(word).next().expect("item");
        assert!(result.is_err(), "assertion: spelling must refuse: {word}");
        let error = result.expect_err("invalid spelling");
        assert_eq!(error.rule(), R::IdentifierSpelling);
        assert_eq!((error.span().start(), error.span().end()), (0, word.len()));
    }
}

#[test]
fn numbers_are_borrowed_without_value_conversion() {
    let source = "000001.0500 99999999999999999999999999999999999999999999999";
    for item in FormulaLexer::new(source) {
        let token = item.expect("no numeric bound at lexical stage");
        assert_eq!(token.kind(), K::Number);
        assert_eq!(
            token.text().as_ptr(),
            source[token.span().start()..].as_ptr()
        );
        assert_eq!(
            token.text(),
            &source[token.span().start()..token.span().end()]
        );
    }
    assert_eq!(scan(source).first().expect("number").1, "000001.0500");
}

#[test]
fn whitespace_gaps_preserve_later_unit_separator_authority() {
    let source = "\t1  cm\n2\rcm\x0b3\x0ccm";
    let tokens = scan(source);
    assert_eq!(tokens.len(), 6);
    assert_eq!(
        &source[tokens.first().expect("fixture token").3..tokens.get(1).expect("fixture token").2],
        "  "
    );
    assert_eq!(
        &source[tokens.get(2).expect("fixture token").3..tokens.get(3).expect("fixture token").2],
        "\r"
    );
    assert_eq!(
        &source[tokens.get(4).expect("fixture token").3..tokens.get(5).expect("fixture token").2],
        "\x0c"
    );
    assert_eq!(scan("1 cm").get(1).expect("unit").0, K::Identifier);
}

#[test]
fn unicode_preflight_refuses_before_any_valid_prefix_token() {
    for (source, start, end) in [
        ("let x = 1 cm é", 13, 15),
        ("√(a)", 0, 3),
        ("1 °", 2, 4),
        ("a²", 1, 3),
        ("a\u{00a0}b", 1, 3),
    ] {
        let mut lexer = FormulaLexer::new(source);
        let result = lexer.next().expect("preflight error");
        assert!(
            result.is_err(),
            "assertion: ASCII preflight must precede tokens"
        );
        let error = result.expect_err("machine ASCII required");
        assert_eq!(error.rule(), R::MachineAscii);
        assert_eq!((error.span().start(), error.span().end()), (start, end));
        assert_eq!(error.diagnostic_code(), "formula_parse");
        assert_eq!(lexer.next(), None);
    }
}

#[test]
fn decimals_require_nonempty_fraction() {
    for source in ["1.", "01. cm", "2..3"] {
        let result = FormulaLexer::new(source).next().expect("item");
        assert!(result.is_err(), "assertion: nonempty fraction required");
        let error = result.expect_err("fraction missing");
        let dot = source.find('.').expect("fixture dot");
        assert_eq!(error.rule(), R::DecimalFraction);
        assert_eq!((error.span().start(), error.span().end()), (dot, dot + 1));
    }
    let error = FormulaLexer::new(".5")
        .next()
        .expect("item")
        .expect_err("no leading digits");
    assert_eq!(error.rule(), R::UnsupportedCharacter);
}

#[test]
fn unsupported_characters_and_lone_bang_refuse_precisely() {
    for (source, rule) in [
        ("!", R::ComparisonPair),
        ("\"wide\"", R::UnsupportedCharacter),
        ("#comment", R::UnsupportedCharacter),
        (";", R::UnsupportedCharacter),
        ("\0", R::UnsupportedCharacter),
        ("[", R::UnsupportedCharacter),
    ] {
        let error = FormulaLexer::new(source)
            .next()
            .expect("item")
            .expect_err("refusal");
        assert_eq!(error.rule(), rule);
        assert_eq!((error.span().start(), error.span().end()), (0, 1));
    }
}

#[test]
fn lexical_success_does_not_certify_expression_syntax() {
    // These are token sequences; whole-form refusals belong to the parser/type/name owner.
    for source in [
        "1e3",
        "1 2",
        "// comment",
        "a < b < c",
        "if(a, b)",
        "a ^ 3",
        "spline(a, 3)",
        "1 alien_unit",
    ] {
        assert!(
            FormulaLexer::new(source).all(|item| item.is_ok()),
            "{source}"
        );
    }
}

#[test]
fn end_and_first_error_are_permanently_fused() {
    fn is_fused<T: std::iter::FusedIterator>(_: &T) {}
    for source in ["", " \t\n", "a", "# a", "1. a"] {
        let mut lexer = FormulaLexer::new(source);
        is_fused(&lexer);
        for item in lexer.by_ref() {
            if item.is_err() {
                break;
            }
        }
        assert_eq!(lexer.next(), None, "{source:?}");
        assert_eq!(lexer.next(), None, "{source:?}");
    }
}

#[test]
fn clones_continue_independently_at_original_positions() {
    let mut first = FormulaLexer::new("a + b");
    assert_eq!(first.next().expect("a").expect("token").text(), "a");
    let mut second = first.clone();
    assert_eq!(first.next().expect("+").expect("token").span().start(), 2);
    assert_eq!(first.next().expect("b").expect("token").text(), "b");
    assert_eq!(
        second.next().expect("independent +").expect("token").text(),
        "+"
    );
    assert_eq!(second.next().expect("b").expect("token").span().start(), 4);
}

#[test]
fn debug_and_errors_do_not_dump_customer_source() {
    let source = "customer_secret + CUSTOMER_SECRET";
    let mut lexer = FormulaLexer::new(source);
    assert!(!format!("{lexer:?}").contains("customer_secret"));
    let _ = lexer.next();
    let _ = lexer.next();
    let error = lexer
        .next()
        .expect("invalid identifier")
        .expect_err("uppercase");
    for output in [
        format!("{lexer:?}"),
        format!("{error:?}"),
        error.to_string(),
    ] {
        assert!(!output.contains("customer_secret"));
        assert!(!output.contains("CUSTOMER_SECRET"));
    }
}

#[test]
fn all_worked_machine_examples_are_scanned_without_evaluating_them() {
    let examples = include_str!("../../../docs/book/src/spec/formula-language/examples.md");
    let mut section = "";
    let (mut bindings, mut assertions, mut refusals) = (0, 0, 0);
    for line in examples.lines() {
        if line.starts_with("## ") {
            section = line;
        }
        if !line.starts_with("| `") {
            continue;
        }
        let cells: Vec<_> = line.split('|').map(str::trim).collect();
        let machine = if section.starts_with("## 2.") {
            bindings += 1;
            format!(
                "let {}: {} = {}",
                cells.get(1).expect("example column").trim_matches('`'),
                cells.get(2).expect("example column"),
                cells.get(3).expect("example column").trim_matches('`')
            )
        } else if section.starts_with("## 3.") {
            assertions += 1;
            cells
                .get(1)
                .expect("example column")
                .trim_matches('`')
                .to_owned()
        } else if section.starts_with("## 4.") {
            refusals += 1;
            cells
                .get(1)
                .expect("example column")
                .split('`')
                .nth(1)
                .expect("machine refusal")
                .to_owned()
        } else {
            continue;
        };
        let result = FormulaLexer::new(&machine).collect::<Result<Vec<_>, _>>();
        if machine == "\"wide\"" {
            assert_eq!(
                result.expect_err("text is not machine syntax").rule(),
                R::UnsupportedCharacter
            );
        } else {
            assert!(result.is_ok(), "lexical form only: {machine}: {result:?}");
        }
    }
    assert_eq!((bindings, assertions, refusals), (17, 4, 13));
}
