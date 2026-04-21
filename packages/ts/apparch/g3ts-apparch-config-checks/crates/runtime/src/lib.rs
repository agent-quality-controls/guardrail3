mod run;
mod support;
mod ts_apparch_config_01_types_dependency_direction;
mod ts_apparch_config_02_logic_dependency_direction;
mod ts_apparch_config_03_io_outbound_dependency_direction;
mod ts_apparch_config_04_io_inbound_dependency_direction;
mod ts_apparch_config_05_app_no_direct_outbound;
mod ts_apparch_config_06_types_purity;
mod ts_apparch_config_07_logic_purity;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_apparch_config_checks_assertions as _;
