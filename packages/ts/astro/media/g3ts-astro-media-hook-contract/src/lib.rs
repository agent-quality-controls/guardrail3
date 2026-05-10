/// Hook contract definition for the Astro media hook.
#[cfg(feature = "api")]
mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;
