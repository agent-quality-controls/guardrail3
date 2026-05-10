# Fix Pre-Commit Routing: Per-Owning-Unit Validation (RS)

## Summary

The previous `verifier independent` slice (`d5d0f5fcc..f3054c45c`) installed `scripts/g3rs/verify` and `scripts/g3ts/verify` but rewrote `.githooks/pre-commit` to call the Rust verifier with one hardcoded scope and dropped the TypeScript verifier line entirely along with the migration and lockfile shared checks. This worklog adds an end-to-end verifier-routing slice that:

- Routes each staged file to its owning adopted Rust workspace (Cargo.toml with `[workspace]` plus sibling `guardrail3-rs.toml`) via an upward-walk discovery loop in the hook.
- Restores all shared inline checks (merge-conflict, gitleaks, file-size, migration, lockfile).
- Adds source-check rules in `g3rs-hooks-source-checks` that enforce the routing contract and shared inline checks; gating regressions surface as `Severity::Error` so `g3rs validate --family hooks` exits non-zero.
- Adds an inward-only topology rule `g3rs-topology/no-nested-guardrail3-rs-toml` that fires from an adopted parent when a descendant marker is found.
- Mirrors the topology family for TypeScript (`packages/ts/topology/g3ts-topology-{types,ingestion,file-tree-checks}`) including the `g3ts-topology/no-nested-guardrail3-ts-toml` rule. Apps/guardrail3-ts wiring is deferred (this repo is RS-only).
- Extends hook ingestion to upward-walk for `.githooks/pre-commit` and `scripts/g3rs/verify` so `g3rs validate` from any sub-workspace surfaces the real hook's findings.
- Fixes the verifier-body source-check parser so it walks nested `case ... esac` constructs in `scripts/g3rs/verify` correctly.
- Bumps 23 source-check rules from `Severity::Warn` to `Severity::Error` on the missing case, so the gate actually blocks regressions instead of merely warning.
- Updates the `required-contract-command-present` rule to accept verifier delegation: when the hook calls `scripts/g3rs/verify --mode pre-commit` and the verifier exists, family-owned cargo commands inside the verifier satisfy the contract.
- Replaces synthesized-fixture-only tests with real-artifact `include_str!` tests against the actual `.githooks/pre-commit` and `scripts/g3rs/verify`, plus injection variants that prove the gate fires `Severity::Error` for: hardcoded scope, ambient-variable scope, env-override (`${VAR:-default}`), command-substitution default, default-fallback assignment (literal and variable-prefixed), missing required cargo command in verifier, and missing inline shared checks in the hook.
- Strengthens routing rule tests via a new `assert_error_finding` helper that pins `Severity::Error` (the previous `assert_finding` only checked id/message).
- Deletes 13 dead `#[cfg(test)]`-only modules in `g3rs-hooks-source-checks` that the production gate never invoked. Their concerns are now covered by `required-contract-command-present` (delegation-aware) and the `verifier-runs-*` rules.

Empirical end-to-end verification at the close of the slice (with `apps/guardrail3-rs/target/release/g3rs validate --path apps/guardrail3-rs --family hooks`):

| Scenario | Exit | Finding |
|---|---|---|
| Unmodified hook + verifier | 0 | only inventory `contract-trigger-coverage` Warn (documented limitation in the rule itself) |
| Hook scope `--scope "apps/guardrail3-rs"` | 1 | `routing-scope-not-hardcoded-literal` Error |
| Hook scope `--scope "$REPO_ROOT/apps/guardrail3-rs"` | 1 | `routing-scope-not-hardcoded-literal` Error |
| Verifier call `--scope "$REPO_ROOT"` after discovery loop | 1 | `routing-scope-not-hardcoded-literal` Error |
| Env-override `${VAR:-default}` | 1 | `routing-no-env-override` Error |
| Command-substitution default `$(... \|\| echo apps/...)` | 1 | `routing-no-env-override` Error |
| Default-fallback assignment `if [ -z "$X" ]; then X="apps/..."; fi` | 1 | `routing-no-env-override` Error |
| Default-fallback assignment with `$REPO_ROOT/apps/...` literal | 1 | `routing-no-env-override` Error |
| `cargo deny check` removed from verifier | 1 | `verifier-runs-cargo-deny-check` Error |
| `gitleaks protect --staged` removed from hook | 1 | `gitleaks-step-present` Error |

## Decisions

- **Adopted unit = marker pair (Cargo.toml `[workspace]` + sibling `guardrail3-rs.toml`).** Per `.plans/2026-04-04-142741-new-parsers.md`'s ownership model. Half-adopted directories (one of pair only) are rejected by source-check rules. Adopted units must not nest (enforced by `g3rs-topology/no-nested-guardrail3-rs-toml`, inward-only).
- **Files with no owning adopted unit are skipped silently.** Validators only see inputs inside the validated unit; the hook mirrors this by not invoking a validator for files outside any adopted unit. Prototypes, standalone publishable crates, `scripts/`, and root-level configs are correctly unmanaged.
- **Inward-only topology.** A topology rule fires from an adopted parent when a descendant marker is found. Validators stay scope-bound; no upward-walk inside validators. Late detection (validate child green; validate parent surfaces violation) is acceptable because the remediation is to delete the descendant's adoption marker, not its source.
- **Upward-walk in hook ingestion only.** When `g3rs validate --path X` runs on a sub-workspace, the hook ingestion walks up to repo root to find `.githooks/pre-commit` and `scripts/g3rs/verify`. This keeps the hook a repo-level concern without requiring per-workspace hook copies.
- **Sovereignty.** `g3rs-hooks-source-checks` and `g3ts-hooks-source-checks` are independent. No shared crate, no shared types. Both check the same root `.githooks/pre-commit` independently. Plan rules around routing and shared inline checks are duplicated AS RULES across packages, not duplicated AS BEHAVIOR in the hook.
- **Dead `#[cfg(test)]`-only modules deleted.** They never ran in the gate; their concerns are now covered by `required-contract-command-present` and `verifier-runs-*`. Specifically removed: `cargo_deny_step_present`, `cargo_dupes_step_present`, `cargo_machete_step_present`, `clippy_step_present`, `config_changes_trigger_validation`, `duplication_tool_is_cargo_dupes`, `fmt_step_present`, `guardrail_validate_staged_present`, `shared_target_dir_present`, `test_step_present`, `test_uses_workspace`. (`clippy_denies_warnings`, `cargo_dupes_excludes` retained because `required_contract_command_present` calls into their helpers.)

Decisions deliberately rejected:

- Routing rule severities at `Severity::Warn`. Plan demanded gating; Warn is advisory and `g3rs validate` exits 0 on Warns. Bumped to Error.
- Mirroring inline cargo commands in the hook for `concrete-lockfile-command`-style rules. Architecture moved cargo commands into the verifier; rule expanded to accept verifier delegation when the verifier exists.
- Outward (child checks ancestors) topology direction. Outward forces validators to traverse outside their declared scope. Kept inward.
- Upward-walk-from-discovered-units mitigation in the hook to compensate for missing topology rules. Rejected as a hack by the user; rule does the work instead.

## Key files

Plan: `.plans/2026-05-06-215807-fix-rust-verifier-workspace-routing-regression.md`. The "Audit Findings (Wave 2.5)" section codifies the post-implementation defects discovered by the comprehensive audit.

Audit reports:
- `/tmp/g3rs-hook-audit-report.md` — Wave 2.5 comprehensive RS hook-rules audit.
- `/tmp/g3rs-wave3-adversarial-report.md` — Wave 3 adversarial review.

Hook + verifier:
- `.githooks/pre-commit` — discovery-loop hook.
- `scripts/g3rs/verify` — unchanged from prior slice; verified to satisfy all `verifier-runs-*` rules.

Topology:
- `packages/rs/topology/g3rs-topology-file-tree-checks/.../no_nested_guardrail3_rs_toml/` — new RS rule.
- `packages/ts/topology/g3ts-topology-{types,ingestion,file-tree-checks}/` — new TS family (full adoption pack; not yet wired into apps/guardrail3-ts validate path).

Hook source-checks:
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/routing/` — five new routing rule modules + `support.rs` helpers (disallow-list ambient vars; command-substitution default detector; default-fallback assignment detector with variable-prefixed literal support).
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs` — depth-aware `verifier_mode_body` parser; `push_required_inventory` helper for Error-on-missing severity.
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run_tests/cases.rs` — real-artifact `include_str!` tests; `assert_error_finding` helper that pins severity.

Hook ingestion:
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/upward.rs` — new module with `find_file_entry`/`find_dir_entry` upward-walk helpers.
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/run.rs` — uses `upward::find_file_entry` for `.githooks/pre-commit`, `scripts/g3rs/verify`, and the modular `pre-commit.d/` directory.

## Out of scope (deferred)

- TypeScript verifier alignment. `scripts/g3ts/verify` still does discovery and rejects repo-root scope, contradicting the plan's "Forbidden in verifiers" clauses. This repo is RS-only for the slice's verification purposes.
- TS hook source-checks parity. F-4 (merge-conflict regex form) and F-7 (env-override / half-adopted detector parity) from the Wave 3 report are TS-side and TS-deferred.
- `apps/guardrail3-ts` wiring of the new `g3ts-topology` family. Rule package exists with full adoption pack; CLI wiring deferred.
- `g3ts-hooks-source-checks-runtime` workspace clippy debt (~105 strict-lint errors). Not gated by any verifier today; deferred.
- A3/A4 dedup + skip-silently rule asymmetry on RS side (TS family has them; RS does not). Deferred per the audit's classification of these as low-impact heuristic gaps.

## Next steps

If the slice is shipped, follow-up slices for the deferred TS items above. Order suggestion:
1. Wire `g3ts-topology` into `apps/guardrail3-ts` validate path (#9).
2. Mirror RS routing-rule parity on TS side (F-2 patterns; F-4 merge-conflict regex; F-7 sovereignty cleanups).
3. Reconcile `scripts/g3ts/verify` with the plan's verifier contract (no discovery; no repo-root rejection).
4. Address `g3ts-hooks-source-checks-runtime` clippy debt as a standalone hygiene slice.
5. Add RS-side `dedups-owning-units` and `skips-when-no-owning-unit` rules to mirror TS family.
