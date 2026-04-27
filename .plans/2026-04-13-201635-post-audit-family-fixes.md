Goal

Fix every remaining non-hexarch gap from the repo-wide family audit. The end state is: clippy no longer skips config checks on typed-invalid root configs and no longer exposes a fake source lane; release fails closed on non-workspace roots and stops advertising dead not-implemented surfaces; code explicitly reflects the session decision to keep rule 24 in code; hooks docs match the real package surface.

Approach

1. Add failing clippy regressions first:
   - typed-invalid but raw-parseable `clippy.toml` must still reach package config checks
   - source ingestion stub/public API must be removed cleanly from package surface
2. Add failing release regressions first:
   - root `[package]` without `[workspace]` must fail for config, filetree, and source ingestion
   - stale not-implemented error variants must be removed from the public ingestion contract
3. Fix clippy and release at the ingestion/package-contract boundary, not in downstream checks.
4. Resolve code family drift by updating package-facing docs/comments/tests to reflect the deliberate decision that `g3rs-code/ast-24-path-attr-with-reason` stays in code.
5. Fix stale hooks READMEs so they describe hooks, not topology/code.
6. Re-run the affected family suites plus `git diff --check`.
7. Write a standalone worklog and commit the fix set separately.

Key decisions

- Do not touch hexarch in this pass.
- Treat `g3rs-code/ast-24-path-attr-with-reason` as an intentional divergence from the old app because that decision was made explicitly in-session.
- Remove fake/public package surfaces when a lane is not real rather than keeping compatibility stubs.
- Fix behavior at ingestion boundaries so rules see the intended typed states.

Files to modify

- `packages/rs/clippy/g3rs-clippy-ingestion/**`
- `packages/rs/clippy/g3rs-clippy-types/src/lib.rs`
- `packages/rs/release/g3rs-release-ingestion/**`
- `packages/rs/release/g3rs-release-config-checks/README.md`
- `packages/rs/code/**` only where docs/tests/comments still claim rule 24 moved to arch
- `packages/rs/hooks/**/README.md`
