# g3rs-fmt-config-checks

Config checks for the `fmt` family.

This package validates typed root config content only.

- It does not discover root files.
- It does not own nested placement.
- It does not own dual-file conflicts.

Current scope:

- `g3rs-fmt/rustfmt-required-settings`: baseline rustfmt settings
- `g3rs-fmt/rustfmt-extra-settings-inventory`: extra setting inventory
- `g3rs-fmt/nightly-keys-on-stable`: nightly-only rustfmt keys on stable toolchains, including missing and parse blockers
- `g3rs-fmt/edition-mismatch`: rustfmt/Cargo edition consistency, including missing and parse blockers
- `g3rs-fmt/ignore-escape-hatch`: documented rustfmt `ignore` waivers

Ingestion remains responsible for:

- selecting the active root rustfmt config
- parsing root config files into typed or blocker states
- extracting Rust-policy waiver entries from `guardrail3-rs.toml`

`g3rs-fmt/rustfmt-config-exists`, `g3rs-fmt/per-crate-override`, and `g3rs-fmt/dual-file-conflict` live in `g3rs-fmt-filetree-checks`.
