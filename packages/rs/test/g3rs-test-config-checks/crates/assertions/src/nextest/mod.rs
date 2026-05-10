#![expect(
    clippy::module_name_repetitions,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
pub mod nextest_timeouts;
