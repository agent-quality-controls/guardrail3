# RS-CLIPPY — clippy.toml checker (22 rules)

**Input:** `clippy.toml` / `.clippy.toml` at allowed Rust policy roots
**Parser:** TOML (`toml::Value`)
**Current code:** `crates/app/rs/checks/rs/clippy/**` + `crates/domain/modules/clippy/**`

## Implementation mapping contract

- exactly one `RS-CLIPPY-*` rule ID per production file
- exactly one sidecar `*_tests.rs` file per production rule file
- `mod.rs` orchestrates only
- `facts.rs`, `inputs.rs`, and `clippy_support.rs` may contain shared facts, typed inputs, canonical baseline data, and normalization helpers only

Forbidden:

- grouped threshold files such as `rs_clippy_thresholds.rs`
- grouped family test files such as `clippy_tests.rs`
- helper files that hide multiple rule predicates behind one API

## Decisions frozen from architecture/policy review

These are the current contract decisions for the clippy family. They override older drift across `config_files.rs`, `clippy_coverage.rs`, `domain/modules/clippy/mod.rs`, and the by-file design docs.

### Scope / goal

- guardrail3 should manage as much clippy hardening as can be applied universally and sanely
- prefer enforcing upstream enforcement knobs rather than trusting source scans alone
- every allow/ignore/removal escape hatch must carry a reason
- the clippy checker is a hardening/configuration checker, not a style-preference checker

### Allowed clippy.toml locations

A `clippy.toml` is allowed only at:

- the validation root (`ProjectTree.root`)
- Rust workspace roots
- package roots that are NOT members of a workspace

Anything else is forbidden shadowing.

This is intentionally NOT coupled to hex-arch or folder naming like `apps/` / `packages/`.

### Coverage rule

Every Rust workspace root and every standalone package root must be covered by some allowed `clippy.toml`:

- its own local `clippy.toml`, or
- an allowed ancestor `clippy.toml` (for example the validation root)

If a Rust unit is uncovered, that is an Error.

### Shadowing rule

If a `clippy.toml` exists below an allowed policy root, it is an Error unless that deeper directory is itself another allowed policy root.

This means:

- root + workspace root is allowed
- root + standalone package root is allowed
- workspace member crate `clippy.toml` is NOT allowed
- arbitrary intermediate shadow configs are NOT allowed

### Managed key set

guardrail3 manages these clippy keys:

- thresholds:
  - `too-many-lines-threshold`
  - `cognitive-complexity-threshold`
  - `too-many-arguments-threshold`
  - `type-complexity-threshold`
  - `max-struct-bools`
  - `max-fn-params-bools`
  - `excessive-nesting-threshold`
- booleans:
  - `avoid-breaking-exported-api`
  - `allow-dbg-in-tests`
  - `allow-print-in-tests`
- ban arrays:
  - `disallowed-methods`
  - `disallowed-types`
  - `disallowed-macros`

Other clippy keys may exist, but `RS-CLIPPY-19` should only warn for keys that look like typos of guardrail-managed keys, not merely “not guardrail-managed”.

### Frozen threshold values

- `max-struct-bools = 3`
- `max-fn-params-bools = 3`
- `too-many-lines-threshold = 75`
- `too-many-arguments-threshold = 7`
- `excessive-nesting-threshold = 4`
- `cognitive-complexity-threshold = 15`
- `type-complexity-threshold = 75`

All threshold checks are exact-match checks.

### Macro bans

`disallowed-macros` is guardrail-managed and required. Baseline macro bans:

- `println!`
- `eprintln!`
- `dbg!`
- `todo!`
- `unimplemented!`

### Reason policy

Every guardrail-managed ban entry must have a real `reason`.

- missing `reason` is Warn
- plain string entries with no table/reason are Warn
- trivial / placeholder reasons (`TODO`, `FIXME`, `TBD`, `...`, `reason`, too-short text) are Warn

### Profile policy

- `library` profile adds global-state type bans
- `avoid-breaking-exported-api` should be explicitly set
- default hardened value is `false`
- `true` may be informationally tolerated for published library crates, but is otherwise Warn
- `allow-dbg-in-tests = true` and `allow-print-in-tests = true` are Warn

### Source of truth after cleanup

After the clippy family is fully reconciled:

1. policy decisions in this plan
2. canonical generated module in `domain/modules/clippy/`
3. checker expectations in `app/rs/checks/rs/clippy`
4. old validator/tests only as migration evidence

The generator and checker must match exactly.

## Existence + thresholds

### RS-CLIPPY-01: clippy coverage exists
- **Old ID:** R1
- **Severity:** Error
- **What:** Every Rust workspace root and every standalone package root must be covered by an allowed `clippy.toml` at:
  - validation root
  - workspace root
  - standalone package root
- **Why:** An uncovered Rust unit silently falls back to clippy defaults.
- **Status:** Implemented

### RS-CLIPPY-02: max-struct-bools threshold
- **Old ID:** R2
- **Severity:** Error
- **What:** `max-struct-bools` must be set (expected: 3)
- **Status:** Implemented

### RS-CLIPPY-03: max-fn-params-bools threshold
- **Old ID:** R3
- **Severity:** Error
- **What:** `max-fn-params-bools` must be set (expected: 3)
- **Status:** Implemented

### RS-CLIPPY-09: too_many_lines threshold
- **Old ID:** NEW
- **Severity:** Error
- **What:** `too-many-lines-threshold` must be set (expected: 75). Controls clippy's `too_many_lines` lint for function length.
- **Status:** Implemented

### RS-CLIPPY-10: too_many_arguments threshold
- **Old ID:** NEW
- **Severity:** Error
- **What:** `too-many-arguments-threshold` must be set (expected: 7). Controls clippy's `too_many_arguments` lint.
- **Status:** Implemented

### RS-CLIPPY-11: excessive_nesting threshold
- **Old ID:** NEW
- **Severity:** Error
- **What:** `excessive-nesting-threshold` must be set (expected: 4). Controls clippy's nesting depth lint.
- **Status:** Implemented

### RS-CLIPPY-21: cognitive_complexity threshold
- **Old ID:** R3 (was overloaded with other thresholds)
- **Severity:** Error
- **What:** `cognitive-complexity-threshold` must equal expected value (15). Controls clippy's cognitive complexity lint.
- **Status:** Implemented

### RS-CLIPPY-22: type_complexity threshold
- **Old ID:** R3 (was overloaded)
- **Severity:** Error
- **What:** `type-complexity-threshold` must equal expected value (75). Controls clippy's type complexity lint.
- **Status:** Implemented

**Note on thresholds:** All threshold rules (02, 03, 09, 10, 11, 21, 22) are exact-match checks — the value must equal the expected value, not just be present. This prevents setting `too-many-lines-threshold = 99999`.

## Ban completeness

### RS-CLIPPY-04: missing method ban
- **Old ID:** R4
- **Severity:** Error
- **What:** `disallowed-methods` must contain the full hardened baseline derived from canonical guardrail modules, including `std::process::abort`
- **Note:** The exact count is no longer authoritative by itself. The canonical module content is authoritative once reconciled.
- **Status:** Implemented

### RS-CLIPPY-05: missing type ban
- **Old ID:** R5
- **Severity:** Error
- **What:** `disallowed-types` must contain all expected type bans (10 base types: HashMap, HashSet, Mutex, RwLock, File, axum::extract::Json, axum::Json, axum::extract::Query, axum::extract::Form, std::any::Any)
- **Note:** `std::any::Any` added from source audit — `Box<dyn Any>` erases type safety, bypassing strongly-typed boundaries.
- **Status:** Implemented

### RS-CLIPPY-06: extra method ban (inventory)
- **Old ID:** R6
- **Severity:** Info
- **What:** Report method bans not in expected baseline (user additions)
- **Status:** Implemented

### RS-CLIPPY-07: extra type ban (inventory)
- **Old ID:** R7
- **Severity:** Info
- **What:** Report type bans not in expected baseline
- **Status:** Implemented

## Ban quality

### RS-CLIPPY-08: ban entry without reason field
- **Old ID:** extracted from R4/R5
- **Severity:** Warn
- **What:** Every ban entry in ALL three ban sections (`disallowed-methods`, `disallowed-types`, `disallowed-macros`) must have a `reason` field. Plain string entries (no table format) also flagged.
- **Why separate:** "Ban missing" vs "ban has no reason" are different problems.
- **Scope:** Covers methods, types, AND macros (RS-CLIPPY-20 checks macro presence, this checks reason quality across all sections).
- **Status:** Implemented

### RS-CLIPPY-15: trivial/placeholder reason text
- **Old ID:** NEW (from audit)
- **Severity:** Warn
- **What:** Reason field must not be empty, a known placeholder ("TODO", "FIXME", "fix later", "TBD", "...", "reason"), or under 10 characters. Reason exists but communicates nothing.
- **Status:** Implemented

### RS-CLIPPY-18: duplicate ban entries
- **Old ID:** NEW (from audit)
- **Severity:** Warn
- **What:** Same `path` appears more than once in ban list. Later entry can shadow reason text. clippy silently uses the last one.
- **Status:** Implemented

## Placement / coverage checks

### RS-CLIPPY-12: clippy.toml placement is allowed
- **Old ID:** R2 (was overloaded)
- **Severity:** Error
- **What:** `clippy.toml` is allowed only at:
  - validation root
  - Rust workspace roots
  - standalone package roots not belonging to a workspace
- **What fails:** nested member-crate `clippy.toml`, intermediate shadow configs, or any other disallowed placement
- **Status:** Implemented

### RS-CLIPPY-13: allowed local policy roots must contain the full baseline
- **Old ID:** NEW (from audit — HIGH)
- **Severity:** Error
- **What:** Any allowed local policy root below the validation root (`clippy.toml` at workspace root / standalone package root) must contain the full profile-resolved guardrail baseline for:
  - thresholds
  - method bans
  - type bans
  - macro bans
- **Why:** A local policy root replaces inherited config for everything below it.
- **Profile interaction:** Library roots must include RS-CLIPPY-14 global-state bans too.
- **Status:** Implemented

## Profile-aware checks

### RS-CLIPPY-14: library profile must have global-state bans
- **Old ID:** NEW (from audit — HIGH)
- **Severity:** Error
- **What:** Library profile clippy.toml must have 4 additional global-state type bans: LazyLock, OnceLock, once_cell::sync::Lazy, once_cell::sync::OnceCell. The `_profile` param is currently ignored.
- **Status:** Implemented

### RS-CLIPPY-16: avoid-breaking-exported-api setting
- **Old ID:** NEW (from audit)
- **Severity:** Warn
- **What:** `avoid-breaking-exported-api` should be explicitly set to `false` (suppresses useful lints when `true`, which is the default). For published library crates, `true` is legitimate — info note instead.
- **Status:** Implemented

## Config hygiene

### RS-CLIPPY-17: allow-dbg-in-tests / allow-print-in-tests
- **Old ID:** NEW (from audit)
- **Severity:** Warn
- **What:** `allow-dbg-in-tests` and `allow-print-in-tests` should not be `true`. Keeps test output clean and deterministic.
- **Status:** Implemented

### RS-CLIPPY-19: unrecognized top-level keys
- **Old ID:** NEW (from audit)
- **Severity:** Warn
- **What:** Flag top-level keys that look like typos of guardrail-managed keys (for example `disalowed-methods`). Do NOT warn on arbitrary user-owned clippy keys just because guardrail3 does not manage them.
- **Status:** Implemented

### RS-CLIPPY-20: disallowed-macros validation
- **Old ID:** NEW (from audit)
- **Severity:** Error
- **What:** `disallowed-macros` section must contain expected macro bans (println!, eprintln!, dbg!, todo!, unimplemented!) with reason fields. Defense in depth alongside cargo workspace lint config — provides per-macro ban reasons in compiler errors.
- **Status:** Implemented

## Explicitly rejected audit findings

| Finding | Why rejected |
|---------|-------------|
| `msrv` cross-reference with toolchain | Cross-file check — not the clippy checker's job. `msrv` is a known user-owned clippy key and should not trigger RS-CLIPPY-19. Toolchain checker (RS-TOOLCHAIN) handles version. |
| Extra fields in ban entries (e.g., fake `allow_in_tests`) | Too unlikely to occur, low value. |
| Generated header comment check | Staleness is `guardrail3 check` command's job, not `validate`. |
