mod app_no_direct_outbound;
mod io_inbound_dependency_direction;
mod io_outbound_dependency_direction;
mod logic_dependency_direction;
mod logic_purity;
mod run;
mod support;
mod types_dependency_direction;
mod types_purity;

#[cfg(feature = "checks")]
pub use run::check;

#[cfg(test)]
use g3ts_apparch_config_checks_assertions as _;
