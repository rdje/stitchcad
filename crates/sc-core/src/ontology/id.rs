//! Entity identity: the ULID, and the injected generator that makes it deterministic.
//!
//! The normative source is `docs/book/src/spec/ontology.md` §1 ("Every semantic entity carries an entity
//! id: a ULID, assigned once at creation, never reused, never re-derived from content. Ids are stable
//! across saves, grades and exports") and §7 (canonical content carries no wall-clock values). The coupled
//! decision is `docs/decisions/decision_entity-identity-ulid-injected-generator.md`.
//!
//! Two properties drive the design:
//!
//! - **Dependency-free.** `sc-core` builds for `wasm32-unknown-unknown` (the G0 CI smoketest), so the
//!   128-bit value and its Crockford base32 form are hand-rolled here rather than pulled from a ULID crate
//!   that would drag a clock and an entropy source into the wasm graph.
//! - **Deterministic where it must be.** Recipe re-evaluation and CLI replay must be byte-identical
//!   (ontology §9, `G1-SLICE.10`), and a real ULID embeds a timestamp and randomness. So *generation* is
//!   behind the injected [`IdGenerator`]: a [`DeterministicIdGenerator`] for tests and replay, a
//!   clock-plus-entropy generator supplied by the composition root (the command bus, `G1-SLICE.6`) for
//!   production. The wall-clock and the entropy source never appear in domain code or canonical content.

use core::fmt;
use core::str::FromStr;

/// The bits of the timestamp half of a ULID.
const TIMESTAMP_BITS: u32 = 48;
/// The bits of the randomness half of a ULID.
const RANDOMNESS_BITS: u32 = 80;
/// The largest millisecond timestamp a ULID can carry (48 bits).
pub const MAX_TIMESTAMP_MS: u64 = (1u64 << TIMESTAMP_BITS) - 1;
/// The mask over the randomness half (80 bits).
const RANDOMNESS_MASK: u128 = (1u128 << RANDOMNESS_BITS) - 1;
/// The length of the canonical Crockford base32 form: 26 characters encode 130 bits, of which a ULID uses
/// the low 128, so the most significant character carries only three bits (values `0`..=`7`).
const CANONICAL_LEN: usize = 26;
/// Crockford's base32 alphabet: the digits, then the consonants, excluding `I L O U`.
const CROCKFORD: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// The identity of a semantic entity: a ULID, 128 bits, lexicographically sortable by creation time.
///
/// The high 48 bits are a millisecond timestamp and the low 80 bits are randomness, so the numeric order
/// of the underlying `u128` is creation order — which is what makes an id sortable without a separate
/// sequence, and what makes its 26-character Crockford form sort the same way as its bytes.
///
/// An `EntityId` is assigned once and never re-derived from content; equality and hashing are over the
/// 128 bits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(u128);

impl EntityId {
    /// Builds an id from its two ULID halves.
    ///
    /// # Errors
    ///
    /// Returns [`IdError::TimestampOverflow`] when `timestamp_ms` does not fit 48 bits, and
    /// [`IdError::RandomnessOverflow`] when `randomness` does not fit 80 bits. Neither is masked silently:
    /// an id is identity, and a truncated half would be a different identity than the caller meant.
    pub fn from_parts(timestamp_ms: u64, randomness: u128) -> Result<Self, IdError> {
        if timestamp_ms > MAX_TIMESTAMP_MS {
            return Err(IdError::TimestampOverflow {
                value: timestamp_ms,
            });
        }
        if randomness > RANDOMNESS_MASK {
            return Err(IdError::RandomnessOverflow { value: randomness });
        }
        Ok(Self(
            (u128::from(timestamp_ms) << RANDOMNESS_BITS) | randomness,
        ))
    }

    /// Builds an id directly from its 128 bits. Infallible: any `u128` is a well-formed id value, and this
    /// is how a [`DeterministicIdGenerator`] produces reproducible ids without a clock.
    #[must_use]
    pub const fn from_bits(bits: u128) -> Self {
        Self(bits)
    }

    /// The 128 bits of the id.
    #[must_use]
    pub const fn as_bits(self) -> u128 {
        self.0
    }

    /// The millisecond timestamp half (the high 48 bits).
    #[must_use]
    pub const fn timestamp_ms(self) -> u64 {
        (self.0 >> RANDOMNESS_BITS) as u64
    }

    /// The randomness half (the low 80 bits).
    #[must_use]
    pub const fn randomness(self) -> u128 {
        self.0 & RANDOMNESS_MASK
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Encode the 128 bits as 26 Crockford base32 characters, most significant first. The first
        // character carries the top three bits, so it is one of '0'..='7'.
        let mut v = self.0;
        let mut buf = [b'0'; CANONICAL_LEN];
        for slot in buf.iter_mut().rev() {
            // `v & 0x1f` is 0..=31 and `CROCKFORD` has exactly 32 entries, so `.get` is always `Some`;
            // the fallback keeps the formatter total without indexing, which the workspace forbids.
            *slot = CROCKFORD.get((v & 0x1f) as usize).copied().unwrap_or(b'0');
            v >>= 5;
        }
        // The buffer is ASCII by construction, so the conversion cannot fail; from_utf8 of a fixed ASCII
        // array is lossless. A formatter has no Result to return an error through, and this cannot panic.
        f.write_str(core::str::from_utf8(&buf).unwrap_or_default())
    }
}

impl FromStr for EntityId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() != CANONICAL_LEN {
            return Err(IdError::InvalidLength { found: bytes.len() });
        }
        let mut v: u128 = 0;
        for (i, &b) in bytes.iter().enumerate() {
            let digit = u128::from(crockford_value(b)?);
            // The most significant character carries only three bits; anything larger would push the value
            // past 128 bits, which is not a ULID.
            if i == 0 && digit > 7 {
                return Err(IdError::InvalidCharacter { byte: b });
            }
            v = (v << 5) | digit;
        }
        Ok(Self(v))
    }
}

/// The value of one Crockford base32 byte, case-insensitive.
///
/// The four excluded letters `I L O U` are rejected rather than substituted: an entity id is identity, and
/// accepting a look-alike would let two distinct strings name the same object, which §1 forbids ("never
/// re-derived from content"). Canonical encoding never emits them.
fn crockford_value(b: u8) -> Result<u8, IdError> {
    let v = match b {
        b'0'..=b'9' => b - b'0',
        b'A'..=b'H' => b - b'A' + 10,
        b'J' | b'K' => b - b'J' + 18,
        b'M' | b'N' => b - b'M' + 20,
        b'P'..=b'T' => b - b'P' + 22,
        b'V'..=b'Z' => b - b'V' + 27,
        b'a'..=b'h' => b - b'a' + 10,
        b'j' | b'k' => b - b'j' + 18,
        b'm' | b'n' => b - b'm' + 20,
        b'p'..=b't' => b - b'p' + 22,
        b'v'..=b'z' => b - b'v' + 27,
        _ => return Err(IdError::InvalidCharacter { byte: b }),
    };
    Ok(v)
}

/// Every way an id can fail to be built or parsed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdError {
    /// The millisecond timestamp does not fit the ULID's 48 bits.
    TimestampOverflow {
        /// The offending value.
        value: u64,
    },
    /// The randomness does not fit the ULID's 80 bits.
    RandomnessOverflow {
        /// The offending value.
        value: u128,
    },
    /// The canonical form was not exactly 26 characters.
    InvalidLength {
        /// The length that was found.
        found: usize,
    },
    /// A byte that is not a canonical Crockford base32 symbol (including the excluded `I L O U`).
    InvalidCharacter {
        /// The offending byte.
        byte: u8,
    },
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimestampOverflow { value } => write!(
                f,
                "timestamp {value} ms exceeds the ULID's {TIMESTAMP_BITS}-bit maximum {MAX_TIMESTAMP_MS}"
            ),
            Self::RandomnessOverflow { value } => write!(
                f,
                "randomness {value} exceeds the ULID's {RANDOMNESS_BITS}-bit maximum"
            ),
            Self::InvalidLength { found } => write!(
                f,
                "an entity id is {CANONICAL_LEN} Crockford base32 characters, found {found}"
            ),
            Self::InvalidCharacter { byte } => write!(
                f,
                "byte {byte:#04x} ({}) is not a canonical Crockford base32 symbol",
                char::from(*byte)
            ),
        }
    }
}

impl std::error::Error for IdError {}

/// The source of entity ids, **injected** so that generation is the only place a clock or an entropy source
/// can enter, and so a deterministic generator can replace it for tests and replay.
///
/// The command bus (`G1-SLICE.6`, the only mutation path) holds the production generator; domain code takes
/// ids from whatever generator it is given and never reads a clock directly.
pub trait IdGenerator {
    /// The next id. Implementations must return distinct, monotonically non-decreasing ids so that creation
    /// order is preserved (the property ULID's layout exists to give).
    fn next_id(&mut self) -> EntityId;
}

/// A reproducible generator: a counter over the id's bits, with no clock and no entropy.
///
/// Two runs that create the same objects in the same order get byte-identical ids, which is what makes
/// recipe re-evaluation and CLI replay deterministic (ontology §9, `G1-SLICE.10`). The ids it produces are
/// well-formed `u128` values with a zero timestamp half — valid identities, just not wall-clock ones, which
/// is exactly right for a test or a replay that must not depend on when it ran.
#[derive(Clone, Copy, Debug)]
pub struct DeterministicIdGenerator {
    next: u128,
}

impl DeterministicIdGenerator {
    /// A generator whose first id is `1` (never the nil id `0`).
    #[must_use]
    pub const fn new() -> Self {
        Self { next: 1 }
    }

    /// A generator whose first id is `start`, for tests that need a specific sequence.
    #[must_use]
    pub const fn from_start(start: u128) -> Self {
        Self { next: start }
    }
}

impl Default for DeterministicIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl IdGenerator for DeterministicIdGenerator {
    fn next_id(&mut self) -> EntityId {
        let id = EntityId::from_bits(self.next);
        // Wrapping is unreachable in practice (2^128 ids), and a wrap would still be deterministic; the
        // generator never panics, per the workspace lints.
        self.next = self.next.wrapping_add(1);
        id
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DeterministicIdGenerator, EntityId, IdError, IdGenerator, CANONICAL_LEN, MAX_TIMESTAMP_MS,
    };
    use core::str::FromStr;

    #[test]
    fn an_id_round_trips_its_canonical_crockford_form() {
        let id = EntityId::from_parts(1_700_000_000_000, 0x0123_4567_89ab_cdef_0123).unwrap();
        let text = id.to_string();
        assert_eq!(text.len(), CANONICAL_LEN);
        assert_eq!(EntityId::from_str(&text).unwrap(), id);
    }

    #[test]
    fn the_halves_survive_a_round_trip() {
        let id = EntityId::from_parts(123_456, 999).unwrap();
        assert_eq!(id.timestamp_ms(), 123_456);
        assert_eq!(id.randomness(), 999);
    }

    #[test]
    fn ids_sort_by_creation_time() {
        let early = EntityId::from_parts(1_000, 0xffff).unwrap();
        let late = EntityId::from_parts(2_000, 0).unwrap();
        assert!(
            early < late,
            "the timestamp is the high bits, so time order is numeric order"
        );
        // The canonical text sorts the same way as the bytes.
        assert!(early.to_string() < late.to_string());
    }

    #[test]
    fn a_timestamp_past_48_bits_is_a_diagnostic() {
        let err = EntityId::from_parts(MAX_TIMESTAMP_MS + 1, 0).unwrap_err();
        assert!(
            matches!(err, IdError::TimestampOverflow { .. }),
            "got {err:?}"
        );
        assert!(EntityId::from_parts(MAX_TIMESTAMP_MS, 0).is_ok());
    }

    #[test]
    fn randomness_past_80_bits_is_a_diagnostic_not_a_mask() {
        let too_big = 1u128 << 80;
        let err = EntityId::from_parts(0, too_big).unwrap_err();
        assert!(
            matches!(err, IdError::RandomnessOverflow { .. }),
            "got {err:?}"
        );
    }

    #[test]
    fn parsing_rejects_a_wrong_length_and_a_bad_character() {
        assert!(matches!(
            EntityId::from_str("0123").unwrap_err(),
            IdError::InvalidLength { found: 4 }
        ));
        // 'I' is excluded from Crockford; a 26-char string containing it is rejected, not substituted.
        let with_i = format!("{}{}", "I", "0".repeat(CANONICAL_LEN - 1));
        assert!(matches!(
            EntityId::from_str(&with_i).unwrap_err(),
            IdError::InvalidCharacter { .. }
        ));
    }

    #[test]
    fn the_most_significant_character_is_bounded_to_three_bits() {
        // '8' as the first character would encode more than 128 bits.
        let too_big = format!("{}{}", "8", "0".repeat(CANONICAL_LEN - 1));
        assert!(EntityId::from_str(&too_big).is_err());
        // '7' is the largest valid leading character.
        let max = format!("{}{}", "7", "Z".repeat(CANONICAL_LEN - 1));
        assert!(EntityId::from_str(&max).is_ok());
    }

    #[test]
    fn the_deterministic_generator_reproduces_a_sequence_byte_for_byte() {
        let mut a = DeterministicIdGenerator::new();
        let mut b = DeterministicIdGenerator::new();
        let mut prev: Option<EntityId> = None;
        for _ in 0..5 {
            let ia = a.next_id();
            let ib = b.next_id();
            assert_eq!(ia, ib, "the replay property: same generator, same ids");
            assert_eq!(
                ia.to_string(),
                ib.to_string(),
                "and they agree byte-for-byte in canonical form"
            );
            if let Some(p) = prev {
                assert!(
                    p < ia,
                    "ids strictly increase, so creation order is preserved"
                );
            }
            prev = Some(ia);
        }
    }

    #[test]
    fn the_generator_never_emits_the_nil_id_first() {
        let mut g = DeterministicIdGenerator::new();
        assert_ne!(g.next_id(), EntityId::from_bits(0));
    }

    #[test]
    fn parsing_accepts_lowercase_canonical_symbols() {
        let id = EntityId::from_parts(42, 0xabc).unwrap();
        let lower = id.to_string().to_ascii_lowercase();
        assert_eq!(EntityId::from_str(&lower).unwrap(), id);
    }
}
