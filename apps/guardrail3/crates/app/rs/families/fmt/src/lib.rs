pub mod domain {
    pub use guardrail3_domain_report as report;
}

#[path = "../../../checks/rs/fmt/mod.rs"]
mod inner;

pub use inner::*;
