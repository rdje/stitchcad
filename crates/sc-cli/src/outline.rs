//! A closed outline drawn from evaluated lengths, as canonical SVG in exact units.
//!
//! This is not drafted geometry. It draws straight lines between points a session names, so the path from a
//! recipe to a picture and a file can run for real before G2's geometry kernel exists.

/// Micrometres, in the page's frame: x to the right, y down, so depth below the waist line is positive.
pub type Point = (i64, i64);

// Micrometres as millimetres, with no floating point anywhere.
fn millimetres(micrometres: i64) -> String {
    format!("{}.{:03}", micrometres / 1000, micrometres % 1000)
}

/// One canonical SVG for these points: the same points always give the same bytes.
///
/// # Errors
/// An outline needs three points, none negative, and some extent in both directions.
pub fn svg(points: &[Point]) -> Result<String, String> {
    if points.len() < 3 {
        return Err("an outline needs at least three points".to_owned());
    }
    if points.iter().any(|(x, y)| *x < 0 || *y < 0) {
        return Err("an outline's points must not be negative".to_owned());
    }
    let width = points.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let height = points.iter().map(|(_, y)| *y).max().unwrap_or(0);
    if width == 0 || height == 0 {
        return Err("an outline needs some width and some height".to_owned());
    }
    let listed: Vec<String> = points.iter().map(|(x, y)| format!("{x},{y}")).collect();
    Ok(format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}mm\" height=\"{}mm\" viewBox=\"0 0 {width} {height}\">\n<polygon points=\"{}\" fill=\"none\" stroke=\"black\" stroke-width=\"500\"/>\n</svg>\n",
        millimetres(width),
        millimetres(height),
        listed.join(" "),
    ))
}
