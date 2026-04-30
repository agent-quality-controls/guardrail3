mod run;

#[cfg(feature = "api")]
pub use run::check;

#[cfg(test)]
#[path = "lib_tests/mod.rs"]
mod lib_tests;
