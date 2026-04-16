#![allow(
    clippy::missing_docs_in_private_items,
    reason = "assertions scaffold will gain rule-specific helpers later"
)]

#[cfg(feature = "checks")]
use g3rs_apparch_config_checks_runtime as _;
use guardrail3_check_types as _;

#[cfg(feature = "checks")]
pub mod rs_apparch_config_01_types_dependency_direction;
#[cfg(feature = "checks")]
pub mod rs_apparch_config_02_logic_dependency_direction;
#[cfg(feature = "checks")]
pub mod rs_apparch_config_03_io_outbound_dependency_direction;
#[cfg(feature = "checks")]
pub mod rs_apparch_config_05_patch_replace_bypass;
#[cfg(feature = "checks")]
pub mod rs_apparch_config_06_same_layer_cycles;
#[cfg(feature = "checks")]
pub mod rs_apparch_config_07_dev_dependency_direction;
#[cfg(feature = "checks")]
pub mod rs_apparch_config_08_types_purity;
#[cfg(feature = "checks")]
pub mod rs_apparch_config_09_logic_purity;
#[cfg(feature = "checks")]
pub mod run;
