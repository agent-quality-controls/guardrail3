pub mod domain {
    pub use guardrail3_domain_config as config;
    pub use guardrail3_domain_report as report;
}

#[path = "../../../checks/rs/rust_root_placement.rs"]
mod rust_root_placement;

#[path = "../../../checks/rs/arch/mod.rs"]
mod inner;

pub use inner::*;
