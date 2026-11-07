//! The determinism policy, as far as a test can hold it: no platform math on any product path, and frozen bits for the math that replaces it.
//!
//! The frozen values are checked on every operating system hosted CI runs, so a platform that rounds differently fails here, by name, before it can
//! move a golden file by a micrometre.
#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::cast_precision_loss,
    clippy::cast_lossless
)]
use sc_units::pure;
use std::path::{Path, PathBuf};

// Each comes from a math library or is not pinned down by IEEE 754.
const PLATFORM_MATH: [&str; 18] = [
    ".hypot(",
    ".sin(",
    ".cos(",
    ".tan(",
    ".asin(",
    ".acos(",
    ".atan(",
    ".atan2(",
    ".sin_cos(",
    ".exp(",
    ".exp2(",
    ".ln(",
    ".log(",
    ".log2(",
    ".log10(",
    ".powf(",
    ".powi(",
    ".cbrt(",
];

fn sources(directory: &Path, found: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(directory).expect("reads") {
        let path = entry.expect("an entry").path();
        if path.is_dir() {
            sources(&path, found);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            found.push(path);
        }
    }
}

#[test]
fn no_product_source_calls_platform_math() {
    let crates = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crates directory");
    let mut checked = 0;
    let mut wrong = Vec::new();
    for entry in std::fs::read_dir(crates).expect("reads") {
        let source = entry.expect("an entry").path().join("src");
        if !source.is_dir() {
            continue;
        }
        let mut files = Vec::new();
        sources(&source, &mut files);
        for file in files {
            let text = std::fs::read_to_string(&file).expect("reads");
            checked += 1;
            for (at, line) in text.lines().enumerate() {
                let code = line.split("//").next().unwrap_or("");
                // A Length or an Angle method of the same name is this project's own exact arithmetic, which reaches here through `pure` or integers.
                if PLATFORM_MATH.iter().any(|call| code.contains(call)) && !is_own_method(code) {
                    wrong.push(format!("{}:{}: {}", file.display(), at + 1, line.trim()));
                }
            }
        }
    }
    assert!(
        checked > 50,
        "only {checked} files: the census read the wrong directory"
    );
    assert!(
        wrong.is_empty(),
        "platform math on a product path:\n{}",
        wrong.join("\n")
    );
}

// `self.sin()` on an Angle and friends are defined in terms of `pure`, and are named in this list of exceptions so each is looked at.
fn is_own_method(code: &str) -> bool {
    code.contains("angle.sin(")
        || code.contains("angle.cos(")
        || code.contains("self.sin(")
        || code.contains("self.cos(")
}

#[test]
fn the_census_sees_a_call_when_there_is_one() {
    assert!(PLATFORM_MATH
        .iter()
        .any(|call| "let x = y.hypot(z);".contains(call)));
    assert!(!is_own_method("let x = y.hypot(z);"));
}

// Computed once and frozen: whatever these give on one machine they must give on every machine. A change here is a change to every drawing.
#[test]
fn the_same_bits_on_every_platform() {
    for (what, value, bits) in [
        ("sin(1)", pure::sin(1.0), 0x3fea_ed54_8f09_0cee_u64),
        ("cos(1)", pure::cos(1.0), 0x3fe1_4a28_0fb5_068a),
        ("atan2(1,2)", pure::atan2(1.0, 2.0), 0x3fdd_ac67_0561_bb4f),
        ("acos(0.25)", pure::acos(0.25), 0x3ff5_1700_e0c1_4b25),
        ("hypot(3,4.5)", pure::hypot(3.0, 4.5), 0x4015_a220_7349_0377),
        ("sin(100)", pure::sin(100.0), 0xbfe0_3425_b78c_4d9a),
        ("cos(-3)", pure::cos(-3.0), 0xbfef_ae04_be85_e5d3),
        (
            "atan2(-7,-3)",
            pure::atan2(-7.0, -3.0),
            0xbfff_9c6b_238c_6434,
        ),
    ] {
        assert_eq!(value.to_bits(), bits, "{what} is {value} on this platform");
    }
}

fn close(a: f64, b: f64, within: f64, what: &str) {
    assert!((a - b).abs() <= within, "{what}: {a} against {b}");
}

#[test]
fn it_agrees_with_this_platform_to_a_few_parts_in_ten_to_the_fifteen() {
    let mut seed: u64 = 0x5EED;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed % 2_000_001) as f64 / 1_000_000.0 - 1.0
    };
    for _ in 0..200_000 {
        let (a, b) = (next() * 10.0, next() * 10.0);
        let (sine, cosine) = pure::sin_cos(a);
        close(sine, a.sin(), 4e-15, "sin");
        close(cosine, a.cos(), 4e-15, "cos");
        if a != 0.0 || b != 0.0 {
            close(pure::atan2(a, b), a.atan2(b), 4e-15, "atan2");
        }
        close(pure::hypot(a, b), a.hypot(b), 4e-15, "hypot");
        let inside = next();
        close(pure::acos(inside), inside.acos(), 4e-15, "acos");
    }
}

#[test]
fn it_holds_at_the_places_a_series_is_most_likely_to_go_wrong() {
    use core::f64::consts::{FRAC_PI_2, PI};
    assert_eq!(pure::atan2(0.0, 0.0), 0.0);
    assert_eq!(pure::atan2(0.0, 1.0), 0.0);
    close(pure::atan2(0.0, -1.0), PI, 1e-15, "the back of the circle");
    close(pure::atan2(1.0, 0.0), FRAC_PI_2, 1e-15, "straight up");
    close(pure::atan2(-1.0, 0.0), -FRAC_PI_2, 1e-15, "straight down");
    close(pure::acos(1.0), 0.0, 1e-15, "acos(1)");
    close(pure::acos(-1.0), PI, 1e-15, "acos(-1)");
    close(pure::acos(0.0), FRAC_PI_2, 1e-15, "acos(0)");
    for quarter in -8..=8 {
        let angle = f64::from(quarter) * FRAC_PI_2;
        let (sine, cosine) = pure::sin_cos(angle);
        close(sine, angle.sin(), 2e-15, "sin at a quarter turn");
        close(cosine, angle.cos(), 2e-15, "cos at a quarter turn");
    }
    assert_eq!(pure::hypot(3.0, 4.0), 5.0);
}

// Crates that lie on the path from a design to a written file. `sc-constraints` is not one: it uses hash maps for membership, and nothing it writes comes from their order.
const OUTPUT_PATH: [&str; 8] = [
    "sc-artifacts",
    "sc-grading",
    "sc-cli",
    "sc-geometry",
    "sc-store",
    "sc-units",
    "sc-viewer",
    "sc-viewport",
];

#[test]
fn nothing_on_the_output_path_depends_on_a_clock_chance_or_a_hash_order() {
    let crates = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crates directory");
    let mut wrong = Vec::new();
    for name in OUTPUT_PATH {
        let mut files = Vec::new();
        sources(&crates.join(name).join("src"), &mut files);
        assert!(!files.is_empty(), "{name} has no sources");
        for file in files {
            let text = std::fs::read_to_string(&file).expect("reads");
            for (at, line) in text.lines().enumerate() {
                let code = line.split("//").next().unwrap_or("");
                for call in [
                    "SystemTime::now",
                    "Instant::now",
                    "thread_rng",
                    "getrandom",
                    "HashMap",
                    "HashSet",
                    "std::env::var",
                ] {
                    if code.contains(call) {
                        wrong.push(format!("{}:{}: {call}", file.display(), at + 1));
                    }
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
