//! Shared certified pi endpoint; exact certificate reproduction stays tied to compiled limbs.
pub(super) const PI_FRACTION_BITS: u32 = 640;
// Little-endian base-2^32 limbs, derived from tests/fixtures/pi640_certificate.tsv.
// The fixture certifies these are floor(pi*2^640); arc and trig retain separate rounding proofs.
pub(super) const PI_LOWER_WORDS: [u32; 21] = [
    0x98dfb5ac, 0xd1310ba6, 0x8979fb1b, 0x9216d5d9, 0xb5470917, 0x3f84d5b5, 0xc97c50dd, 0xc0ac29b7,
    0x34e90c6c, 0xbe5466cf, 0x38d01377, 0x452821e6, 0xec4e6c89, 0x082efa98, 0x299f31d0, 0xa4093822,
    0x03707344, 0x13198a2e, 0x85a308d3, 0x243f6a88, 0x00000003,
];

#[cfg(test)]
mod tests {
    use super::{PI_FRACTION_BITS, PI_LOWER_WORDS};
    use num_bigint::BigUint;

    #[test]
    #[allow(clippy::expect_used)]
    fn compiled_pi_endpoint_matches_the_reproduced_certificate() {
        let fixture = include_bytes!("../../tests/fixtures/pi640_certificate.tsv");
        // Cargo runs package tests at the package root. Derive the checkout path at runtime so
        // no checkout-specific path is embedded or persisted for this producer invocation.
        let project = std::env::current_dir()
            .expect("Cargo package working directory")
            .join("../..");
        let output = std::process::Command::new("python3")
            .args([
                "-I",
                "-B",
                "docs/tasks/artifacts/formula_structure/pi_rounding_certificate.py",
            ])
            .current_dir(project)
            .output()
            .expect("independent exact certificate producer");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(output.stdout, fixture);
        let text = core::str::from_utf8(fixture).expect("ASCII certificate");
        let hex = text
            .lines()
            .find_map(|line| line.strip_prefix("pi_lower_hex\t"))
            .expect("certified endpoint");
        let bits: u32 = text
            .lines()
            .find_map(|line| line.strip_prefix("fraction_bits\t"))
            .expect("certificate scale")
            .parse()
            .expect("integer bits");
        assert_eq!(bits, PI_FRACTION_BITS);
        assert_eq!(
            BigUint::from_slice(&PI_LOWER_WORDS),
            BigUint::parse_bytes(hex.as_bytes(), 16).expect("endpoint hex")
        );
    }
}
