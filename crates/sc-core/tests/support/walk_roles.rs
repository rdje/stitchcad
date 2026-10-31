//! Complete actual walk context plus original typed role metadata;no assignment/sharing proof.
#[path = "walk_copy_context.rs"]
pub mod copies;
use copies::input::id;
use sc_core::{construction::*, name::MachineToken, ontology::LocalTag};
pub fn with_roles(pairs: usize, active: &[u32], run: impl FnOnce(BoundWalkNotchRoles<'_>)) {
    let bind = |copies: BoundWalkCopyInventory<'_>| {
        let definitions = [(9, "balance"), (2, "end"), (7, "latent")]
            .into_iter()
            .map(|(tag, key)| {
                NotchOutputRoleDefinition::new(
                    LocalTag::new(tag),
                    MachineToken::new(key).expect("key"),
                )
            })
            .collect();
        let raw = NotchOutputRoleCatalog::new(
            id(10),
            definitions,
            active.iter().copied().map(LocalTag::new).collect(),
        );
        let limits = NotchOutputRoleCatalogLimits::new(3, 3, 16);
        let catalog = raw.admit(limits).expect("complete typed roles");
        let roles = copies
            .bind_notch_roles(&catalog, limits)
            .expect("actual same walk operation");
        run(roles);
    };
    if pairs == 1 {
        copies::with_copies(bind)
    } else {
        copies::with_pair_count(pairs, bind)
    }
}
