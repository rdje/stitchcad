fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    for (name, n, d) in [
        ("negative i128 endpoint", i128::MIN, 1),
        ("positive i128 endpoint", i128::MAX, 1),
        ("positive magnitude 2^127", i128::MIN, -1),
        ("negative i64 endpoint", i128::from(i64::MIN), 1),
        ("positive control", 1, 2),
        ("negative control", -1, 2),
    ] {
        println!(
            "{name}: {:?}",
            std::panic::catch_unwind(|| sc_units::round::div_round_half_away_from_zero(n, d))
        );
    }
}
