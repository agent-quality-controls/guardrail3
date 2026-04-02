use guardrail3_app_rs_family_hexarch as _;

#[cfg(feature = "checks")]
pub mod dependency_facts;
#[cfg(feature = "checks")]
pub mod dependency_integrity;
#[cfg(feature = "checks")]
pub mod dependency_policy;
#[cfg(feature = "checks")]
pub mod inventory_contract;
#[cfg(feature = "checks")]
pub use inventory_contract::{
    HEXARCH_INVENTORY_RULE_IDS, PATCH_REPLACE_BYPASS_RULE_ID, assert_inventory_ids,
    assert_inventory_result,
};
#[cfg(feature = "checks")]
pub mod structure;
#[cfg(feature = "checks")]
pub mod workspace_policy;
