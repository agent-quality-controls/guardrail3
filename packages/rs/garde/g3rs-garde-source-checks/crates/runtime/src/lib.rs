mod rs_garde_10_input_failures;
mod rs_garde_ast_01_struct_derive_validate;
mod rs_garde_ast_02_manual_deserialize_impl;
mod rs_garde_ast_03_enum_derive_validate;
mod rs_garde_ast_04_query_as_inventory;
mod rs_garde_ast_05_field_level_constraints;
mod rs_garde_ast_06_nested_validation_dive;
mod rs_garde_ast_07_context_validation_surface;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
