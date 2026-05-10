/// `input_failures` module.
mod input_failures;
/// `owned_sidecar_shape` module.
mod owned_sidecar_shape;
/// `run` module.
mod run;
/// `runtime_assertions_split` module.
mod runtime_assertions_split;
/// `support` module.
mod support;
/// `test_support_generic` module.
mod test_support_generic;

#[cfg(feature = "checks")]
pub use run::check;
