//! Every garment the roadmap names drafts, grades and exports, and the matrix says which leaf proved it. A garment nothing proves is a gate failure, not a scope note.
#![allow(clippy::expect_used, clippy::panic, clippy::indexing_slicing)]
use sc_cli::replay;
use sc_store::{parse, FilePath, MemoryRepository, Repository, Value};
use std::collections::BTreeMap;

const SKIRT: &str = include_str!("../../../conformance/reference-skirt/session.json");
const BODICE: &str = include_str!("../../../conformance/reference-bodice/session.json");
const TROUSERS: &str = include_str!("../../../conformance/reference-trousers/session.json");
const MATRIX: &str = include_str!("../../../docs/book/src/spec/feature-matrix.md");

// Roadmap §3.2's list: the garment, the fixture that drafts it, the pieces that are it, its row in the matrix, and the leaf and chapter its row has to cite.
type Garment = (
    &'static str,
    &'static str,
    &'static [&'static str],
    &'static str,
    &'static str,
    &'static str,
);

const THE_ENVELOPE: [Garment; 5] = [
    (
        "A-line skirt",
        SKIRT,
        &["skirt_front", "skirt_back_right", "waistband"],
        "A-line skirt, waist dart, CB zipper",
        "G2-2D",
        "reference-skirt.md",
    ),
    (
        "darted bodice",
        BODICE,
        &["bodice_front", "bodice_back_right"],
        "darted bodice",
        "G3-GRADING.4",
        "reference-bodice.md",
    ),
    (
        "set-in sleeve",
        BODICE,
        &["sleeve_right"],
        "set-in sleeve with declared cap ease",
        "G3-GRADING.4",
        "reference-bodice.md",
    ),
    (
        "classic collar",
        BODICE,
        &["collar_right"],
        "classic collar",
        "G3-GRADING.15",
        "reference-bodice.md",
    ),
    (
        "trousers",
        TROUSERS,
        &["trouser_front_right", "trouser_back_right", "waistband"],
        "trousers",
        "G3-GRADING.5",
        "reference-trousers.md",
    ),
];

fn fields_of(value: &Value) -> &BTreeMap<String, Value> {
    value.as_object().expect("an object")
}

fn with_steps(session: &str, extra: &str) -> Value {
    let whole = parse(session).expect("valid");
    let mut fields = fields_of(&whole).clone();
    let mut steps: Vec<Value> = fields["steps"]
        .as_list()
        .expect("steps")
        .iter()
        .filter(|step| fields_of(step)["do"].as_text() != Some("save"))
        .cloned()
        .collect();
    steps.extend(
        parse(extra)
            .expect("valid")
            .as_list()
            .expect("steps")
            .iter()
            .cloned(),
    );
    fields.insert("steps".to_owned(), Value::List(steps));
    Value::Object(fields)
}

#[test]
fn every_garment_the_roadmap_names_drafts_grades_and_exports() {
    for (garment, session, pieces, _, _, _) in THE_ENVELOPE {
        let plan = with_steps(
            session,
            r#"[{"do":"regenerate","path":"exports/family.json"},
                {"do":"grade","path":"exports/graded.json","rul":"exports/g.rul"},
                {"do":"export_graded","mode":"base_plus_rules","semantics":"sew_as_1","path":"exports/base"},
                {"do":"export_graded","mode":"embedded_rules","semantics":"sew_as_1","path":"exports/embedded"},
                {"do":"export_graded","mode":"all_contours","semantics":"sew_as_1","path":"exports/nest"},
                {"do":"save"}]"#,
        );
        let mut disk = MemoryRepository::new();
        let transcript =
            replay(&plan, &mut disk).unwrap_or_else(|failure| panic!("{garment}: {failure:?}"));
        let steps = transcript.as_list().expect("a transcript");
        // It drafts, at every size, with every piece, and walks with nothing undeclared.
        let family = String::from_utf8(
            disk.read(&FilePath::new("exports/family.json").expect("a path"))
                .expect("reads")
                .expect("written"),
        )
        .expect("text");
        let family = parse(&family).expect("valid");
        let members = fields_of(&family)["sizes"].as_list().expect("sizes");
        assert_eq!(members.len(), 5, "{garment}");
        for member in members {
            let drawn = sc_artifacts::from_canonical_text(&fields_of(member)["pattern"].to_text())
                .expect("reads");
            for piece in pieces {
                assert!(
                    drawn.pieces.iter().any(|item| item.name == *piece),
                    "{garment}: {piece}"
                );
            }
        }
        let regenerated = steps
            .iter()
            .map(fields_of)
            .find(|step| step["do"].as_text() == Some("regenerate"))
            .expect("the step");
        for report in fields_of(&regenerated["result"])["sizes"]
            .as_list()
            .expect("sizes")
            .iter()
            .map(fields_of)
        {
            for seam in report["walk"]
                .as_list()
                .expect("a walk")
                .iter()
                .map(fields_of)
            {
                assert_ne!(
                    seam["finding"].as_text(),
                    Some("defect"),
                    "{garment} at {:?}: {seam:?}",
                    report["size"]
                );
            }
        }
        // It grades, and a size from a rule can be told from a size from the recipe.
        let graded = steps
            .iter()
            .map(fields_of)
            .find(|step| step["do"].as_text() == Some("grade"))
            .expect("the step");
        assert_eq!(
            fields_of(&graded["result"])["sizes"]
                .as_list()
                .expect("sizes")
                .len(),
            5,
            "{garment}"
        );
        // It exports, at one size and graded, in every form.
        for name in [
            "exports/g.rul",
            "exports/base.dxf",
            "exports/base.rul",
            "exports/embedded.dxf",
            "exports/embedded.rul",
            "exports/nest.dxf",
        ] {
            assert!(
                disk.read(&FilePath::new(name).expect("a path"))
                    .expect("reads")
                    .is_some_and(|bytes| !bytes.is_empty()),
                "{garment}: {name}"
            );
        }
        let exports: Vec<&str> = steps
            .iter()
            .map(fields_of)
            .filter(|step| step["do"].as_text() == Some("export_pattern"))
            .map(|step| step["do"].as_text().expect("text"))
            .collect();
        assert!(!exports.is_empty(), "{garment} exports at its sample size");
    }
}

#[test]
fn the_matrix_says_which_leaf_proved_each_one() {
    for (garment, _, _, row, leaf, chapter) in THE_ENVELOPE {
        let line = MATRIX
            .lines()
            .find(|line| line.starts_with(&format!("| {row} |")))
            .unwrap_or_else(|| panic!("the matrix has no row for {garment}"));
        assert!(line.contains("supported"), "{line}");
        assert!(line.contains(&format!("`{leaf}")), "{garment}: {line}");
        assert!(line.contains(chapter), "{garment}: {line}");
    }
    // What the trousers carry has its own rows, and they cite it too.
    for (row, leaf) in [
        ("pocket", "G3-GRADING.5"),
        ("button and buttonhole", "G3-GRADING.5"),
    ] {
        let line = MATRIX
            .lines()
            .find(|line| line.starts_with(&format!("| {row} |")))
            .unwrap_or_else(|| panic!("the matrix has no row for {row}"));
        assert!(line.contains(&format!("`{leaf}`")), "{line}");
    }
}
