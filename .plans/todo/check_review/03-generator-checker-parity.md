# Generator Checker Parity

## fmt / toolchain

- `rs/fmt` and `rs/toolchain` still need explicit canonical-drift protection against generated baseline.
- Add consistency tests similar in spirit to the stronger cargo/deny handling.

## clippy

- Generator and checker still disagree on global-state bans outside `library` profile:
  - generation includes pure-layer service crates
  - checker only expects library-profile bans
- Add an explicit generator-vs-checker consistency test.
- Update the contract to describe the real profile/layer decision model.

## deny

- `deny.toml` generation uses workspace-level `profile` where per-app/per-root `effective_profile` should drive the baseline.
- Add a regression test for that concrete bug.
- Add a real generator-vs-checker parity test for deny baseline.
- Replace stale hardcoded canonical service fixture in deny tests with generator-derived or parity-checked baseline.

## release

- Residual semantic-baseline hardening from the archived semver-release template is still unimplemented:
  - `release-plz.toml` workspace settings
  - `cliff.toml` git settings / parser coverage
- Promote those into explicit future rules or narrow the release contract.

## module registry completeness

- The confirmed missing Rust-relevant generator surface is `DENY_BANS_LIBRARY_IO`.
- Expose it through `list-modules` / `show-module`.
