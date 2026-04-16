#[cfg(test)]
use g3rs_arch_config_checks_assertions as _;

mod rs_arch_05_no_boundary_crossing;
mod rs_arch_06_shared_flag_required;
mod rs_arch_07b_dependency_count_split;
mod rs_arch_08b_feature_contract;
mod run;

#[cfg(feature = "checks")]
pub use run::check;
