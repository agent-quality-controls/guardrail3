//! guardrail3 — composable code guardrails for Rust and TypeScript projects.

use proc_macro2 as _;
use toml_edit as _;

#[cfg(test)]
use proptest as _;
#[cfg(test)]
use tempfile as _;

pub mod domain {
    pub mod config;
    pub mod modules;
    pub mod project_tree;
    pub mod report;
}

pub mod ports {
    pub mod outbound {
        #[path = "traits/mod.rs"]
        pub mod traits;
        pub use traits::*;
    }
}

pub mod app {
    pub mod arch_helpers;
    pub mod core;
    pub mod hooks;
    pub mod rs;
    pub mod ts;
}

pub mod adapters {
    pub mod inbound {
        pub mod cli;
    }
    pub mod outbound {
        pub mod fs;
        pub mod report;
        #[path = "tool-runner/mod.rs"]
        pub mod tool_runner;
    }
}

pub mod fs;
