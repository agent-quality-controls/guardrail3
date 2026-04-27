#[cfg(test)]
use g3rs_apparch_config_checks_assertions as _;
#[cfg(test)]
use guardrail3_rs_toml_parser as _;

mod dev_dependency_direction;
mod io_outbound_dependency_direction;
mod logic_dependency_direction;
mod logic_purity;
mod patch_replace_bypass;
mod run;
mod same_layer_cycles;
mod types_dependency_direction;
mod types_purity;

#[cfg(feature = "checks")]
pub use run::check;
