#![allow(
    clippy::missing_docs_in_private_items,
    reason = "assertions scaffold will gain rule-specific helpers later"
)]

#[cfg(feature = "checks")]
use g3rs_arch_source_checks_runtime as _;
use guardrail3_check_types as _;

#[cfg(feature = "checks")]
pub mod rs_arch_08a_feature_gated_exports;
#[cfg(feature = "checks")]
pub mod rs_arch_09_no_path_attr;
