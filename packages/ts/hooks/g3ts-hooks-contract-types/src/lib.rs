/// Public type definitions for the G3TS hook contract.
#[cfg(feature = "api")]
mod contract;

#[cfg(feature = "api")]
pub use contract::{
    G3TsHookCommandRequirement, G3TsHookCriticalCommand, G3TsHookRequirement,
    G3TsHookTriggerPattern, PackageManager,
};
