//! Material metadata over canonical declarations; no physical fact or applied shrinkage is inferred.
use super::EntityId;
use crate::value::{
    CanonicalInputTarget, CanonicalInputs, LengthDeclaration, ScalarInputDeclaration,
    ScalarInputKind, ValueProvenanceValidation,
};
use core::fmt;
use sc_units::Length;

/// Explicit supported application intent; merely storing it applies no transformation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialShrinkageStage {
    /// Roadmap8/G4-PROFILES.13's stage, after grading and before export.
    PostGradePreExport,
}

/// Authored shrinkage measurement targets and treatment/stage intent, validated within Material.
/// None of these fields supplies a measured value, zero fallback or an applied scale factor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterialShrinkage {
    /// X-direction Ratio declaration; unknown/derived states remain explicit.
    pub x: CanonicalInputTarget,
    /// Y-direction Ratio declaration, independently supplied even when intentionally shared with X.
    pub y: CanonicalInputTarget,
    /// Declared treatment/source record; existence, scope and truth remain unproved here.
    pub treatment: EntityId,
    /// Explicit supported application intent; no default stage is supplied.
    pub stage: MaterialShrinkageStage,
}

/// Editable Material metadata, without a second value/state/evidence cache.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaterialDefinition {
    /// Stable Material identity.
    pub id: EntityId,
    /// Source record for this metadata, including any composition claim.
    pub source: EntityId,
    /// Nonblank authored name; no machine identifier is inferred from it.
    pub name: String,
    /// Authored composition when supplied; None means unprovided, not an inferred composition.
    pub composition: Option<String>,
    /// Length target; a present authored width must be positive.
    pub width: CanonicalInputTarget,
    /// Boolean nap declaration, never silently defaulted to false.
    pub nap: CanonicalInputTarget,
    /// Boolean directional-print declaration.
    pub directional_print: CanonicalInputTarget,
    /// Boolean stripe declaration; later layout/grainline validation consumes its actual state.
    pub striped: CanonicalInputTarget,
    /// Boolean check declaration, separate from stripe intent.
    pub checked: CanonicalInputTarget,
    /// Optional declared X/Y shrinkage sources; no absent measurement becomes zero.
    pub shrinkage: Option<MaterialShrinkage>,
}

/// Fixed property role, independently of the declaration's id, state or physical truth.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialProperty {
    /// Positive material width when an authored/evaluated value exists.
    Width,
    /// Directional surface intent.
    Nap,
    /// Directional-print intent.
    DirectionalPrint,
    /// Stripe intent.
    Striped,
    /// Check intent.
    Checked,
    /// X-direction shrinkage measurement target, when supplied.
    ShrinkageX,
    /// Y-direction shrinkage measurement target, when supplied.
    ShrinkageY,
}
impl MaterialProperty {
    /// Complete fixed inspection/check order; optional shrinkage remains absent when unprovided.
    pub const ALL: [Self; 7] = [
        Self::Width,
        Self::Nap,
        Self::DirectionalPrint,
        Self::Striped,
        Self::Checked,
        Self::ShrinkageX,
        Self::ShrinkageY,
    ];

    /// Required canonical kind, not a value or uncertainty-state assertion.
    #[must_use]
    pub const fn expected_kind(self) -> ScalarInputKind {
        match self {
            Self::Width => ScalarInputKind::Length,
            Self::Nap | Self::DirectionalPrint | Self::Striped | Self::Checked => {
                ScalarInputKind::Boolean
            }
            Self::ShrinkageX | Self::ShrinkageY => ScalarInputKind::Ratio,
        }
    }
}

/// Actual borrowed canonical declaration, retaining its original representation and complete state.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MaterialPropertyDeclaration<'a> {
    /// Actual dedicated length record; no general scalar conversion is performed.
    Length(&'a LengthDeclaration),
    /// Borrowed general scalar record; property_declaration checks the selected role's kind.
    /// Constructing this carrier directly supplies no Material validation certificate.
    Scalar(&'a ScalarInputDeclaration),
}
impl fmt::Debug for MaterialPropertyDeclaration<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (family, kind) = match self {
            Self::Length(_) => ("dedicated length", ScalarInputKind::Length),
            Self::Scalar(value) => ("general scalar", value.kind()),
        };
        f.debug_struct("MaterialPropertyDeclaration")
            .field("family", &family)
            .field("kind", &kind)
            .finish_non_exhaustive()
    }
}

/// Required Material text field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialTextField {
    /// Name must be nonblank.
    Name,
    /// Supplied composition must be nonblank; an explicit None is legal.
    Composition,
}

/// Actual structural/current Material refusal, without invented physical data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MaterialError {
    /// Required authored text is blank; default diagnostics omit its raw contents.
    EmptyText {
        /// Material carrying the text.
        material: EntityId,
        /// Failed field.
        field: MaterialTextField,
    },
    /// Metadata identity aliases a canonical declaration identity in this supplied context.
    IdentityCollision {
        /// Aliased identity.
        material: EntityId,
    },
    /// A requested optional shrinkage property was not supplied.
    NoShrinkage {
        /// Material with no authored shrinkage sources.
        material: EntityId,
        /// Requested X/Y role.
        property: MaterialProperty,
    },
    /// The authored target's family/kind is incompatible with this role, before any value lookup.
    TargetKind {
        /// Material carrying the target.
        material: EntityId,
        /// Failed role.
        property: MaterialProperty,
        /// Required property kind.
        expected: ScalarInputKind,
        /// Actual authored target, including family/id/kind.
        target: CanonicalInputTarget,
    },
    /// Exact held target is absent; no same-kind or other-family peer substitutes for it.
    MissingDeclaration {
        /// Material carrying the target.
        material: EntityId,
        /// Failed role.
        property: MaterialProperty,
        /// Exact absent family/id/kind.
        target: CanonicalInputTarget,
    },
    /// A same-id canonical general scalar changed its actual kind.
    CanonicalKind {
        /// Material carrying the target.
        material: EntityId,
        /// Failed role.
        property: MaterialProperty,
        /// Current canonical identity.
        declaration: EntityId,
        /// Required kind.
        expected: ScalarInputKind,
        /// Current canonical kind.
        actual: ScalarInputKind,
    },
    /// Present authored or explicitly supplied evaluated width is zero or negative.
    NonPositiveWidth {
        /// Material carrying the width target.
        material: EntityId,
        /// Exact authored target.
        target: CanonicalInputTarget,
        /// Actual signed micrometres, without clamping.
        actual_micrometres: i64,
    },
}
impl fmt::Display for MaterialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Material: {self:?}")
    }
}
impl std::error::Error for MaterialError {}

/// Immutable Material metadata with current typed property references and unapplied shrinkage intent.
/// Source/evidence truth, full Design identity, grainline/layout, geometry and G4 pipeline/policy remain
/// separate. No method turns an unknown flag into false or stores a second numeric/state copy.
///
/// ```compile_fail
/// fn edit(material: &mut sc_core::ontology::Material) { material.definition.name.clear(); }
/// ```
/// ```compile_fail
/// fn edit(material: &mut sc_core::ontology::Material) { material.definition().shrinkage = None; }
/// ```
/// ```compile_fail
/// fn detached(material: &sc_core::ontology::Material)
///     -> sc_core::ontology::MaterialPropertyDeclaration<'static> {
///     let inputs = sc_core::value::CanonicalInputs::new(vec![], vec![]).unwrap();
///     material.property_declaration(sc_core::ontology::MaterialProperty::Width, &inputs).unwrap()
/// }
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct Material {
    definition: MaterialDefinition,
}
impl fmt::Debug for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Material")
            .field("has_composition", &self.definition.composition.is_some())
            .field("has_shrinkage", &self.definition.shrinkage.is_some())
            .finish_non_exhaustive()
    }
}
impl Material {
    pub(crate) fn resource_definition(&self) -> &MaterialDefinition {
        let Self { definition } = self;
        definition
    }

    /// Validate authored text and actual current property targets without resolving physical truth.
    ///
    /// Name is checked first, then supplied composition, then complete current validation. Known,
    /// Assumed and Preference widths require positive authored values. Unknown/Derived drafts accept
    /// without a value; the eventual evaluated width must pass validate_width_result separately.
    /// Shrinkage ratios remain signed Ratio data, not a checked or applied physical transformation.
    ///
    /// # Errors
    /// Returns the first blank text, identity, target/kind or present-width refusal.
    pub fn new(
        definition: MaterialDefinition,
        inputs: &CanonicalInputs,
    ) -> Result<Self, MaterialError> {
        if definition.name.trim().is_empty() {
            return Err(MaterialError::EmptyText {
                material: definition.id,
                field: MaterialTextField::Name,
            });
        }
        if definition
            .composition
            .as_ref()
            .is_some_and(|text| text.trim().is_empty())
        {
            return Err(MaterialError::EmptyText {
                material: definition.id,
                field: MaterialTextField::Composition,
            });
        }
        let material = Self { definition };
        material.validate_current(inputs)?;
        Ok(material)
    }

    /// Stable metadata identity.
    #[must_use]
    pub const fn id(&self) -> EntityId {
        self.definition.id
    }

    /// Original exact authored metadata; a shared borrow grants no mutator.
    #[must_use]
    pub const fn definition(&self) -> &MaterialDefinition {
        &self.definition
    }

    /// Original property target; None means only an unprovided optional shrinkage source.
    #[must_use]
    pub fn target(&self, property: MaterialProperty) -> Option<CanonicalInputTarget> {
        Some(match property {
            MaterialProperty::Width => self.definition.width,
            MaterialProperty::Nap => self.definition.nap,
            MaterialProperty::DirectionalPrint => self.definition.directional_print,
            MaterialProperty::Striped => self.definition.striped,
            MaterialProperty::Checked => self.definition.checked,
            MaterialProperty::ShrinkageX => self.definition.shrinkage?.x,
            MaterialProperty::ShrinkageY => self.definition.shrinkage?.y,
        })
    }

    /// Borrow the exact selected current property, checking its family/id/kind without reading a value.
    /// This proves only that target match; complete Material validation and physical/evidence checks
    /// remain separate. Even a present invalid width remains inspectable through this targeted view.
    ///
    /// # Errors
    /// Returns absent shrinkage, invalid authored target kind, missing target or current-kind mismatch.
    pub fn property_declaration<'a>(
        &self,
        property: MaterialProperty,
        inputs: &'a CanonicalInputs,
    ) -> Result<MaterialPropertyDeclaration<'a>, MaterialError> {
        let material = self.id();
        let target = self
            .target(property)
            .ok_or(MaterialError::NoShrinkage { material, property })?;
        let expected = property.expected_kind();
        let declared = match target {
            CanonicalInputTarget::Length(_) => ScalarInputKind::Length,
            CanonicalInputTarget::Scalar { kind, .. } => kind,
        };
        if declared != expected {
            return Err(MaterialError::TargetKind {
                material,
                property,
                expected,
                target,
            });
        }
        match target {
            CanonicalInputTarget::Length(identity) => inputs
                .length(identity)
                .map(MaterialPropertyDeclaration::Length)
                .ok_or(MaterialError::MissingDeclaration {
                    material,
                    property,
                    target,
                }),
            CanonicalInputTarget::Scalar { declaration, .. } => {
                let actual =
                    inputs
                        .scalar(declaration)
                        .ok_or(MaterialError::MissingDeclaration {
                            material,
                            property,
                            target,
                        })?;
                if actual.kind() != expected {
                    return Err(MaterialError::CanonicalKind {
                        material,
                        property,
                        declaration,
                        expected,
                        actual: actual.kind(),
                    });
                }
                Ok(MaterialPropertyDeclaration::Scalar(actual))
            }
        }
    }

    /// Check all present current property targets, then any available authored width.
    /// The fixed property order follows MaterialProperty::ALL. Referenced source/treatment/target ids
    /// are uses, not newly owned entities. This neither evaluates Derived nor invents Unknown values.
    ///
    /// # Errors
    /// Returns canonical identity collision, then first property refusal, then nonpositive width.
    pub fn validate_current(&self, inputs: &CanonicalInputs) -> Result<(), MaterialError> {
        if inputs.length(self.id()).is_some() || inputs.scalar(self.id()).is_some() {
            return Err(MaterialError::IdentityCollision {
                material: self.id(),
            });
        }
        for property in MaterialProperty::ALL {
            if self.target(property).is_some() {
                self.property_declaration(property, inputs)?;
            }
        }
        let width = match self.property_declaration(MaterialProperty::Width, inputs)? {
            MaterialPropertyDeclaration::Length(value) => {
                value.authored_value().ok().map(Length::as_micrometres)
            }
            MaterialPropertyDeclaration::Scalar(value) => value
                .authored_value()
                .ok()
                .and_then(|value| value.numeric_integer()),
        };
        if let Some(actual) = width {
            self.check_width(actual)?;
        }
        Ok(())
    }

    /// Require a positive explicitly supplied computed width; no computation/source proof is supplied.
    ///
    /// # Errors
    /// Returns NonPositiveWidth for zero/negative micrometres without clamping or changing metadata.
    pub fn validate_width_result(&self, width: Length) -> Result<(), MaterialError> {
        self.check_width(width.as_micrometres())
    }

    /// Registry/evidence facts and artifact policy remain unproved by structural readability.
    #[must_use]
    pub const fn provenance_validation(&self) -> ValueProvenanceValidation {
        ValueProvenanceValidation::DeferredToDesignAndG4
    }

    fn check_width(&self, actual_micrometres: i64) -> Result<(), MaterialError> {
        if actual_micrometres <= 0 {
            Err(MaterialError::NonPositiveWidth {
                material: self.id(),
                target: self.definition.width,
                actual_micrometres,
            })
        } else {
            Ok(())
        }
    }
}
