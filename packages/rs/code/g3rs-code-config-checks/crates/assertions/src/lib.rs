#![allow(
    clippy::missing_docs_in_private_items,
    reason = "assertions scaffold will gain rule-specific helpers later"
)]

#[cfg(feature = "checks")]
use g3rs_code_config_checks_runtime as _;

#[cfg(feature = "checks")]
pub mod rs_code_config_07_exception_comment_inventory;
#[cfg(feature = "checks")]
pub mod rs_code_config_12_unsafe_code_lint;
