#![allow(
    clippy::missing_docs_in_private_items,
    reason = "assertions scaffold will gain rule-specific helpers later"
)]

#[cfg(feature = "checks")]
use g3rs_code_file_tree_checks_runtime as _;
use guardrail3_check_types as _;
