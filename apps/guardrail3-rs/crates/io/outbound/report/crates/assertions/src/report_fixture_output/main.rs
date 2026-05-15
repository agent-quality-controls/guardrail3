#![allow(
    clippy::missing_docs_in_private_items,
    reason = "the fixture-output binary only wires CLI arguments into the assertions crate"
)]
#![allow(
    unused_crate_dependencies,
    reason = "Cargo exposes the package library crate to the binary target, but the fixture binary compiles its harness module directly"
)]

use std::io::Write as _;

use guardrail3_check_types as _;
use guardrail3_rs_app_types as _;
use guardrail3_rs_report as _;

mod fixture_output;

type MainResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> MainResult<()> {
    let fixture_paths = std::env::args().skip(1).collect::<Vec<_>>();
    let rendered = fixture_output::render(&fixture_paths)?;
    std::io::stdout().write_all(rendered.as_bytes())?;
    Ok(())
}
