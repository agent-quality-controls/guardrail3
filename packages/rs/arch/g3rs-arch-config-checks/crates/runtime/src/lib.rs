//! Runtime crate for the g3rs-arch config-checks family.
#![allow(
    clippy::missing_docs_in_private_items,
    reason = "rule modules in this crate follow a uniform layout (private const ID/SELECTOR + pub(crate) check fn); doc-per-private-item adds no signal beyond the module's public docstring"
)]

#[cfg(test)]
use g3rs_arch_config_checks_assertions as _;

/// Rule: split crates exceeding the dependency-count threshold.
mod dependency_count_split;
/// Rule: validate feature contract across crate boundaries.
mod feature_contract;
/// Rule: forbid boundary crossing between architectural layers.
mod no_boundary_crossing;
/// Runtime orchestrator that wires inputs to rule checks.
mod run;
/// Rule: shared crates must declare the `shared = true` flag.
mod shared_flag_required;

#[cfg(feature = "checks")]
pub use run::check;
