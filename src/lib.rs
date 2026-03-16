//! guardrail3 — composable code guardrails for Rust and TypeScript projects.

use proc_macro2 as _;

#[cfg(test)]
use proptest as _;

pub mod cli;
pub mod commands;
pub mod config;
pub mod discover;
pub mod fs;
pub mod hooks;
pub mod modules;
pub mod report;
pub mod rs;
pub mod ts;
