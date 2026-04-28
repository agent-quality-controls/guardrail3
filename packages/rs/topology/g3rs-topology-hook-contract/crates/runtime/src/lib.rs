mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;

#[cfg(test)]
#[path = "lib_tests/mod.rs"] // reason: owned sidecar tests for runtime lib facade.
mod lib_tests;
