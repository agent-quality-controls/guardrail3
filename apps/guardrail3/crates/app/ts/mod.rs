pub mod domain {
    pub use guardrail3_domain_config as config;
    pub use guardrail3_domain_report as report;
}

pub mod app {
    pub use guardrail3_app_arch_helpers as arch_helpers;
    pub use guardrail3_app_core as core;
}

pub mod validate;
