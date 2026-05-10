/// `assertions_modules_prove` module.
mod assertions_modules_prove;
/// `external_harnesses_use_assertions` module.
mod external_harnesses_use_assertions;
/// `ignore_reason` module.
mod ignore_reason;
/// `inline_test_bodies` module.
mod inline_test_bodies;
/// `input_failures` module.
mod input_failures;
/// `real_proof_site` module.
mod real_proof_site;
/// `run` module.
mod run;
/// `should_panic_expected` module.
mod should_panic_expected;
/// `support` module.
mod support;
/// `tautological_assertions` module.
mod tautological_assertions;
/// `weak_matches_assert` module.
mod weak_matches_assert;

#[cfg(feature = "checks")]
pub use run::check;
