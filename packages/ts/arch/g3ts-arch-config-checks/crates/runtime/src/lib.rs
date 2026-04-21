mod run;
mod support;
mod ts_arch_config_01_root_manifest_exists;
mod ts_arch_config_02_root_manifest_parseable;
mod ts_arch_config_03_declared_entrypoints_canonical;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_arch_config_checks_assertions as _;
