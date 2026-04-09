#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::wildcard_enum_match_arm,
    clippy::disallowed_methods,
    clippy::missing_docs_in_private_items,
    reason = "test code uses expect/panic for assertions and direct fs access for fixtures"
)]

mod basic;
mod deps;
mod pipeline;
