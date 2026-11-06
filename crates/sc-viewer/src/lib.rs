//! The `wasm-viewer` runtime profile's entry point: runs a session against a project image.
//!
//! The browser holds the project as one image, which it stores in IndexedDB in a single transaction. Everything
//! between the two is ordinary Rust, shared with the native program, so both give the same bytes.
use sc_store::{parse, FilePath, MemoryRepository, Value};
use std::collections::BTreeMap;

/// The project image as canonical text: each file's path and its bytes in hexadecimal.
#[must_use]
pub fn image_to_text(repository: &MemoryRepository) -> String {
    let mut fields = BTreeMap::new();
    for (path, bytes) in repository.image() {
        fields.insert(path.to_string(), Value::Text(to_hex(bytes)));
    }
    Value::Object(fields).to_text()
}

/// A project image from its canonical text. An empty text is an empty project.
///
/// # Errors
/// The text is not an image: not canonical text, a bad path, or bad hexadecimal.
pub fn image_from_text(text: &str) -> Result<MemoryRepository, String> {
    if text.trim().is_empty() {
        return Ok(MemoryRepository::new());
    }
    let value = parse(text).map_err(|error| format!("the project image: {error}"))?;
    let fields = value
        .as_object()
        .ok_or("the project image must be an object")?;
    let mut files = BTreeMap::new();
    for (path, bytes) in fields {
        let path = FilePath::new(path).map_err(|error| error.to_string())?;
        let hex = bytes
            .as_text()
            .ok_or_else(|| format!("{path} must be hexadecimal text"))?;
        files.insert(
            path.clone(),
            from_hex(hex).ok_or_else(|| format!("{path} is not hexadecimal"))?,
        );
    }
    Ok(MemoryRepository::from_image(files))
}

fn to_hex(bytes: &[u8]) -> String {
    let digit = |value: u8| char::from_digit(u32::from(value), 16).unwrap_or('0');
    bytes
        .iter()
        .flat_map(|byte| [digit(byte >> 4), digit(byte & 15)])
        .collect()
}

fn from_hex(text: &str) -> Option<Vec<u8>> {
    let digits: Vec<u32> = text
        .chars()
        .map(|character| character.to_digit(16))
        .collect::<Option<_>>()?;
    digits
        .chunks(2)
        .map(|pair| match pair {
            [high, low] => u8::try_from(high * 16 + low).ok(),
            _ => None,
        })
        .collect()
}

/// What a run returns: the transcript and the project image after it, or a structured failure.
#[must_use]
pub fn run(session: &str, image: &str) -> Value {
    match run_inner(session, image) {
        Ok(value) => value,
        Err(failure) => failure,
    }
}

fn plain_failure(message: String) -> Value {
    let mut fields = BTreeMap::new();
    fields.insert("failed".to_owned(), Value::Text("usage".to_owned()));
    fields.insert("message".to_owned(), Value::Text(message));
    Value::Object(fields)
}

fn run_inner(session: &str, image: &str) -> Result<Value, Value> {
    let session = parse(session).map_err(|error| plain_failure(format!("the session: {error}")))?;
    let mut repository = image_from_text(image).map_err(plain_failure)?;
    let transcript =
        sc_cli::replay(&session, &mut repository).map_err(|failure| failure.to_value())?;
    let mut fields = BTreeMap::new();
    fields.insert("transcript".to_owned(), transcript);
    fields.insert("image".to_owned(), Value::Text(image_to_text(&repository)));
    Ok(Value::Object(fields))
}

/// The WebAssembly exports. Bytes cross one at a time, so nothing here needs a raw pointer.
#[allow(unsafe_code, missing_docs)]
pub mod exports {
    use std::cell::RefCell;

    thread_local! {
        static SESSION: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
        static IMAGE: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
        static OUTPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    }

    fn with_slot<T>(slot: u32, action: impl FnOnce(&mut Vec<u8>) -> T) -> T {
        let cell = if slot == 0 { &SESSION } else { &IMAGE };
        cell.with(|bytes| action(&mut bytes.borrow_mut()))
    }

    /// Empties an input: 0 is the session, and anything else is the image.
    #[no_mangle]
    pub extern "C" fn sc_input_clear(slot: u32) {
        with_slot(slot, Vec::clear);
    }

    /// Appends one byte to an input.
    #[no_mangle]
    pub extern "C" fn sc_input_push(slot: u32, byte: u32) {
        with_slot(slot, |bytes| bytes.push(u8::try_from(byte).unwrap_or(0)));
    }

    /// Runs the session, leaving the result as the output.
    #[no_mangle]
    pub extern "C" fn sc_run() {
        let read = |slot| with_slot(slot, |bytes| String::from_utf8_lossy(bytes).into_owned());
        let result = super::run(&read(0), &read(1)).to_text();
        OUTPUT.with(|bytes| *bytes.borrow_mut() = result.into_bytes());
    }

    /// The output's length in bytes.
    #[no_mangle]
    pub extern "C" fn sc_output_len() -> u32 {
        OUTPUT.with(|bytes| u32::try_from(bytes.borrow().len()).unwrap_or(u32::MAX))
    }

    /// One byte of the output.
    #[no_mangle]
    pub extern "C" fn sc_output_byte(at: u32) -> u32 {
        OUTPUT.with(|bytes| {
            let at = usize::try_from(at).unwrap_or(usize::MAX);
            bytes.borrow().get(at).copied().map_or(0, u32::from)
        })
    }

    // The canvas spike's exports: the corpus, its answers, and one exact view.
    thread_local! {
        static PIECES: RefCell<Vec<sc_viewport::Piece>> = const { RefCell::new(Vec::new()) };
        static VIEW: RefCell<Option<sc_viewport::View>> = const { RefCell::new(None) };
    }

    /// Generates the corpus for a seed, and leaves it as the output.
    #[no_mangle]
    pub extern "C" fn sc_spike_load(seed: u32) {
        let text = super::corpus_text(u64::from(seed));
        OUTPUT.with(|bytes| *bytes.borrow_mut() = text.into_bytes());
        PIECES.with(|pieces| *pieces.borrow_mut() = sc_viewport::corpus(u64::from(seed)).pieces);
    }

    fn found(edge: Option<sc_viewport::EdgeRef>) -> i64 {
        edge.map_or(-1, |edge| {
            i64::try_from(edge.piece * 100_000 + edge.edge).unwrap_or(-1)
        })
    }

    /// The edge within reach of a world point, as piece * 100000 + edge, or -1.
    #[no_mangle]
    pub extern "C" fn sc_spike_pick(x: i64, y: i64, reach: i64) -> i64 {
        PIECES.with(|pieces| {
            found(sc_viewport::pick(
                &pieces.borrow(),
                sc_viewport::Point { x, y },
                reach,
            ))
        })
    }

    /// The vertex nearest a world point, as piece * 100000 + vertex, or -1.
    #[no_mangle]
    pub extern "C" fn sc_spike_snap(x: i64, y: i64) -> i64 {
        PIECES.with(|pieces| {
            found(sc_viewport::snap(
                &pieces.borrow(),
                sc_viewport::Point { x, y },
            ))
        })
    }

    /// Starts a view at this zoom. Returns 0, or 1 if the zoom is refused.
    #[no_mangle]
    pub extern "C" fn sc_view_new(numerator: i64, denominator: i64) -> u32 {
        let view = sc_viewport::View::new((numerator, denominator));
        let failed = u32::from(view.is_err());
        VIEW.with(|cell| *cell.borrow_mut() = view.ok());
        failed
    }

    fn change(
        action: impl FnOnce(sc_viewport::View) -> Result<sc_viewport::View, sc_viewport::ViewError>,
    ) -> u32 {
        VIEW.with(|cell| {
            let current = *cell.borrow();
            match current.map(action) {
                Some(Ok(next)) => {
                    *cell.borrow_mut() = Some(next);
                    0
                }
                _ => 1,
            }
        })
    }

    /// Changes the zoom about a screen position. Returns 0, or 1 if it could not.
    #[no_mangle]
    pub extern "C" fn sc_view_zoom_about(numerator: i64, denominator: i64, x: i64, y: i64) -> u32 {
        change(|view| view.zoom_about((numerator, denominator), x, y))
    }

    /// Moves by whole pixels. Returns 0, or 1 if it could not.
    #[no_mangle]
    pub extern "C" fn sc_view_pan(x: i64, y: i64) -> u32 {
        change(|view| view.pan(x, y))
    }

    fn read<T: Default>(action: impl FnOnce(sc_viewport::View) -> T) -> T {
        VIEW.with(|cell| (*cell.borrow()).map(action).unwrap_or_default())
    }

    /// The renderer's scale: pixels per micrometre.
    #[no_mangle]
    pub extern "C" fn sc_view_scale() -> f64 {
        read(|view| view.transform().0)
    }

    /// The renderer's horizontal offset, in pixels.
    #[no_mangle]
    pub extern "C" fn sc_view_offset_x() -> f64 {
        read(|view| view.transform().1)
    }

    /// The renderer's vertical offset, in pixels.
    #[no_mangle]
    pub extern "C" fn sc_view_offset_y() -> f64 {
        read(|view| view.transform().2)
    }

    /// A world point's exact screen x.
    #[no_mangle]
    pub extern "C" fn sc_view_pixel_x(x: i64, y: i64) -> f64 {
        read(|view| {
            view.pixel_of(sc_viewport::Point { x, y })
                .map_or(0.0, |pixel| pixel.0)
        })
    }

    /// A world point's exact screen y.
    #[no_mangle]
    pub extern "C" fn sc_view_pixel_y(x: i64, y: i64) -> f64 {
        read(|view| {
            view.pixel_of(sc_viewport::Point { x, y })
                .map_or(0.0, |pixel| pixel.1)
        })
    }

    /// The world x nearest a screen position, to the micrometre.
    #[no_mangle]
    pub extern "C" fn sc_view_point_x(x: i64, y: i64) -> i64 {
        read(|view| view.point_of_pixel(x, y).map_or(0, |point| point.x))
    }

    /// The world y nearest a screen position, to the micrometre.
    #[no_mangle]
    pub extern "C" fn sc_view_point_y(x: i64, y: i64) -> i64 {
        read(|view| view.point_of_pixel(x, y).map_or(0, |point| point.y))
    }
}

/// The canvas spike's corpus as canonical text, in the form the spike's page reads.
#[must_use]
pub fn corpus_text(seed: u64) -> String {
    let set = sc_viewport::corpus(seed);
    let coordinates = |points: &[sc_viewport::Point]| {
        Value::List(
            points
                .iter()
                .flat_map(|point| [Value::Int(point.x), Value::Int(point.y)])
                .collect(),
        )
    };
    let probes = |probes: &[sc_viewport::Probe]| {
        Value::List(
            probes
                .iter()
                .map(|probe| {
                    Value::List(vec![
                        Value::Int(probe.at.x),
                        Value::Int(probe.at.y),
                        Value::Int(i64::try_from(probe.piece).unwrap_or(0)),
                        Value::Int(i64::try_from(probe.entity).unwrap_or(0)),
                    ])
                })
                .collect(),
        )
    };
    let pieces = set.pieces.iter().map(|piece| {
        let mut fields = BTreeMap::new();
        fields.insert("outline".to_owned(), coordinates(&piece.outline));
        fields.insert("annotations".to_owned(), coordinates(&piece.annotations));
        Value::Object(fields)
    });
    let mut fields = BTreeMap::new();
    fields.insert("pieces".to_owned(), Value::List(pieces.collect()));
    fields.insert("picks".to_owned(), probes(&set.picks));
    fields.insert("snaps".to_owned(), probes(&set.snaps));
    Value::Object(fields).to_text()
}
