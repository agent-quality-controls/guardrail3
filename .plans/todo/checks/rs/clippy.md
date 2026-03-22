# RS-CLIPPY — clippy.toml checker (22 rules)

**Input:** clippy.toml (one per workspace, plus per-crate)
**Parser:** TOML (`toml::Value`)
**Current code:** `crates/app/rs/validate/clippy_coverage.rs` + `config_files.rs`

## Existence + thresholds

### RS-CLIPPY-01: clippy.toml exists
- **Old ID:** R1
- **Severity:** Error
- **What:** clippy.toml must exist at workspace root
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
- **Status:** Planned

### RS-CLIPPY-10: too_many_arguments threshold
- **Old ID:** NEW
- **Severity:** Error
- **What:** `too-many-arguments-threshold` must be set (expected: 5). Controls clippy's `too_many_arguments` lint.
- **Status:** Planned

### RS-CLIPPY-11: excessive_nesting threshold
- **Old ID:** NEW
- **Severity:** Error
- **What:** `excessive-nesting-threshold` must be set. Controls clippy's nesting depth lint.
- **Status:** Planned

### RS-CLIPPY-21: cognitive_complexity threshold
- **Old ID:** R3 (was overloaded with other thresholds)
- **Severity:** Error
- **What:** `cognitive-complexity-threshold` must equal expected value (15). Controls clippy's cognitive complexity lint.
- **Status:** Implemented (in config_files.rs), missing from plan until now

### RS-CLIPPY-22: type_complexity threshold
- **Old ID:** R3 (was overloaded)
- **Severity:** Error
- **What:** `type-complexity-threshold` must equal expected value (75). Controls clippy's type complexity lint.
- **Status:** Implemented (in config_files.rs), missing from plan until now

**Note on thresholds:** All threshold rules (02, 03, 09, 10, 11, 21, 22) are exact-match checks — the value must equal the expected value, not just be present. This prevents setting `too-many-lines-threshold = 99999`.

## Ban completeness

### RS-CLIPPY-04: missing method ban
- **Old ID:** R4
- **Severity:** Error
- **What:** `disallowed-methods` must contain all expected method bans (43 methods: std::fs, std::env, std::process, reqwest, serde_json, toml, serde_yaml)
- **Note:** `std::process::abort` must be added to expected bans (currently only `std::process::exit` is there). abort() is worse than exit() — no unwinding, no destructors. No clippy lint covers it.
- **Status:** Implemented (42 methods), needs +1 (abort)

### RS-CLIPPY-05: missing type ban
- **Old ID:** R5
- **Severity:** Error
- **What:** `disallowed-types` must contain all expected type bans (10 base types: HashMap, HashSet, Mutex, RwLock, File, axum::extract::Json, axum::Json, axum::extract::Query, axum::extract::Form, std::any::Any)
- **Note:** `std::any::Any` added from source audit — `Box<dyn Any>` erases type safety, bypassing strongly-typed boundaries.
- **Status:** Implemented (9 types), needs +1 (Any)

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
- **Status:** Implemented for methods/types, needs extension to macros

### RS-CLIPPY-15: trivial/placeholder reason text
- **Old ID:** NEW (from audit)
- **Severity:** Warn
- **What:** Reason field must not be empty, a known placeholder ("TODO", "FIXME", "fix later", "TBD", "...", "reason"), or under 10 characters. Reason exists but communicates nothing.
- **Status:** Planned

### RS-CLIPPY-18: duplicate ban entries
- **Old ID:** NEW (from audit)
- **Severity:** Warn
- **What:** Same `path` appears more than once in ban list. Later entry can shadow reason text. clippy silently uses the last one.
- **Status:** Planned

## Per-crate checks

### RS-CLIPPY-12: per-crate clippy.toml inheritance
- **Old ID:** R2 (was overloaded)
- **Severity:** Warn
- **What:** Each workspace member crate should either have its own clippy.toml or inherit from the workspace root
- **Status:** Implemented

### RS-CLIPPY-13: per-crate clippy.toml must contain workspace bans
- **Old ID:** NEW (from audit — HIGH)
- **Severity:** Error
- **What:** When a per-crate clippy.toml exists, it REPLACES the workspace root (clippy does NOT merge). The per-crate file must contain all expected bans from the workspace root, or be absent (inheriting).
- **Why:** An agent creates a minimal per-crate clippy.toml → all 42 method bans silently dropped for that crate.
- **Profile interaction:** Uses the profile-resolved expected ban set. For library profile, this includes RS-CLIPPY-14's global-state bans. A library per-crate file must have BOTH the base bans AND the global-state bans.
- **Status:** Planned

## Profile-aware checks

### RS-CLIPPY-14: library profile must have global-state bans
- **Old ID:** NEW (from audit — HIGH)
- **Severity:** Error
- **What:** Library profile clippy.toml must have 4 additional global-state type bans: LazyLock, OnceLock, once_cell::sync::Lazy, once_cell::sync::OnceCell. The `_profile` param is currently ignored.
- **Status:** Planned

### RS-CLIPPY-16: avoid-breaking-exported-api setting
- **Old ID:** NEW (from audit)
- **Severity:** Warn
- **What:** `avoid-breaking-exported-api` should be explicitly set to `false` (suppresses useful lints when `true`, which is the default). For published library crates, `true` is legitimate — info note instead.
- **Status:** Planned

## Config hygiene

### RS-CLIPPY-17: allow-dbg-in-tests / allow-print-in-tests
- **Old ID:** NEW (from audit)
- **Severity:** Warn
- **What:** `allow-dbg-in-tests` and `allow-print-in-tests` should not be `true`. Keeps test output clean and deterministic.
- **Status:** Planned

### RS-CLIPPY-19: unrecognized top-level keys
- **Old ID:** NEW (from audit)
- **Severity:** Warn
- **What:** Flag any top-level key not in the known set (disallowed-methods, disallowed-types, disallowed-macros, thresholds, etc.). Catches typos like `disalowed-methods` that silently have no effect.
- **Status:** Planned

### RS-CLIPPY-20: disallowed-macros validation
- **Old ID:** NEW (from audit)
- **Severity:** Warn
- **What:** `disallowed-macros` section should contain expected macro bans (println!, eprintln!, dbg!, todo!, unimplemented!) with reason fields. Defense in depth alongside cargo workspace lint config — provides per-macro ban reasons in compiler errors.
- **Status:** Planned

## Explicitly rejected audit findings

| Finding | Why rejected |
|---------|-------------|
| `msrv` cross-reference with toolchain | Cross-file check — not the clippy checker's job. RS-CLIPPY-19 catches unexpected `msrv` key. Toolchain checker (RS-TOOLCHAIN) handles version. |
| Extra fields in ban entries (e.g., fake `allow_in_tests`) | Too unlikely to occur, low value. |
| Generated header comment check | Staleness is `guardrail3 check` command's job, not `validate`. |
