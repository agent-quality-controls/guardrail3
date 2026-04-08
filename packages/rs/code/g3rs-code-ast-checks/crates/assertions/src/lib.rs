use g3rs_code_ast_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_code_13_todo_macros;
#[cfg(feature = "checks")]
pub mod rs_code_15_direct_fs_usage;
#[cfg(feature = "checks")]
pub mod rs_code_16_panic_macro;
