mod declared_entrypoints_canonical;
mod root_manifest_exists;
mod root_manifest_parseable;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_arch_config_checks_assertions as _;
