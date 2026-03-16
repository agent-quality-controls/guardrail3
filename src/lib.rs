//! guardrail3 — composable code guardrails for Rust and TypeScript projects.

use proc_macro2 as _;

#[cfg(test)]
use proptest as _;

pub mod adapters;
pub mod app;
pub mod cli;
pub mod commands;
pub mod domain;
pub mod fs;
pub mod ports;
pub mod report;
