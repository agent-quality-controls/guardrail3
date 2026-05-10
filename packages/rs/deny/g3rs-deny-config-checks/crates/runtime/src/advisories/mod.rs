/// Rule implementation for `advisories baseline`.
mod advisories_baseline;
/// Rule implementation for `deprecated advisories`.
mod deprecated_advisories;
/// Rule implementation for `graph all features`.
mod graph_all_features;
/// Rule implementation for `graph no default features`.
mod graph_no_default_features;
/// Family entry point that runs all rules.
mod run;
/// Rule implementation for `stricter advisories inventory`.
mod stricter_advisories_inventory;

pub(crate) use run::check;
