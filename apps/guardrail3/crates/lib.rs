#![recursion_limit = "2048"]
//! guardrail3 — composable code guardrails for Rust and TypeScript projects.

use clap as _;
use colored as _;
use garde as _;
use glob as _;
use guardrail3_adapters_inbound_cli as _;
use guardrail3_adapters_outbound_fs as _;
use guardrail3_adapters_outbound_report as _;
use guardrail3_adapters_outbound_tool_runner as _;
use guardrail3_app_arch_helpers as _;
use guardrail3_app_commands as _;
use guardrail3_app_core as _;
use guardrail3_app_hooks as _;
use guardrail3_app_rs_ast as _;
use guardrail3_app_rs_family_arch as _;
use guardrail3_app_rs_family_cargo as _;
use guardrail3_app_rs_family_clippy as _;
use guardrail3_app_rs_family_code as _;
use guardrail3_app_rs_family_deny as _;
use guardrail3_app_rs_family_deps as _;
use guardrail3_app_rs_family_fmt as _;
use guardrail3_app_rs_family_garde as _;
use guardrail3_app_rs_family_hexarch as _;
use guardrail3_app_rs_family_hooks_rs as _;
use guardrail3_app_rs_family_hooks_shared as _;
use guardrail3_app_rs_family_release as _;
use guardrail3_app_rs_family_test as _;
use guardrail3_app_rs_family_toolchain as _;
use guardrail3_app_rs_generate as _;
use guardrail3_app_rs_legacy_validate as _;
use guardrail3_app_rs_runtime as _;
use guardrail3_app_ts as _;
use guardrail3_domain_config as _;
use guardrail3_domain_modules as _;
use guardrail3_domain_project_tree as _;
use guardrail3_domain_report as _;
use guardrail3_outbound_traits as _;
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
