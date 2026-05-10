#![expect(
    clippy::disallowed_methods,
    clippy::indexing_slicing,
    clippy::doc_markdown,
    reason = "test fixtures need direct std::fs and std::process::Command for real-artifact integration tests; selection.rs indexing into Vec<_> built by the same test is bounds-safe by construction; upward.rs doc strings reference filesystem path literals that doc_markdown wants backticked"
)]

mod helpers;
mod selection;
mod upward;
