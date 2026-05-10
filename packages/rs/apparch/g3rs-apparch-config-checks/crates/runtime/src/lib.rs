//! Runtime crate for the g3rs-apparch config-checks family.
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::type_complexity,
    reason = "rule modules in this crate follow a uniform layout (private const ID/SELECTOR + pub(crate) check fn); doc-per-private-item adds no signal beyond the module's public docstring. type_complexity arises from check signatures that destructure inputs into nested tuples; extracting type aliases would split the rule definition across files"
)]

#[cfg(test)]
use g3rs_apparch_config_checks_assertions as _;
#[cfg(test)]
use guardrail3_rs_toml_parser as _;

/// Rule: dev-dependency direction policy.
mod dev_dependency_direction;
/// Rule: outbound IO dependency direction policy.
mod io_outbound_dependency_direction;
/// Rule: logic-layer dependency direction policy.
mod logic_dependency_direction;
/// Rule: logic-layer purity policy.
mod logic_purity;
/// Rule: detect bypasses of `[patch]` and `[replace]` sections.
mod patch_replace_bypass;
/// Runtime orchestrator that wires inputs to rule checks.
mod run;
/// Rule: forbid cycles within the same architectural layer.
mod same_layer_cycles;
/// Rule: types-layer dependency direction policy.
mod types_dependency_direction;
/// Rule: types-layer purity policy.
mod types_purity;

#[cfg(feature = "checks")]
pub use run::check;
