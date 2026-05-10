#![expect(
    clippy::disallowed_methods,
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::too_many_lines,
    reason = "test fixtures need direct fs/process access, indexing, and unwrap for real-artifact integration tests"
)]

mod basic;
mod deps;
mod pipeline;
mod support;
