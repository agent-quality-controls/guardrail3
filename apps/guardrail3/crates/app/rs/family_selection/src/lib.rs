mod selection;

pub use selection::resolve;

#[cfg(test)]
#[path = "selection_tests.rs"] // reason: owned sidecar tests stay adjacent without widening runtime visibility
mod selection_tests;
