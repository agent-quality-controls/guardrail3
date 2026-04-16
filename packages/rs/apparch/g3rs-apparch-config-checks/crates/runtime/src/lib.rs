#[cfg(test)]
use g3rs_apparch_config_checks_assertions as _;
#[cfg(test)]
use guardrail3_rs_toml_parser as _;

mod rs_apparch_config_01_types_dependency_direction;
mod rs_apparch_config_02_logic_dependency_direction;
mod rs_apparch_config_03_io_outbound_dependency_direction;
mod rs_apparch_config_05_patch_replace_bypass;
mod rs_apparch_config_06_same_layer_cycles;
mod rs_apparch_config_07_dev_dependency_direction;
mod rs_apparch_config_08_types_purity;
mod rs_apparch_config_09_logic_purity;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
