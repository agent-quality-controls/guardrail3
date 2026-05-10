#[cfg(feature = "api")]
/// Internal module `contract`.
mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;
