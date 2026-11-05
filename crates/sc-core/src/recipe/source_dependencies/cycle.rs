//! Bounded iterative cycle admission over the exact original canonical Derived relation.
mod resources;
use super::{FormulaSourceDependencies, FormulaSourceDependency};
use core::fmt;
use std::collections::TryReserveError;

/// Explicit complete source and logical cycle-scratch allowances, without defaults.
/// Scratch is `2 * sources`: one color and one active frame per actual source. Existing source/
/// edge/offset/proof storage,allocator capacity/bytes,CPU and full operation graphs are separate.
/// ```compile_fail
/// let limits: sc_core::recipe::FormulaSourceCycleLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormulaSourceCycleLimits {
    sources: usize,
    workspace_records: usize,
}
impl FormulaSourceCycleLimits {
    /// Select both complete allowances explicitly; empty sources need zero cycle scratch.
    #[must_use]
    pub const fn new(sources: usize, workspace_records: usize) -> Self {
        Self {
            sources,
            workspace_records,
        }
    }
    /// Maximum complete source population, including unused and constant vertices.
    #[must_use]
    pub const fn sources(self) -> usize {
        self.sources
    }
    /// Maximum complete `2 * sources` scratch reservation.
    #[must_use]
    pub const fn workspace_records(self) -> usize {
        self.workspace_records
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    White,
    Gray,
    Black,
}
#[derive(Debug)]
struct Frame {
    source: usize,
    next_edge: usize,
    incoming_edge: Option<usize>,
}

/// Concrete original cycle, borrowing the actual relation and reusing the real active-frame path.
/// No graph/path clone,source/value cache or fabricated loader diagnostic is constructed.
#[derive(Debug)]
pub struct FormulaSourceCycle<'a> {
    dependencies: &'a FormulaSourceDependencies<'a>,
    frames: Vec<Frame>,
    start: usize,
    closing_edge: usize,
}
impl<'a> FormulaSourceCycle<'a> {
    /// Actual complete relation whose original edges close this cycle.
    #[must_use]
    pub const fn dependencies(&self) -> &'a FormulaSourceDependencies<'a> {
        self.dependencies
    }
    /// Nonempty cycle edge population; excludes any traversal prefix before the cycle.
    #[must_use]
    pub fn step_count(&self) -> usize {
        let Some(path) = self.frames.get(self.start..) else {
            unreachable!("private cycle start is an actual gray vertex on its active path");
        };
        path.len()
    }
    /// Original directed cycle edges in path order,ending with the actual gray-ancestor edge.
    /// Each retains actual source endpoints,canonical adapter and original name/span metadata.
    pub fn steps(&self) -> impl DoubleEndedIterator<Item = FormulaSourceDependency<'a>> + '_ {
        let Some(path) = self.frames.get(self.start + 1..) else {
            unreachable!("private cycle path contains its original start vertex");
        };
        path.iter()
            .map(|frame| {
                let Some(index) = frame.incoming_edge else {
                    unreachable!(
                        "every noninitial active cycle frame has its actual incoming edge"
                    );
                };
                edge(self.dependencies, index)
            })
            .chain(core::iter::once(edge(self.dependencies, self.closing_edge)))
    }
}

/// Complete resource/allocator refusal or concrete original cycle; no accepted partial proof.
#[derive(Debug)]
pub enum FormulaSourceCycleRefusal<'a> {
    /// Complete source quota,checked before scratch arithmetic/allocation.
    Sources {
        /// Selected allowance.
        allowed: usize,
        /// Complete actual population.
        required: usize,
    },
    /// Complete prospective scratch count cannot be represented.
    PopulationOverflow,
    /// Complete `2 * sources` scratch reservation cannot fit.
    WorkspaceRecords {
        /// Selected allowance.
        allowed: usize,
        /// Complete prospective reservation.
        required: usize,
    },
    /// Original color-vector reservation cause; no new diagnostic allocation.
    ColorAllocation(TryReserveError),
    /// Original frame-vector reservation cause; no accepted prefix escapes.
    FrameAllocation(TryReserveError),
    /// Actual path/closing edge; this is a component refusal,not invented persisted-loader context.
    Cycle(FormulaSourceCycle<'a>),
}

/// Refusal pinned to the actual immutable relation and selected explicit scratch allowances.
#[derive(Debug)]
pub struct FormulaSourceCycleError<'a> {
    dependencies: &'a FormulaSourceDependencies<'a>,
    limits: FormulaSourceCycleLimits,
    refusal: FormulaSourceCycleRefusal<'a>,
}
impl<'a> FormulaSourceCycleError<'a> {
    /// Actual complete relation; original source/edge/offset data were never copied or changed.
    #[must_use]
    pub const fn dependencies(&self) -> &'a FormulaSourceDependencies<'a> {
        self.dependencies
    }
    /// Selected complete allowances,independent of canonical content.
    #[must_use]
    pub const fn limits(&self) -> FormulaSourceCycleLimits {
        self.limits
    }
    /// Actual resource/allocator/cycle cause; no guessed failing source or statement ordinal.
    #[must_use]
    pub const fn refusal(&self) -> &FormulaSourceCycleRefusal<'a> {
        &self.refusal
    }
}
impl fmt::Display for FormulaSourceCycleError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.refusal {
            FormulaSourceCycleRefusal::Cycle(cycle) => write!(
                f,
                "canonical Derived source cycle: {} original edges",
                cycle.step_count()
            ),
            cause => write!(f, "canonical Derived cycle admission: {cause:?}"),
        }
    }
}
impl std::error::Error for FormulaSourceCycleError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.refusal {
            FormulaSourceCycleRefusal::ColorAllocation(error)
            | FormulaSourceCycleRefusal::FrameAllocation(error) => Some(error),
            FormulaSourceCycleRefusal::Sources { .. }
            | FormulaSourceCycleRefusal::PopulationOverflow
            | FormulaSourceCycleRefusal::WorkspaceRecords { .. }
            | FormulaSourceCycleRefusal::Cycle(_) => None,
        }
    }
}

/// Complete acyclic canonical Derived relation over these actual immutable source/edge owners.
/// This is no whole geometry/operation graph,current Design/host,recipe binding,execution order,
/// provider availability,rounding or computed-value proof. No topological order is manufactured.
/// ```compile_fail
/// fn replace<'a>(proof: &mut sc_core::recipe::FormulaAcyclicSourceDependencies<'a>,
///     other: &'a sc_core::recipe::FormulaSourceDependencies<'a>) { proof.dependencies = other; }
/// ```
/// ```compile_fail
/// fn escape<'a>(checked: &'a sc_core::recipe::FormulaCheckedSources<'a>)
///     -> sc_core::recipe::FormulaAcyclicSourceDependencies<'a> {
///     use sc_core::recipe::{FormulaSourceDependencyLimits, FormulaSourceCycleLimits};
///     let deps = checked.prepare_dependencies(FormulaSourceDependencyLimits::new(
///         usize::MAX, usize::MAX, usize::MAX)).unwrap();
///     deps.admit_acyclic(FormulaSourceCycleLimits::new(usize::MAX, usize::MAX)).unwrap()
/// }
/// ```
#[derive(Debug)]
pub struct FormulaAcyclicSourceDependencies<'a> {
    dependencies: &'a FormulaSourceDependencies<'a>,
    limits: FormulaSourceCycleLimits,
    workspace_records: usize,
}
impl<'a> FormulaAcyclicSourceDependencies<'a> {
    /// Exact original complete canonical Derived relation certified acyclic.
    #[must_use]
    pub const fn dependencies(&self) -> &'a FormulaSourceDependencies<'a> {
        self.dependencies
    }
    /// Explicit selected complete scratch allowances.
    #[must_use]
    pub const fn limits(&self) -> FormulaSourceCycleLimits {
        self.limits
    }
    /// Actual complete `2 * sources` scratch reservation,not allocator bytes or retained output.
    #[must_use]
    pub const fn workspace_records(&self) -> usize {
        self.workspace_records
    }
}
impl<'s> FormulaSourceDependencies<'s> {
    /// Admit all original canonical Derived edges with an iterative white/gray/black walk.
    /// Source and outgoing occurrence order select the first concrete cycle;no execution order is
    /// published. Every unused vertex/untaken/repeated edge participates. No values/providers are read.
    /// # Errors
    /// Complete source/checked scratch/allowance checks precede both full vector reservations.
    /// Original allocator causes stay inline. A cycle reuses its real active-frame vector and
    /// original closing edge without allocating another path or cloning graph data. No prefix proof.
    pub fn admit_acyclic<'a>(
        &'a self,
        limits: FormulaSourceCycleLimits,
    ) -> Result<FormulaAcyclicSourceDependencies<'a>, FormulaSourceCycleError<'a>>
    where
        's: 'a,
    {
        use FormulaSourceCycleRefusal as Refusal;
        let dependencies: &'a FormulaSourceDependencies<'a> = self;
        let refuse = |refusal| FormulaSourceCycleError {
            dependencies,
            limits,
            refusal,
        };
        let sources = dependencies.checked.sources().len();
        if sources > limits.sources {
            return Err(refuse(Refusal::Sources {
                allowed: limits.sources,
                required: sources,
            }));
        }
        let workspace_records = sources
            .checked_mul(2)
            .ok_or_else(|| refuse(Refusal::PopulationOverflow))?;
        if workspace_records > limits.workspace_records {
            return Err(refuse(Refusal::WorkspaceRecords {
                allowed: limits.workspace_records,
                required: workspace_records,
            }));
        }
        let mut colors = Vec::new();
        colors
            .try_reserve_exact(sources)
            .map_err(|error| refuse(Refusal::ColorAllocation(error)))?;
        let mut frames = Vec::new();
        frames
            .try_reserve_exact(sources)
            .map_err(|error| refuse(Refusal::FrameAllocation(error)))?;
        colors.resize(sources, Color::White);
        for root in 0..sources {
            if color(&colors, root) != Color::White {
                continue;
            }
            set_color(&mut colors, root, Color::Gray);
            frames.push(Frame {
                source: root,
                next_edge: offsets(dependencies, root).0,
                incoming_edge: None,
            });
            while let Some(frame) = frames.last_mut() {
                let (_, end) = offsets(dependencies, frame.source);
                if frame.next_edge == end {
                    set_color(&mut colors, frame.source, Color::Black);
                    frames.pop();
                    continue;
                }
                let index = frame.next_edge;
                frame.next_edge += 1;
                let Some(actual) = dependencies.edges.get(index) else {
                    unreachable!("complete private offsets bound the actual outgoing edge range");
                };
                match color(&colors, actual.supplier) {
                    Color::Black => {}
                    Color::White => {
                        set_color(&mut colors, actual.supplier, Color::Gray);
                        frames.push(Frame {
                            source: actual.supplier,
                            next_edge: offsets(dependencies, actual.supplier).0,
                            incoming_edge: Some(index),
                        });
                    }
                    Color::Gray => {
                        let Some(start) = frames
                            .iter()
                            .position(|frame| frame.source == actual.supplier)
                        else {
                            unreachable!("gray sources are exactly the actual active frame path");
                        };
                        return Err(refuse(Refusal::Cycle(FormulaSourceCycle {
                            dependencies,
                            frames,
                            start,
                            closing_edge: index,
                        })));
                    }
                }
            }
        }
        Ok(FormulaAcyclicSourceDependencies {
            dependencies,
            limits,
            workspace_records,
        })
    }
}
fn color(colors: &[Color], source: usize) -> Color {
    let Some(color) = colors.get(source) else {
        unreachable!("original admitted source edge/root indices are inside complete colors");
    };
    *color
}
fn set_color(colors: &mut [Color], source: usize, color: Color) {
    let Some(slot) = colors.get_mut(source) else {
        unreachable!("original admitted source edge/root indices are inside complete colors");
    };
    *slot = color;
}
fn offsets(dependencies: &FormulaSourceDependencies<'_>, source: usize) -> (usize, usize) {
    let (Some(start), Some(end)) = (
        dependencies.offsets.get(source),
        dependencies.offsets.get(source + 1),
    ) else {
        unreachable!(
            "complete private offsets include every original vertex and terminal sentinel"
        );
    };
    (*start, *end)
}
fn edge<'a>(
    dependencies: &'a FormulaSourceDependencies<'a>,
    index: usize,
) -> FormulaSourceDependency<'a> {
    let Some(edge) = dependencies.edges.get(index) else {
        unreachable!("private cycle path retains an original admitted relation edge index");
    };
    FormulaSourceDependency {
        checked: dependencies.checked,
        edge: *edge,
    }
}
