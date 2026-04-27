#[cfg(test)]
use g3rs_arch_config_checks_assertions as _;

mod dependency_count_split;
mod feature_contract;
mod no_boundary_crossing;
mod run;
mod shared_flag_required;

#[cfg(feature = "checks")]
pub use run::check;
