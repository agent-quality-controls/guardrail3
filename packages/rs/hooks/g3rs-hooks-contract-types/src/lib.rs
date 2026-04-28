#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use types::{
    G3HookCommandRequirement, G3HookCriticalCommand, G3HookRequirement, G3HookTriggerPattern,
};
