//! Pure-stdout diagnostic inverse certificate chunk. No product inverse API or filesystem writes.
use num_bigint::BigUint;
use std::io::{self, Read, Write};

const BITS: u32 = 640;
const TERMS: u32 = 64;
const POPULATION: u32 = 45_000_000;

#[derive(Debug)]
struct Interval {
    low: BigUint,
    high: BigUint,
}

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

fn ceil_div(n: BigUint, d: &BigUint) -> BigUint {
    (n + d - 1_u8) / d
}

fn series(x: &Interval, cosine: bool, scale: &BigUint) -> io::Result<Interval> {
    let square = Interval {
        low: &x.low * &x.low / scale,
        high: ceil_div(&x.high * &x.high, scale),
    };
    let mut term = if cosine {
        Interval {
            low: scale.clone(),
            high: scale.clone(),
        }
    } else {
        Interval {
            low: x.low.clone(),
            high: x.high.clone(),
        }
    };
    let mut low: num_bigint::BigInt = 0_u8.into();
    let mut high: num_bigint::BigInt = 0_u8.into();
    for n in 0..TERMS {
        if n % 2 == 0 {
            low += num_bigint::BigInt::from(term.low.clone());
            high += num_bigint::BigInt::from(term.high.clone());
        } else {
            low -= num_bigint::BigInt::from(term.high.clone());
            high -= num_bigint::BigInt::from(term.low.clone());
        }
        let k = if cosine {
            (2 * n + 1) * (2 * n + 2)
        } else {
            (2 * n + 2) * (2 * n + 3)
        };
        let divisor = scale * k;
        term = Interval {
            low: &term.low * &square.low / &divisor,
            high: ceil_div(term.high * &square.high, &divisor),
        };
    }
    // Both true series stay in[0,1], with alternating tail <1/128!<2^-640.
    let result = Interval {
        low: (low - 1_u8).to_biguint().unwrap_or_default(),
        high: (high + 1_u8)
            .to_biguint()
            .ok_or_else(|| invalid("negative series upper enclosure"))?
            .min(scale.clone()),
    };
    if result.low > result.high || &result.high - &result.low > BigUint::from(770_u16) {
        return Err(invalid("series interval outside proved width"));
    }
    Ok(result)
}

fn slope_band(index: u32, pi: &Interval, scale: &BigUint) -> io::Result<Interval> {
    let numerator = BigUint::from(2 * index + 1) * 2_u8;
    let denominator = BigUint::from(720_000_000_u32);
    let x = Interval {
        low: &numerator * &pi.low / &denominator,
        high: ceil_div(numerator * &pi.high, &denominator),
    };
    if x.high > *scale || &x.high - &x.low > BigUint::from(3_u8) {
        return Err(invalid("radian interval outside prototype envelope"));
    }
    let sine = series(&x, false, scale)?;
    let cosine = series(&x, true, scale)?;
    if cosine.low == BigUint::from(0_u8) {
        return Err(invalid("nonpositive cosine enclosure"));
    }
    let low = sine.low * scale / &cosine.high;
    let high = ceil_div(sine.high * scale, &cosine.low);
    // Every actual angle band is inside the first octant: tan'<=2 and2*pi<8.
    // Hence slope displacement <=16*2^-600, a conservative superset of both endpoint images.
    let padding = BigUint::from(1_u8) << (BITS - 600 + 4);
    if low <= padding || &high + &padding >= *scale {
        return Err(invalid("padded slope outside open unit interval"));
    }
    Ok(Interval {
        low: low - &padding,
        high: high + padding,
    })
}

type Gap = (BigUint, BigUint, BigUint, BigUint);

fn gap(band: &Interval, scale: &BigUint, bound: &BigUint) -> io::Result<Gap> {
    let (mut n, mut d) = (&band.low + &band.high, scale * 2_u8);
    let (mut p0, mut q0, mut p1, mut q1) = (
        BigUint::from(0_u8),
        BigUint::from(1_u8),
        BigUint::from(1_u8),
        BigUint::from(0_u8),
    );
    loop {
        if d == BigUint::from(0_u8) {
            return Err(invalid("midpoint has an admitted rational denominator"));
        }
        let a = &n / &d;
        let q2 = &q0 + &a * &q1;
        if q2 > *bound {
            break;
        }
        let p2 = p0 + &a * &p1;
        (p0, q0, p1, q1) = (p1, q1, p2, q2);
        (n, d) = (d.clone(), n % d);
    }
    if q1 == BigUint::from(0_u8) {
        return Err(invalid("empty continued fraction"));
    }
    let k = (bound - &q0) / &q1;
    let (mut a, mut b, mut c, mut d) = (p0 + &k * &p1, q0 + k * &q1, p1, q1);
    if &a * &d > &c * &b {
        std::mem::swap(&mut a, &mut c);
        std::mem::swap(&mut b, &mut d);
    }
    if b == BigUint::from(0_u8)
        || d == BigUint::from(0_u8)
        || b > *bound
        || d > *bound
        || &b + &d <= *bound
        || &c * &b != &a * &d + 1_u8
        || &a * scale >= &band.low * &b
        || &band.high * &d >= &c * scale
    {
        return Err(invalid("exact strict Farey predicates failed"));
    }
    Ok((a, b, c, d))
}

fn write_fixed(stream: &mut impl Write, value: &BigUint, width: usize) -> io::Result<()> {
    let bytes = value.to_bytes_be();
    if bytes.len() > width {
        return Err(invalid("encoded component exceeds row width"));
    }
    stream.write_all(&vec![0; width - bytes.len()])?;
    stream.write_all(&bytes)
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let start = args
        .next()
        .ok_or_else(|| invalid("missing start"))?
        .parse::<u32>()
        .map_err(|_| invalid("invalid start"))?;
    let count = args
        .next()
        .ok_or_else(|| invalid("missing count"))?
        .parse::<u32>()
        .map_err(|_| invalid("invalid count"))?;
    let end = start
        .checked_add(count)
        .ok_or_else(|| invalid("range overflow"))?;
    if args.next().is_some() || count == 0 || count > 100_000 || end > POPULATION {
        return Err(invalid("range outside bounded prototype chunk"));
    }
    let mut protocol = String::new();
    io::stdin().take(1025).read_to_string(&mut protocol)?;
    if protocol.len() > 1024 {
        return Err(invalid("oversize pi input"));
    }
    let mut lines = protocol.lines();
    let low = BigUint::parse_bytes(
        lines
            .next()
            .ok_or_else(|| invalid("missing pi lower"))?
            .as_bytes(),
        16,
    )
    .ok_or_else(|| invalid("invalid pi lower"))?;
    let high = BigUint::parse_bytes(
        lines
            .next()
            .ok_or_else(|| invalid("missing pi upper"))?
            .as_bytes(),
        16,
    )
    .ok_or_else(|| invalid("invalid pi upper"))?;
    let scale = BigUint::from(1_u8) << BITS;
    if lines.next().is_some()
        || high != &low + 1_u8
        || low <= &scale * 3_u8
        || high >= &scale * 4_u8
    {
        return Err(invalid(
            "pi interval outside declared640-bit enclosure shape",
        ));
    }
    let pi = Interval { low, high };
    let magnitude = BigUint::from(u128::MAX);
    let bound = &magnitude * &magnitude;
    let mut stdout = io::BufWriter::new(io::stdout().lock());
    for index in start..end {
        let band = slope_band(index, &pi, &scale)?;
        let (a, b, c, d) = gap(&band, &scale, &bound)
            .map_err(|error| invalid(&format!("threshold {index}: {error}")))?;
        for (value, width) in [
            (&band.low, 80),
            (&band.high, 80),
            (&a, 32),
            (&b, 32),
            (&c, 32),
            (&d, 32),
        ] {
            write_fixed(&mut stdout, value, width)?;
        }
    }
    stdout.flush()
}
