use guardrail3_app_rs_family_garde as _;

mod common;
#[cfg(feature = "checks")]
pub mod facts;

#[cfg(feature = "checks")]
pub mod rs_garde_01_dependency_present;
#[cfg(feature = "checks")]
pub mod rs_garde_02_core_method_bans;
#[cfg(feature = "checks")]
pub mod rs_garde_03_extractor_type_bans;
#[cfg(feature = "checks")]
pub mod rs_garde_04_reqwest_json_ban;
#[cfg(feature = "checks")]
pub mod rs_garde_05_struct_derive_validate;
#[cfg(feature = "checks")]
pub mod rs_garde_06_additional_method_bans;
#[cfg(feature = "checks")]
pub mod rs_garde_07_manual_deserialize_impl;
#[cfg(feature = "checks")]
pub mod rs_garde_08_enum_derive_validate;
#[cfg(feature = "checks")]
pub mod rs_garde_09_query_as_inventory;
#[cfg(feature = "checks")]
pub mod rs_garde_10_input_failures;
#[cfg(feature = "checks")]
pub mod rs_garde_11_field_level_constraints;
#[cfg(feature = "checks")]
pub mod rs_garde_12_nested_validation_dive;
#[cfg(feature = "checks")]
pub mod rs_garde_13_context_validation_surface;
#[cfg(feature = "checks")]
pub mod rs_garde_14_guardrail_config_validate_call;
