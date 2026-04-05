use g3_garde_ast_checks_runtime as _;

mod common;

#[cfg(feature = "checks")]
pub mod rs_garde_05_struct_derive_validate;
#[cfg(feature = "checks")]
pub mod rs_garde_07_manual_deserialize_impl;
#[cfg(feature = "checks")]
pub mod rs_garde_08_enum_derive_validate;
#[cfg(feature = "checks")]
pub mod rs_garde_09_query_as_inventory;
#[cfg(feature = "checks")]
pub mod rs_garde_11_field_level_constraints;
#[cfg(feature = "checks")]
pub mod rs_garde_12_nested_validation_dive;
#[cfg(feature = "checks")]
pub mod rs_garde_13_context_validation_surface;
#[cfg(feature = "checks")]
pub mod rs_garde_14_guardrail_config_validate_call;
