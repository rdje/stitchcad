//! Actual retained catalogue fields and indexes,not prior peak admission-work estimates.
use super::{
    ActiveAssociation, AdmittedEdgeOutputRoleCatalog, EdgeOutputRoleCatalog,
    EdgeOutputRoleDefinition,
};
use crate::{
    construction::operation_input::resources::fixed,
    design::{
        DesignResourceLimitError, DesignResourceLocation, DesignResourceScope, DesignResourceSink,
    },
};
use std::collections::BTreeMap;

pub(in crate::construction) enum OutputRoleResourceKind {
    Point,
    Edge,
}
impl OutputRoleResourceKind {
    fn fields(self) -> [&'static str; 6] {
        match self {
            Self::Point => [
                "drafting.names.point_roles.definitions",
                "drafting.names.point_roles.active",
                "drafting.names.point_roles.by_tag",
                "drafting.names.point_roles.active_by_tag",
                "drafting.names.point_roles.active_roles",
                "drafting.names.point_roles.key",
            ],
            Self::Edge => [
                "drafting.names.edge_roles.definitions",
                "drafting.names.edge_roles.active",
                "drafting.names.edge_roles.by_tag",
                "drafting.names.edge_roles.active_by_tag",
                "drafting.names.edge_roles.active_roles",
                "drafting.names.edge_roles.key",
            ],
        }
    }
}
fn fixed_map<K: Copy, V: Copy>(_: &BTreeMap<K, V>) {}
fn fixed_slice<T: Copy>(_: &[T]) {}
impl AdmittedEdgeOutputRoleCatalog<'_> {
    pub(in crate::construction) fn charge_retained_resources(
        &self,
        kind: OutputRoleResourceKind,
        source_index: usize,
        sink: &mut dyn DesignResourceSink,
    ) -> Result<(), DesignResourceLimitError> {
        let Self {
            catalog,
            limits,
            key_bytes,
            workspace_entries,
            by_tag,
            active_by_tag,
            active_roles,
        } = self;
        let EdgeOutputRoleCatalog {
            operation,
            definitions,
            active,
        } = *catalog;
        fixed(operation);
        fixed(limits);
        fixed(key_bytes);
        fixed(workspace_entries);
        fixed_map(by_tag);
        fixed_map(active_by_tag);
        fixed_slice(active);
        let [definitions_field, active_field, tag_index_field, active_index_field, roles_field, key_field] =
            kind.fields();
        let location = |field, index| DesignResourceLocation {
            scope: DesignResourceScope::CommandState,
            owner: Some(*operation),
            field,
            index: Some(index),
        };
        // Complete actual retained collections precede nested key text; prior workspace totals
        // include temporary structures and cannot substitute for these actual owner fields.
        for (len, field) in [
            (definitions.len(), definitions_field),
            (active.len(), active_field),
            (by_tag.len(), tag_index_field),
            (active_by_tag.len(), active_index_field),
            (active_roles.len(), roles_field),
        ] {
            sink.charge_entries(len, location(field, source_index))?;
        }
        for (index, definition) in definitions.iter().enumerate() {
            let EdgeOutputRoleDefinition { tag, key } = definition;
            fixed(tag);
            sink.charge_text(key.resource_text(), location(key_field, index))?;
        }
        for association in active_roles {
            let ActiveAssociation {
                index,
                tag,
                definition,
            } = association;
            fixed(index);
            fixed(tag);
            fixed(definition);
        }
        Ok(())
    }
}
