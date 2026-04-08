mod parse;
mod rs_code_ast_13_todo_macros;
mod rs_code_ast_15_direct_fs_usage;
mod rs_code_ast_16_panic_macro;
mod rs_code_ast_17_impl_allow_blast_radius;
mod rs_code_ast_18_always_true_cfg_attr_bypass;
mod rs_code_ast_20_extern_allow;
mod rs_code_ast_21_fs_glob_import;
mod rs_code_ast_23_include_bypass;
mod rs_code_ast_30_input_failures;
mod rs_code_ast_32_test_expect_message_quality;
mod rs_code_ast_34_generic_parameter_cap;
mod rs_code_ast_36_string_dispatch_cap;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
