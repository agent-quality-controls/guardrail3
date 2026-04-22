mod rs_test_01_inline_test_bodies;
mod rs_test_04_ignore_reason;
mod rs_test_05_should_panic_expected;
mod rs_test_06_tautological_assertions;
mod rs_test_07_real_proof_site;
mod rs_test_08_weak_matches_assert;
mod rs_test_10_input_failures;
mod rs_test_16_assertions_modules_prove;
mod rs_test_17_external_harnesses_use_assertions;
mod run;
mod support;

#[cfg(feature = "checks")]
pub use run::check;
