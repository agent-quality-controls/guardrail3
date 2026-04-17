Goal
- Make `packages/rs/release/g3rs-release-filetree-checks` validate cleanly under current rules without changing the rules.

Approach
- Normalize root package metadata and add missing root policy/config files.
- Switch runtime from the local `crates/types` package to `g3rs-release-types` if the local types crate is only a facade.
- Make root and member publish intent explicit and add any package metadata required by release checks.
- Convert inline `#[cfg(test)]` modules and ad hoc sidecars to owned sidecar directories plus shared assertions modules.
- Re-run package tests and `guardrail3-rs validate --path ...` until the package is clean.

Key decisions
- Keep the fix package-local. If a rule contradicts the cleaned shape, stop and handle that as a separate bug.
- Follow the same flat-vs-directory test shape rule as the production files themselves.

Files to modify
- `packages/rs/release/g3rs-release-filetree-checks/Cargo.toml`
- `packages/rs/release/g3rs-release-filetree-checks/README.md`
- `packages/rs/release/g3rs-release-filetree-checks/guardrail3-rs.toml`
- `packages/rs/release/g3rs-release-filetree-checks/{clippy.toml,deny.toml,rustfmt.toml,rust-toolchain.toml}`
- `packages/rs/release/g3rs-release-filetree-checks/crates/*`
