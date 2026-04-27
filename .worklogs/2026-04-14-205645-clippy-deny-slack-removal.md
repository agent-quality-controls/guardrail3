# Summary
Removed 2 unproven exception comments from `packages/rs/clippy/g3rs-clippy-config-checks/deny.toml` and tightened the config to match the actual package graph. This eliminated the `g3rs-code/exception-comment-inventory` warning noise and exposed the real deny-family findings that had been hidden behind the slack.

# Decisions Made
- Removed the `regex` wrapper carveout because `cargo-deny` reported all three wrappers as unused in this package. Rejected keeping the exception because no current package dependency requires it.
- Changed `advisories.yanked` from `warn` to `deny` because `cargo deny check advisories` is currently clean. Rejected a preemptive downgrade for hypothetical future transitive issues.
- Kept the package otherwise unchanged. Rejected fixing the newly surfaced deny-family findings in the same commit because the requested scope was to remove slack, not to repair every resulting policy violation.

# Key Files For Context
- `packages/rs/clippy/g3rs-clippy-config-checks/deny.toml`
- `.worklogs/2026-04-14-201349-clippy-arch-mod-facade-reshaping.md`

# Next Steps
- `g3rs-deny/advisories-baseline` now needs a decision on the intended `yanked` policy baseline for unpublished package workspaces.
- `g3rs-deny/wrappers` now needs review of the managed ban-wrapper baseline, since the package no longer wants the old regex wrapper carveout.
- The package still has the previously ignored `test` family findings.
