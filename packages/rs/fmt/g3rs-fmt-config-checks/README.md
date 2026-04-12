# g3rs-fmt-config-checks

Config checks for the `fmt` family.

This package validates typed root config content only.

- It does not discover root files.
- It does not own nested placement.
- It does not own dual-file conflicts.

Current scope:

- `RS-FMT-CONFIG-01`: baseline rustfmt settings
- `RS-FMT-CONFIG-02`: extra setting inventory
- `RS-FMT-CONFIG-03`: nightly-only rustfmt keys on stable toolchains, including missing and parse blockers
- `RS-FMT-CONFIG-04`: rustfmt/Cargo edition consistency, including missing and parse blockers
- `RS-FMT-CONFIG-07`: documented rustfmt `ignore` escape hatches

Ingestion remains responsible for:

- selecting the active root rustfmt config
- parsing root config files into typed or blocker states
- extracting escape hatch entries from `guardrail3.toml`

`RS-FMT-FILETREE-01`, `RS-FMT-FILETREE-05`, and `RS-FMT-FILETREE-08` live in `g3rs-fmt-filetree-checks`.
