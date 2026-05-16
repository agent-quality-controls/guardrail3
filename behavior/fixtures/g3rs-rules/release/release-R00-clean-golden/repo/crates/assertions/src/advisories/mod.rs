#![expect(
    clippy::module_name_repetitions,
    reason = "submodule names mirror per-rule ids in the advisories family (advisories-baseline, deprecated-advisories, etc.); renaming the modules would break the rule-id-to-module mapping that other crates rely on"
)]

pub mod advisories_baseline;
pub mod deprecated_advisories;
pub mod graph_all_features;
pub mod graph_no_default_features;
pub mod stricter_advisories_inventory;
