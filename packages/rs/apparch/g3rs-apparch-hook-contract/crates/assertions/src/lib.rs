//! Assertion helpers that verify the apparch hook contract matches its expected policy.

#[cfg(feature = "api")]
pub mod contract;

#[cfg(feature = "api")]
pub use contract::assert_contract_matches_expected_policy;
