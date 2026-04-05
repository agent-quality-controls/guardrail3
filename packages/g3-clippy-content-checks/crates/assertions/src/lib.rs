use g3_clippy_content_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_clippy_02_max_struct_bools;
#[cfg(feature = "checks")]
pub mod rs_clippy_03_max_fn_params_bools;
#[cfg(feature = "checks")]
pub mod rs_clippy_09_too_many_lines_threshold;
#[cfg(feature = "checks")]
pub mod rs_clippy_10_too_many_arguments_threshold;
#[cfg(feature = "checks")]
pub mod rs_clippy_11_excessive_nesting_threshold;
#[cfg(feature = "checks")]
pub mod rs_clippy_17_test_relaxations;
#[cfg(feature = "checks")]
pub mod rs_clippy_21_cognitive_complexity_threshold;
#[cfg(feature = "checks")]
pub mod rs_clippy_22_type_complexity_threshold;
