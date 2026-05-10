/// Hook contract definition for the fmt hook.
#[cfg(feature = "api")]
mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;
