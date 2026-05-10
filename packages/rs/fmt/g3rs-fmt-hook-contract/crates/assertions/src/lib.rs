//! Assertions that pin the g3rs fmt family hook contract shape.

#[cfg(feature = "api")]
pub mod contract;

#[cfg(feature = "api")]
pub use contract::assert_contract_matches_expected_policy;
