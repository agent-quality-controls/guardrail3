#![recursion_limit = "2048"]
//! guardrail3 — composable code guardrails for Rust and TypeScript projects.

use clap as _;
use colored as _;
use garde as _;
use glob as _;
use guardrail3_app_arch_helpers as _;
use guardrail3_app_commands as _;
use guardrail3_app_rs_ast as _;
use guardrail3_app_rs_generate as _;
use guardrail3_app_rs_legacy_validate as _;
use guardrail3_shared_fs as _;
use guardrail3_validation_model as _;
use ignore as _;
use proc_macro2 as _;
use quote as _;
use semver as _;
use serde as _;
use serde_json as _;
use serde_yaml as _;
use syn as _;
use toml as _;
use toml_edit as _;
use tree_sitter as _;
use tree_sitter_javascript as _;
use tree_sitter_typescript as _;
use walkdir as _;

#[cfg(test)]
use proptest as _;
#[cfg(test)]
use tempfile as _;
// Package-level dependency used by the bin target and integration tests.
use guardrail3_adapters_outbound_fs as _;
use guardrail3_adapters_outbound_report as _;
use guardrail3_adapters_outbound_tool_runner as _;

pub mod domain {
    pub use guardrail3_domain_config as config;
    pub use guardrail3_domain_modules as modules;
    pub use guardrail3_domain_report as report;
}

pub mod app {
    pub use guardrail3_app_arch_helpers as arch_helpers;
    pub use guardrail3_app_core as core;
    pub use guardrail3_app_hooks as hooks;
    pub mod rs;
    pub use guardrail3_app_ts as ts;
}

pub mod adapters {
    pub mod inbound {
        pub use guardrail3_adapters_inbound_cli as cli;
    }
    pub mod outbound {
        pub use guardrail3_adapters_outbound_fs as fs;
        pub use guardrail3_adapters_outbound_report as report;
    }
}
