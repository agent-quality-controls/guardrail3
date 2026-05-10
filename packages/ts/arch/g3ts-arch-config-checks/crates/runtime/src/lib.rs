/// Checks that declared package entry points use canonical paths.
mod declared_entrypoints_canonical;
/// Checks that the workspace root `package.json` manifest exists.
mod root_manifest_exists;
/// Checks that the workspace root `package.json` manifest is parseable.
mod root_manifest_parseable;
/// Orchestrates the arch config rule fan-out.
mod run;
/// Shared accessor helpers for the rule implementations.
mod support;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_arch_config_checks_assertions as _;
