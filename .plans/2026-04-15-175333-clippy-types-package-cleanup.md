Goal

- Clean `packages/rs/clippy/g3rs-clippy-types` under the active rules without weakening any good rule.
- Stop only if the remaining signal is no longer clearly a package bug.

Approach

- Add the missing workspace-root policy files so the package is validated as a real active Rust workspace.
- Convert the single-crate package into an explicit one-crate workspace instead of leaving it in the old standalone shape.
- Make publish intent explicit and add the local release files only if the package is published.
- Add the missing cargo/clippy/deny/deps policy files and align the root manifest with the active baseline.
- Re-run validation and then judge the remaining source/API rules on their merits.

Key decisions

- Treat the active package shape as a Rust workspace even when it contains only one crate, because multiple active families currently normalize through workspace-local inputs.
- Fix clear root and manifest problems first before deciding whether the public-field rules on types crates are actually good.

Files to modify

- `packages/rs/clippy/g3rs-clippy-types/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-types/clippy.toml`
- `packages/rs/clippy/g3rs-clippy-types/deny.toml`
- `packages/rs/clippy/g3rs-clippy-types/guardrail3-rs.toml`
- `packages/rs/clippy/g3rs-clippy-types/rust-toolchain.toml`
- `packages/rs/clippy/g3rs-clippy-types/rustfmt.toml`
- `packages/rs/clippy/g3rs-clippy-types/src/lib.rs`
