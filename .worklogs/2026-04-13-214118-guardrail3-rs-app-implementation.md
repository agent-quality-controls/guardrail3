Summary

Built the new standalone Rust CLI app at `apps/guardrail3-rs` and wired it to the migrated workspace-local package families, excluding `hexarch`. The app now crawls one pointed workspace, dispatches per-family package ingestion/checks, renders findings, preserves canonical family ordering, and keeps family-level runtime errors separate on stderr without dropping successful family results.

Decisions made

- Kept the app minimal.
  - Implemented only `validate --path ... [--family ...] [--inventory]`.
  - Rejected carrying over old app commands and legacy multi-language orchestration.
- Used one small shared app-types crate instead of splitting cli/report types.
  - Rejected extra crate splits from the earlier architecture sketch because they were unnecessary for this CLI.
- Wired families directly through package facades.
  - Each adapter calls only the public package ingestion/check facades for its family.
  - Rejected deep imports into package internals.
- Treated missing root config differently depending on family surface.
  - For `toolchain`, `fmt`, `clippy`, and `deny`, the adapter skips config ingestion on the specific root-config-not-found error and relies on the filetree lane to report the missing file.
  - Rejected aborting the whole family when the package already has a real filetree missing-config rule.
- Kept `deps` and `garde` family-level missing-policy errors as stderr family errors.
  - Adversarial review cleared this as acceptable under the current package model instead of forcing fake findings.
- Normalized explicit family selection to canonical family order.
  - Rejected preserving caller order because it makes output unstable across equivalent invocations.
- Changed CLI output handling to explicit stdout/stderr writes with flushes before `process::exit`.
  - Rejected `print!`/`eprint!` because buffered output can be lost on immediate exit.

Key files for context

- `.plans/2026-04-13-203607-guardrail3-rs-app-architecture.md`
- `.plans/2026-04-13-211757-guardrail3-rs-implementation.md`
- `apps/guardrail3-rs/Cargo.toml`
- `apps/guardrail3-rs/crates/types/app-types/src/lib.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/src/lib.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/src/main.rs`
- `apps/guardrail3-rs/crates/io/outbound/report/src/lib.rs`
- `apps/guardrail3-rs/crates/io/outbound/packages/src/lib.rs`
- `apps/guardrail3-rs/crates/io/outbound/packages/src/families/`

Next steps

- Move `packages/rs/hexarch` under a legacy location when you are ready to retire it from active package work.
- If you want default-family applicability to be quieter, add an explicit app-level applicability policy layer rather than special-casing more package adapter errors ad hoc.
