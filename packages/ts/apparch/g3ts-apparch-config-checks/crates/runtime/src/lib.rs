mod run;
mod support;
mod ts_apparch_config_01_types_dependency_direction;
mod ts_apparch_config_02_logic_dependency_direction;
mod ts_apparch_config_03_io_outbound_dependency_direction;
mod ts_apparch_config_04_io_inbound_dependency_direction;
mod ts_apparch_config_05_app_no_direct_outbound;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_apparch_config_checks_assertions as _;
