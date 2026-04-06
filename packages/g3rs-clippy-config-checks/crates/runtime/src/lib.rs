mod rs_clippy_config_01_max_struct_bools;
mod rs_clippy_config_02_max_fn_params_bools;
mod rs_clippy_config_03_too_many_lines_threshold;
mod rs_clippy_config_04_too_many_arguments_threshold;
mod rs_clippy_config_05_excessive_nesting_threshold;
mod rs_clippy_config_06_test_relaxations;
mod rs_clippy_config_07_cognitive_complexity_threshold;
mod rs_clippy_config_08_type_complexity_threshold;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
