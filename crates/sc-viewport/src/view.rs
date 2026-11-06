//! The mapping between a pattern's coordinates and the screen, held exactly so it cannot drift.
use core::fmt;

/// A point in micrometres, the model's own unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    /// Across.
    pub x: i64,
    /// Down.
    pub y: i64,
}

/// Screen pixels per millimetre at a zoom of 1:1. An exact value, chosen for the arithmetic.
const PIXELS_PER_MILLIMETRE: i128 = 4;
const MICROMETRES_PER_MILLIMETRE: i128 = 1000;

/// The zoom ladder ADR-0002's spike declares, from 1:100 to 40:1, as numerator and denominator.
pub const ZOOMS: [(i64, i64); 9] = [
    (1, 100),
    (1, 50),
    (1, 20),
    (1, 10),
    (1, 5),
    (1, 1),
    (5, 1),
    (20, 1),
    (40, 1),
];

/// A viewport went outside what it can hold exactly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewError {
    /// A zoom must be a positive fraction.
    BadZoom,
    /// A coordinate grew past what exact arithmetic can hold.
    TooLarge,
}

impl fmt::Display for ViewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadZoom => f.write_str("a zoom must be a positive fraction"),
            Self::TooLarge => f.write_str("this coordinate is too large to hold exactly"),
        }
    }
}

impl std::error::Error for ViewError {}

const fn gcd(mut a: i128, mut b: i128) -> i128 {
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }
    while b != 0 {
        let next = a % b;
        a = b;
        b = next;
    }
    a
}

/// An exact fraction, always reduced with a positive denominator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Exact {
    top: i128,
    bottom: i128,
}

impl Exact {
    const fn int(value: i128) -> Self {
        Self {
            top: value,
            bottom: 1,
        }
    }

    fn new(top: i128, bottom: i128) -> Result<Self, ViewError> {
        if bottom == 0 {
            return Err(ViewError::TooLarge);
        }
        let divisor = gcd(top, bottom).max(1);
        let sign = if bottom < 0 { -1 } else { 1 };
        Ok(Self {
            top: sign * top / divisor,
            bottom: sign * bottom / divisor,
        })
    }

    fn add(self, other: Self) -> Result<Self, ViewError> {
        let left = self
            .top
            .checked_mul(other.bottom)
            .ok_or(ViewError::TooLarge)?;
        let right = other
            .top
            .checked_mul(self.bottom)
            .ok_or(ViewError::TooLarge)?;
        let top = left.checked_add(right).ok_or(ViewError::TooLarge)?;
        Self::new(
            top,
            self.bottom
                .checked_mul(other.bottom)
                .ok_or(ViewError::TooLarge)?,
        )
    }

    fn mul(self, other: Self) -> Result<Self, ViewError> {
        Self::new(
            self.top.checked_mul(other.top).ok_or(ViewError::TooLarge)?,
            self.bottom
                .checked_mul(other.bottom)
                .ok_or(ViewError::TooLarge)?,
        )
    }

    fn neg(self) -> Self {
        Self {
            top: -self.top,
            bottom: self.bottom,
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn to_f64(self) -> f64 {
        self.top as f64 / self.bottom as f64
    }
}

/// Where the viewport looks, and how closely. Every move is computed exactly, so zooming out again returns to
/// exactly where it was, however many times, and no error can build up.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct View {
    // World position at the screen's top-left corner, in micrometres.
    origin_x: Exact,
    origin_y: Exact,
    // Pixels per micrometre.
    scale: Exact,
}

impl View {
    /// A view at a zoom of `zoom.0 : zoom.1`, with the world origin at the screen's corner.
    ///
    /// # Errors
    /// The zoom is not a positive fraction.
    pub fn new(zoom: (i64, i64)) -> Result<Self, ViewError> {
        Ok(Self {
            origin_x: Exact::int(0),
            origin_y: Exact::int(0),
            scale: scale_of(zoom)?,
        })
    }

    /// The world position under a screen position.
    ///
    /// # Errors
    /// The value does not fit exact arithmetic.
    pub fn world_of_pixel(self, x: i64, y: i64) -> Result<(f64, f64), ViewError> {
        let (world_x, world_y) = self.exact_world(x, y)?;
        Ok((world_x.to_f64(), world_y.to_f64()))
    }

    fn exact_world(self, x: i64, y: i64) -> Result<(Exact, Exact), ViewError> {
        let inverse = Exact::new(self.scale.bottom, self.scale.top)?;
        Ok((
            Exact::int(i128::from(x)).mul(inverse)?.add(self.origin_x)?,
            Exact::int(i128::from(y)).mul(inverse)?.add(self.origin_y)?,
        ))
    }

    /// The world point nearest a screen position, to the micrometre.
    ///
    /// # Errors
    /// The value does not fit exact arithmetic.
    pub fn point_of_pixel(self, x: i64, y: i64) -> Result<Point, ViewError> {
        let (world_x, world_y) = self.exact_world(x, y)?;
        Ok(Point {
            x: nearest(world_x)?,
            y: nearest(world_y)?,
        })
    }

    /// The exact screen position of a world point, as a floating value for a renderer.
    ///
    /// # Errors
    /// The value does not fit exact arithmetic.
    pub fn pixel_of(self, point: Point) -> Result<(f64, f64), ViewError> {
        let x = Exact::int(i128::from(point.x))
            .add(self.origin_x.neg())?
            .mul(self.scale)?;
        let y = Exact::int(i128::from(point.y))
            .add(self.origin_y.neg())?
            .mul(self.scale)?;
        Ok((x.to_f64(), y.to_f64()))
    }

    /// Change the zoom, keeping the world point under the screen position `(x, y)` exactly where it is.
    ///
    /// # Errors
    /// The zoom is not a positive fraction, or a value does not fit exact arithmetic.
    pub fn zoom_about(self, zoom: (i64, i64), x: i64, y: i64) -> Result<Self, ViewError> {
        let scale = scale_of(zoom)?;
        let (world_x, world_y) = self.exact_world(x, y)?;
        let inverse = Exact::new(scale.bottom, scale.top)?;
        Ok(Self {
            origin_x: world_x.add(Exact::int(i128::from(x)).mul(inverse)?.neg())?,
            origin_y: world_y.add(Exact::int(i128::from(y)).mul(inverse)?.neg())?,
            scale,
        })
    }

    /// Move by whole pixels.
    ///
    /// # Errors
    /// A value does not fit exact arithmetic.
    pub fn pan(self, x: i64, y: i64) -> Result<Self, ViewError> {
        let (origin_x, origin_y) = self.exact_world(x, y)?;
        Ok(Self {
            origin_x,
            origin_y,
            scale: self.scale,
        })
    }

    /// The transform a renderer applies: `pixel = world * scale - offset`, as floating values.
    #[must_use]
    pub fn transform(self) -> (f64, f64, f64) {
        let scale = self.scale.to_f64();
        let offset = |origin: Exact| origin.mul(self.scale).map_or(0.0, Exact::to_f64);
        (scale, offset(self.origin_x), offset(self.origin_y))
    }
}

fn scale_of(zoom: (i64, i64)) -> Result<Exact, ViewError> {
    if zoom.0 <= 0 || zoom.1 <= 0 {
        return Err(ViewError::BadZoom);
    }
    Exact::new(
        PIXELS_PER_MILLIMETRE * i128::from(zoom.0),
        MICROMETRES_PER_MILLIMETRE * i128::from(zoom.1),
    )
}

fn nearest(value: Exact) -> Result<i64, ViewError> {
    let doubled = value.top.checked_mul(2).ok_or(ViewError::TooLarge)?;
    let rounded = (doubled + value.bottom)
        .div_euclid(value.bottom.checked_mul(2).ok_or(ViewError::TooLarge)?);
    i64::try_from(rounded).map_err(|_| ViewError::TooLarge)
}
