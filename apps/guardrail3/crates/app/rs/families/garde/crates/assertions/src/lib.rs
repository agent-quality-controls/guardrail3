use guardrail3_app_rs_family_garde as _;

mod common;
pub mod facts;

pub mod rs_garde_01_dependency_present;
pub mod rs_garde_02_core_method_bans;
pub mod rs_garde_03_extractor_type_bans;
pub mod rs_garde_04_reqwest_json_ban;
pub mod rs_garde_05_struct_derive_validate;
pub mod rs_garde_06_additional_method_bans;
pub mod rs_garde_07_manual_deserialize_impl;
pub mod rs_garde_08_enum_derive_validate;
pub mod rs_garde_09_query_as_inventory;
pub mod rs_garde_10_input_failures;
pub mod rs_garde_11_field_level_constraints;
pub mod rs_garde_12_nested_validation_dive;
pub mod rs_garde_13_context_validation_surface;
pub mod rs_garde_14_guardrail_config_validate_call;
