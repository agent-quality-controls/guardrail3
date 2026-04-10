use g3rs_garde_source_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_garde_ast_01_struct_derive_validate;
#[cfg(feature = "checks")]
pub mod rs_garde_ast_02_manual_deserialize_impl;
#[cfg(feature = "checks")]
pub mod rs_garde_ast_03_enum_derive_validate;
#[cfg(feature = "checks")]
pub mod rs_garde_ast_04_query_as_inventory;
#[cfg(feature = "checks")]
pub mod rs_garde_ast_05_field_level_constraints;
#[cfg(feature = "checks")]
pub mod rs_garde_ast_06_nested_validation_dive;
#[cfg(feature = "checks")]
pub mod rs_garde_ast_07_context_validation_surface;
#[cfg(feature = "checks")]
pub mod rs_garde_ast_08_guardrail_config_validate_call;
