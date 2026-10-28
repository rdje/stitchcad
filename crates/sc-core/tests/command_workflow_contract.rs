//! Registry discovery and generated book projection; actual bus workflow proof lives in sc-measure.
#![allow(clippy::expect_used)]
use sc_core::command::{
    CommandAuthority as Authority, CommandClass as Class, CommandWorkflowId as Workflow,
    CommandWorkflowStep as Step,
};
use std::collections::BTreeSet;

const BOOK: &str = include_str!("../../../docs/book/src/spec/command-workflows.md");

#[test]
fn generated_book_registry_matches_all_public_declarations_without_missing_or_stale_rows() {
    let table = BOOK
        .split_once("<!-- native-workflows:start -->")
        .expect("generated table start")
        .1
        .split_once("<!-- native-workflows:end -->")
        .expect("generated table end")
        .0;
    let rows = table
        .lines()
        .filter(|line| line.starts_with("| "))
        .skip(2)
        .collect::<Vec<_>>();
    assert_eq!(
        rows.len(),
        Workflow::ALL.len(),
        "every declaration is published exactly once"
    );
    let mut names = BTreeSet::new();
    for (workflow, row) in Workflow::ALL.into_iter().zip(rows) {
        assert!(
            names.insert(workflow.token()),
            "discovery identity is unique"
        );
        let cells = row
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        let [token, steps, commands, classes, authorities]: [&str; 5] =
            cells.try_into().expect("five generated cells");
        assert_eq!(token, workflow.token());
        assert_eq!(
            steps,
            workflow
                .steps()
                .iter()
                .map(|step| step.token())
                .collect::<Vec<_>>()
                .join(", ")
        );
        assert_eq!(
            commands,
            workflow
                .commands()
                .iter()
                .map(|command| command.token())
                .collect::<Vec<_>>()
                .join(", ")
        );
        assert_eq!(
            classes,
            workflow
                .command_classes()
                .iter()
                .map(|class| class.token())
                .collect::<Vec<_>>()
                .join(", ")
        );
        assert_eq!(
            authorities,
            workflow
                .required_authorities()
                .iter()
                .map(|authority| authority.token())
                .collect::<Vec<_>>()
                .join(", ")
        );
        assert_eq!(Workflow::from_token(token), Some(workflow));
    }
}

#[test]
fn exact_lookup_and_independent_step_permissions_have_no_rank_or_fallback() {
    for unknown in [
        "",
        "EditMetadata",
        "EDIT_METADATA",
        " edit_metadata",
        "edit_metadata ",
        "approve_package",
    ] {
        assert_eq!(Workflow::from_token(unknown), None);
    }
    assert_eq!(
        Workflow::InspectPieces.command_classes(),
        vec![Class::Query]
    );
    assert_eq!(
        Workflow::InspectPieces.required_authorities(),
        vec![Authority::Inspect]
    );
    for workflow in [Workflow::EditMetadata, Workflow::UndoRedoMetadata] {
        assert_eq!(
            workflow.command_classes(),
            vec![Class::Query, Class::Mutation]
        );
        assert_eq!(
            workflow.required_authorities(),
            vec![Authority::Inspect, Authority::Commit]
        );
        assert_eq!(
            workflow.commands().len(),
            4,
            "actual Piece query and three metadata mutations"
        );
    }
    assert_eq!(Step::PreviewGroup.required_authority(), Authority::Inspect);
    for operation in [
        Step::CommitGroup,
        Step::ReplayGroup,
        Step::UndoGroup,
        Step::RedoGroup,
    ] {
        assert_eq!(operation.required_authority(), Authority::Commit);
        assert_eq!(
            operation.commands(),
            Step::PreviewGroup.commands(),
            "one complete original group scope"
        );
    }
}
