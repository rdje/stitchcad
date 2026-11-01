//! Actual original source/role/identity parents for join Edge contract checks.
#[path = "join_banks.rs"]
mod banks;
use banks::with_custom_banks;
pub use banks::{id, BankFixture, SpanFixture};
use sc_core::{construction::*, name::MachineToken, ontology::*};
pub fn with_identity(options: BankFixture, run: impl FnOnce(DeclaredJoinPieceIdentity<'_>)) {
    with_custom_banks(options, |banks| {
        let tags = [9, 2, 4, 5, 6, 8, 10, 11, 12, 13, 7];
        let raw = EdgeOutputRoleCatalog::new(
            id(1000),
            tags.into_iter()
                .map(|t| {
                    EdgeOutputRoleDefinition::new(
                        LocalTag::new(t),
                        MachineToken::new(format!("edge_{t}")).expect("canonical key"),
                    )
                })
                .collect(),
            tags.into_iter()
                .filter(|t| *t != 7)
                .map(LocalTag::new)
                .collect(),
        );
        let b = raw
            .definitions()
            .iter()
            .map(|d| d.key().as_str().len())
            .sum();
        let limits = EdgeOutputRoleCatalogLimits::new(11, 10, b);
        let catalog = raw.admit(limits).expect("whole actual catalogue");
        let roles = banks
            .bind_edge_roles(&catalog, limits)
            .expect("actual owner");
        let field = id(5000);
        let identity = roles
            .declare_piece_identity(
                &field,
                JoinPieceIdentityLimits::new(
                    if matches!(options.span, SpanFixture::RetainedWhole) {
                        2
                    } else {
                        1
                    },
                ),
            )
            .expect("actual local identity");
        run(identity);
    });
}
