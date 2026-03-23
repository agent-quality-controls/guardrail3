# Hooks Hardening Lane

## Focus

Hooks are still the biggest architecture gap and should be treated as a new-family migration plus hardening effort.

## Deliverables

1. Define migrated `HOOK-SHARED` and `HOOK-RS` family shape.
2. Create executable-command parsing model.
3. Build golden hook fixture(s).
4. Attack hook bypasses broadly.
5. Follow `17-hooks-execution-plan.md` for the complete ordered implementation sequence.

## Current status

Implemented so far:
- migrated hook family scaffold under `apps/guardrail3/crates/app/rs/checks/hooks/`
- shared/Rust family split with `mod.rs`, `facts.rs`, `inputs.rs`
- Rust validate/report routing now includes migrated hook checks
- `rs hooks-validate` now uses the migrated Rust hook report path
- legacy `app/hooks/validate.rs` now delegates to the migrated Rust hook report whenever a Rust repo is present
- `ts hooks-validate` is now forced onto the legacy TS/non-Rust path instead of accidentally inheriting Rust hook routing in mixed repos
- hook generation now uses one shared workspace-root-aware content path across:
  - full generate
  - Rust-only generate
  - TypeScript-only generate
  - hooks-install
  - expected/diff output
- executable-command parser in `apps/guardrail3/crates/app/rs/checks/hooks/shell.rs`
- parser support for:
  - shebang capture
  - comment/prose separation
  - wrapped commands inside `if ! ...; then`
  - `cd ... && cargo ...` command extraction
  - dispatcher syntax
  - fail-open wrappers such as `|| true`, `|| :`, `|| echo ...`
  - `exit 0`
  - command substitutions inside assignments such as `FOO=$(git cat-file -s ...)`
- migrated shared rules currently implemented:
  - `HOOK-SHARED-01`
  - `HOOK-SHARED-02`
  - `HOOK-SHARED-03`
  - `HOOK-SHARED-04`
  - `HOOK-SHARED-05`
  - `HOOK-SHARED-06`
  - `HOOK-SHARED-07`
  - `HOOK-SHARED-08`
  - `HOOK-SHARED-09`
  - `HOOK-SHARED-10`
  - `HOOK-SHARED-11`
  - `HOOK-SHARED-12`
  - `HOOK-SHARED-13`
  - `HOOK-SHARED-14`
  - `HOOK-SHARED-15`
  - `HOOK-SHARED-16`
  - `HOOK-SHARED-17`
  - `HOOK-SHARED-18`
  - `HOOK-SHARED-19`
  - `HOOK-SHARED-20`
  - `HOOK-SHARED-21`
- migrated Rust rules currently implemented:
  - `HOOK-RS-01`
  - `HOOK-RS-02`
  - `HOOK-RS-03`
  - `HOOK-RS-04`
  - `HOOK-RS-05`
  - `HOOK-RS-06`
  - `HOOK-RS-07`
  - `HOOK-RS-08`
  - `HOOK-RS-09`
  - `HOOK-RS-10`
  - `HOOK-RS-11`
  - `HOOK-RS-12`
  - `HOOK-RS-13`
  - `HOOK-RS-14`
  - `HOOK-RS-15`
  - `HOOK-RS-16`
- first adversarial hardening pass applied to semantic command rules:
  - `HOOK-RS-08` now requires `validate` and `--staged` on the same executable command
  - `HOOK-SHARED-15` no longer treats `echo "conflict markers"` as a real conflict scan
  - `HOOK-SHARED-16` now looks for a real size-read command, not bare `MAX_FILE_SIZE` references
  - `HOOK-RS-16` no longer passes on comments alone; config triggers must appear in non-comment trigger logic
  - Rust/shared presence rules were tightened so echoed tool names like `echo "cargo clippy"` or `echo "gitleaks"` do not count as executed steps
- adversarial sidecar tests added for the above false-pass cases
- second hardening pass added direct sidecar coverage for previously-untested Rust hook rules:
  - `HOOK-RS-01`
  - `HOOK-RS-02`
  - `HOOK-RS-03`
  - `HOOK-RS-04`
  - `HOOK-RS-05`
  - `HOOK-RS-06`
  - `HOOK-RS-07`
  - `HOOK-RS-14`
  - `HOOK-RS-15`

Not implemented yet:
- remaining TS/non-Rust legacy hook cleanup
- remaining generator/checker parity fixes beyond `workspace_root`
- any deeper `RS-TEST-08` test hardening beyond parser reuse
- shared inventory/metadata rule sidecar coverage for:
  - `HOOK-SHARED-01`
  - `HOOK-SHARED-02`
  - `HOOK-SHARED-03`
  - `HOOK-SHARED-04`
  - `HOOK-SHARED-05`
  - `HOOK-SHARED-06`
  - `HOOK-SHARED-07`
  - `HOOK-SHARED-08`
  - `HOOK-SHARED-09`
  - `HOOK-SHARED-10`
  - `HOOK-SHARED-12`
  - `HOOK-SHARED-17`

## Fixture constraint

Current hook work has been kept inside the “empty golden fixture” constraint:
- safe now:
  - hook script content checks
  - cached pre-commit file checks
  - directory/file inventory from the `ProjectTree`
  - parser-driven command-shape checks
  - `ToolChecker`-backed installed/not-installed checks
- do not assume available yet:
  - full generated-project behavior
  - populated Rust workspace contents
  - end-to-end staged-file behavior against a real project tree

Call this out explicitly before doing work that needs more than folder structure plus config/hook files.

## Next concrete step

Next code step:
- add shared inventory/metadata sidecar tests for the remaining uncovered `HOOK-SHARED-*` rules
- then continue into generator/checker parity work and remaining TS/non-Rust cleanup
- note:
  - migrated Rust hook checks are already on the Rust validate path
  - legacy hook entrypoints no longer own Rust hook validation
  - remaining routing debt is now isolated to the TS/non-Rust hook path

## Main attack classes

- comments/prose masquerading as commands
- shebang and execute-bit issues
- `exit 0` and fail-open wrappers
- missing Rust steps
- bad `workspace_root`
- missing prerequisite tools
- config-change triggers absent
- executable `echo` / banner lines masquerading as real tool steps

## Cross-family note

`RS-TEST-08` mutation-hook checking should ultimately reuse the same executable-command model rather than diverging again.

## Success condition

Hook validation no longer relies on raw substring matching for semantic command presence.

See also: `17-hooks-execution-plan.md`.
