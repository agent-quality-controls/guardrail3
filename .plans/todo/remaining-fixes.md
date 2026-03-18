# Remaining Fixes from Adversarial Audits

## Open Issues

### T-ARCH checks too narrow (#11)
T-ARCH-01/02 only look in `apps/*/src/modules/`. Should also support:
- `packages/{domain,ports,adapters}/` (monorepo pattern, used by graf)
- `src/{domain,ports,adapters}/` (flat pattern)
When type = "service" in config, check all patterns.

### T50 fires on non-web projects (#25)
Detect project type from package.json deps. Skip T50 when no HTTP framework found
(no express, fastify, hono, next with API routes). Or use the per-app type config
and only fire on type = "service".

### No --fix/auto-fix mode (#14)
For mechanical fixes: missing Cargo.toml fields, missing config files, creating
.cargo/mutants.toml. An auto-fix would save significant effort. Future feature.

### TS file length block comment counting (#9)
TS check_file_length uses simple line filter that doesn't track block comment state.
Lines inside `/* ... */` without leading `*` are counted as code. Should use
tree-sitter to count only non-comment nodes, or port RS's block-comment-aware filter.

### TS dependency allowlist (#17)
No equivalent of R-DEPS-01/02 for TypeScript. RS has allowlist-based dependency
checking. TS only has blocklist (T59). The allowlist model is stronger — new unknown
deps rejected by default. Would need per-package config in guardrail3.toml.

### RS env::var centralization (#26)
No check for scattered `std::env::var()` calls. Could add R-ENV-01 similar to
T30 (process.env). Low priority since clippy bans handle this for projects that
adopt them (graf already has `std::env::var` in disallowed_methods).

### RS #[ignore] without reason (#27)
No check for `#[ignore]` on test functions without a reason. The `#[ignore]`
attribute skips tests — without documentation, no one knows why the test was
disabled or when to re-enable it. Equivalent of TS T-TEST-04 (.skip() without
reason). R-TEST-07 exists but needs verification it actually works — the
adversarial audit flagged it as potentially broken.

### R2 per-crate clippy circular with R-ARCH-04 (#13 user feedback)
R2 warns "per-crate clippy.toml missing" but you can't generate per-crate
clippy.toml without `[rust.crates.*]` in guardrail3.toml. R-ARCH-04 tells you
to add crate config. But R2 fires before R-ARCH-04, so the user sees "clippy
missing" before understanding they need crate config first.

Fix: either suppress R2 when R-ARCH-04 would fire (no crate config = no
per-crate clippy expected), or change R2 message to say "configure crates in
guardrail3.toml first, then run rs generate."

### Per-app TS config in guardrail3.toml
No `[typescript.apps.*]` config exists yet. Needed for:
- Project type (service/content/library/extension)
- Per-app check customization
- T-ARCH, T-CONTENT, T50 gating on type
See ts-project-types.md for full spec.

### JSON compact mode
JSON output is always pretty-printed. For CI pipelines and log storage, compact
JSON (one line) is preferable. Add `--json-compact` or default to compact when
stdout is not a TTY.

### SARIF output format
Static Analysis Results Interchange Format. GitHub, VS Code, and other tools
consume SARIF directly. Would enable native integration with GitHub PR checks
and IDE problem panels. Future feature.

### --quiet mode
Only print summary line (or nothing on success, just exit code). Useful for CI
scripts that only care about pass/fail. `guardrail3 rs validate . --quiet`
returns exit code 0 or 1, no output.

### --severity filter
`--severity error` shows only errors. `--severity warn` shows errors + warnings.
Currently no way to filter by severity in text output.
