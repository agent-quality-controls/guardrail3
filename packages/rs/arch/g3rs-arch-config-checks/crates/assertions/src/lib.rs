#[cfg(feature = "checks")]
use g3rs_arch_config_checks_runtime as _;
use guardrail3_check_types as _;

#[cfg(feature = "checks")]
pub mod rs_arch_05_no_boundary_crossing;
#[cfg(feature = "checks")]
pub mod rs_arch_06_shared_flag_required;
#[cfg(feature = "checks")]
pub mod rs_arch_07b_dependency_count_split;
#[cfg(feature = "checks")]
pub mod rs_arch_08b_feature_contract;
