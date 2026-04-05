#![allow(
    clippy::missing_docs_in_private_items,
    reason = "runtime is currently a scaffold; rule modules will add the real documented surface"
)]

mod inputs;
mod run;

#[cfg(test)]
use g3_cargo_content_checks_assertions as _;

pub use run::check;
