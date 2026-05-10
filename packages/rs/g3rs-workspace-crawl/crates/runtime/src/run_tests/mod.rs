//! Integration tests for the workspace crawl runtime.
//!
//! Test fixtures here require real filesystem and process access to
//! initialise on-disk gitignore semantics.
#![allow(
    clippy::disallowed_methods,
    reason = "test fixtures need real fs and process access to exercise gitignore semantics"
)]

mod crawl_mechanics;
mod fixtures;
mod hidden_files;
mod ignore_state;
