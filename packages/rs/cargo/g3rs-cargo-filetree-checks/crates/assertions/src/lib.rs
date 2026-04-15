#![allow(
    clippy::missing_docs_in_private_items,
    reason = "assertions scaffold will gain cargo filetree helpers later"
)]

#[cfg(feature = "checks")]
use g3rs_cargo_filetree_checks_runtime as _;
use guardrail3_check_types as _;

#[cfg(feature = "checks")]
pub mod run;
