//! Reusable assertion helpers for the topology hook contract.

#[cfg(feature = "api")]
pub mod contract;

#[cfg(feature = "api")]
pub use contract::assert_contract_matches_expected_policy;
