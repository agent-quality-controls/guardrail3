use g3rs_code_ast_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_code_13_todo_macros;
#[cfg(feature = "checks")]
pub mod rs_code_15_direct_fs_usage;
#[cfg(feature = "checks")]
pub mod rs_code_16_panic_macro;
#[cfg(feature = "checks")]
pub mod rs_code_17_impl_allow_blast_radius;
#[cfg(feature = "checks")]
pub mod rs_code_18_always_true_cfg_attr_bypass;
#[cfg(feature = "checks")]
pub mod rs_code_20_extern_allow;
#[cfg(feature = "checks")]
pub mod rs_code_21_fs_glob_import;
#[cfg(feature = "checks")]
pub mod rs_code_23_include_bypass;
#[cfg(feature = "checks")]
pub mod rs_code_30_input_failures;
#[cfg(feature = "checks")]
pub mod rs_code_32_test_expect_message_quality;
#[cfg(feature = "checks")]
pub mod rs_code_34_generic_parameter_cap;
#[cfg(feature = "checks")]
pub mod rs_code_36_string_dispatch_cap;
