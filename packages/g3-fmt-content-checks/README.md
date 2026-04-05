# g3-fmt-content-checks

Extracted rustfmt content checks for guardrail3.

This package is intentionally narrower than the in-app `fmt` family:

- it validates typed parsed config content only
- it does not discover authoritative files
- it does not decide repo-global ownership
- it does not report nested override placement, dual-file conflicts, or guardrail escape hatches

Current scope:

- `RS-FMT-02`: baseline rustfmt settings
- `RS-FMT-03`: extra setting inventory
- `RS-FMT-04`: nightly-only rustfmt keys on stable toolchains
- `RS-FMT-06`: rustfmt/Cargo edition consistency

The app remains responsible for:

- selecting the authoritative `rustfmt.toml`, `Cargo.toml`, and `rust-toolchain.toml`
- upstream parse-failure and missing-file reporting
- `RS-FMT-01`, `RS-FMT-05`, `RS-FMT-07`, and `RS-FMT-08`
