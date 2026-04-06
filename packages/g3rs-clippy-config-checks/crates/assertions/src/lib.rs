use g3rs_clippy_config_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_clippy_config_01_max_struct_bools;
#[cfg(feature = "checks")]
pub mod rs_clippy_config_02_max_fn_params_bools;
#[cfg(feature = "checks")]
pub mod rs_clippy_config_03_too_many_lines_threshold;
#[cfg(feature = "checks")]
pub mod rs_clippy_config_04_too_many_arguments_threshold;
#[cfg(feature = "checks")]
pub mod rs_clippy_config_05_excessive_nesting_threshold;
#[cfg(feature = "checks")]
pub mod rs_clippy_config_06_test_relaxations;
#[cfg(feature = "checks")]
pub mod rs_clippy_config_07_cognitive_complexity_threshold;
#[cfg(feature = "checks")]
pub mod rs_clippy_config_08_type_complexity_threshold;
