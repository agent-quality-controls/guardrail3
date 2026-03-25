pub mod domain {
    pub use guardrail3_domain_config as config;
    pub use guardrail3_domain_report as report;
}

#[path = "../arch_helpers.rs"]
pub mod ts_arch_helpers;

pub mod app {
    pub use crate::ts_arch_helpers as arch_helpers;
    pub use guardrail3_app_core as core;
}

pub mod validate;
