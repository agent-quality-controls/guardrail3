mod run;
mod support;
mod ts_tsconfig_config_01_exists;
mod ts_tsconfig_config_02_parseable;
mod ts_tsconfig_config_03_extends_chain_resolves;
mod ts_tsconfig_config_04_extends_or_inline;
mod ts_tsconfig_config_05_strict_baseline;

#[cfg(feature = "checks")]
pub use run::check;
