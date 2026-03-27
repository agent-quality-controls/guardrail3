# RS-DENY — `deny.toml` checker

**Input:** effective cargo-deny config coverage over Rust validation roots

Accepted config filenames:
- `deny.toml`
- `.deny.toml`
- `.cargo/deny.toml`

Allowed config roots:
- validation root
- workspace roots
- standalone package roots that are not members of a workspace

Cargo-deny resolution model:
- config walks up from the manifest directory
- nearest config wins
- no merging
- nested deny configs can silently shadow parent policy

So the checker must validate:
- coverage: every Rust root is covered by an allowed deny config
- placement: deny configs only exist at allowed roots
- shadowing: nested deny configs below an allowed root are forbidden unless the deeper directory is itself another allowed root

**Parser:** TOML (`toml::Value`)

## Implementation mapping contract

- exactly one `RS-DENY-*` rule ID per production file
- exactly one rule-specific `*_tests/` module directory per production rule file
- `mod.rs` orchestrates only
- `facts.rs`, `inputs.rs`, and `deny_support.rs` may contain shared facts, typed inputs, and canonical baseline helpers only

Forbidden:

- grouped concern files such as `rs_deny_bans.rs`
- grouped family test files such as `deny_tests.rs`
- single-file sidecars as the long-term target; each rule should move to a rule-specific test module directory split by attack vector
- helper files that hide multiple rule predicates behind one API

**Current code:** `crates/app/rs/checks/rs/deny/**` + `crates/domain/modules/deny.rs`

**Canonical sources:**
- generator baseline: `apps/guardrail3/crates/domain/modules/deny.rs`
- historical validator: `deny_audit.rs`, `deny_bans.rs`, `deny_licenses.rs`, `deny_inventory.rs`

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-DENY-01 | R8 | Error | Every Rust root is covered by an effective deny config | Implemented |
| RS-DENY-02 | — | Error | deny config files may exist only at allowed roots | Implemented |
| RS-DENY-03 | — | Error | Nested deny configs that shadow a parent root are forbidden | Implemented |
| RS-DENY-04 | R9 | Warn | Deprecated `[advisories]` fields (`vulnerability`, `notice`, `unsound`) | Implemented |
| RS-DENY-05 | R10 | Error | `[advisories]` must set `unmaintained = "workspace"` and `yanked = "warn"` | Implemented |
| RS-DENY-06 | R11 | Info | Advisory settings stricter than baseline are inventoried | Implemented |
| RS-DENY-07 | — | Error | `[graph].all-features = true` must be set | Implemented |
| RS-DENY-08 | — | Error | `[graph].no-default-features = false` must be set | Implemented |
| RS-DENY-09 | R12 | Error | `[bans].deny` must contain the full canonical baseline ban set for the active profile | Implemented |
| RS-DENY-10 | R12 | Warn | `[bans].multiple-versions` weaker than `"deny"` | Implemented |
| RS-DENY-11 | R13 | Info | `[bans].highlight` setting inventoried when it differs from `"all"` | Implemented |
| RS-DENY-12 | — | Error | `[bans].allow-wildcard-paths = true` must be set | Implemented |
| RS-DENY-13 | — | Warn | `[bans].wildcards` missing / default-reliant / weaker-than-expected is inventoried | Implemented |
| RS-DENY-14 | R14 | Error | `[licenses]` must contain the baseline allow list and `[licenses.private].ignore = true` | Implemented |
| RS-DENY-15 | R15 | Warn/Info | `confidence-threshold` must be `0.8` or stricter; stricter values are inventoried | Implemented |
| RS-DENY-16 | — | Warn | `[licenses].allow` must not include copyleft licenses | Implemented |
| RS-DENY-17 | — | Info | `[licenses].exceptions` entries are inventoried | Implemented |
| RS-DENY-18 | R16 | Error | `[sources].unknown-registry = "deny"` and `unknown-git = "deny"` | Implemented |
| RS-DENY-19 | R16 | Error | `[sources].allow-registry` must allow only crates.io (git or sparse URL) | Implemented |
| RS-DENY-20 | — | Warn/Info | `[sources].allow-git` entries are warned and inventoried | Implemented |
| RS-DENY-21 | R17 | Warn | `[[bans.features]]` must ban `tokio` feature `full` and keep the canonical tokio allow list | Implemented |
| RS-DENY-22 | R18 | Info | Extra feature bans beyond tokio are inventoried | Implemented |
| RS-DENY-23 | R19 | Warn/Info | `[bans.skip]` entries: malformed entry or missing/non-string reason warns; valid entries inventory | Implemented |
| RS-DENY-24 | R20 | Warn/Info | `[advisories].ignore` entries: malformed entry or missing/non-string reason warns; valid entries inventory | Implemented |
| RS-DENY-25 | — | Error/Warn | `[bans].allow` is forbidden; overlap with deny baseline is an explicit error | Implemented |
| RS-DENY-26 | — | Info | `[bans].deny` entries without `reason` are inventoried | Implemented |
| RS-DENY-27 | — | Warn | Duplicate entries in `deny`, `skip`, `ignore`, or `[[bans.features]]` are warned | Implemented |
| RS-DENY-28 | — | Warn | Unknown keys / unsupported schema in critical deny sections are warned | Implemented |
| RS-DENY-29 | — | Warn | Advisory ignore accumulation over threshold `5` is warned | Implemented |
| RS-DENY-30 | — | Error/Info | Ban-entry `wrappers` must match canonical policy where managed; project-specific wrappers inventory otherwise | Implemented |

## Canonical baseline

Do not hardcode a prose count like “35 expected bans”.

The active expected deny set must be derived from the canonical generator baseline in
`apps/guardrail3/crates/domain/modules/deny.rs`, plus:
- profile-aware additions for `library`
- explicit audited additions such as `lazy_static`

The checker should have a generator-vs-validator consistency test so the baseline cannot drift silently.

## Key reconciliations from the audit

### Placement and coverage

The old plan assumed “one `deny.toml` at workspace root”.
That is not how cargo-deny behaves.

cargo-deny walks up from the manifest directory and nearest config wins, so:
- nested deny files can shadow parent policy
- validation must model effective coverage, not just file existence
- deny placement must be checked similarly to clippy placement

### Ban settings vs ban completeness

The old plan merged `[bans].multiple-versions = "deny"` and ban-list completeness into one rule.
That is wrong.

They are separate concerns:
- ban-list completeness is hard correctness
- `multiple-versions` is a threshold-like policy knob and may be relaxed with a warning

### Sources policy

The old plan merged all source checks into one rule.
That hid an important distinction:
- `unknown-*` policy is guardrail-owned
- `allow-registry` is guardrail-owned and should stay exact
- `allow-git` is user-owned but risky and should be inventoried / warned, not blanket-error'ed

### Graph correctness

The old plan only added `all-features = true`.
That is incomplete.

For hard coverage, the deny checker must also require:
- `no-default-features = false`

Otherwise cargo-deny can still check only a subset of the graph.

## Rule details

### RS-DENY-01 — Effective deny coverage (Error)

Every Rust root must be covered by an effective deny config found via cargo-deny walk-up semantics.

Covered means:
- the root itself has an allowed deny config, or
- an allowed ancestor config is the nearest config cargo-deny would resolve

Uncovered Rust roots are errors.

### RS-DENY-02 / RS-DENY-03 — Placement and shadowing (Error)

Deny configs are allowed only at:
- validation root
- workspace roots
- standalone package roots not inside a workspace

Any deny config below one of those roots is an error unless the deeper directory is itself another allowed root.

This applies to all accepted filenames:
- `deny.toml`
- `.deny.toml`
- `.cargo/deny.toml`

### RS-DENY-07 / RS-DENY-08 — Graph coverage (Error)

Required:
- `[graph]` exists
- `all-features = true`
- `no-default-features = false`

These are not cosmetic defaults. They control which dependency graph cargo-deny inspects.

### RS-DENY-09 — Canonical ban coverage (Error)

`[bans].deny` must contain the full canonical deny baseline for the active profile.

That expected set must be derived from the generator baseline, not duplicated manually in prose.

For `library` profile, the baseline includes library-IO bans from the canonical module.

### RS-DENY-10 — `multiple-versions` floor (Warn)

`multiple-versions = "deny"` is the preferred hardening baseline.

If the value is weaker, warn rather than error.
This is a deliberate design choice: it behaves like a threshold/floor, not like a must-match invariant.

### RS-DENY-12 / RS-DENY-13 — Wildcard policy

Hard requirement:
- `allow-wildcard-paths = true`

Inventory / warning:
- `wildcards` missing
- `wildcards` relying on defaults
- `wildcards` weaker than expected

Do not treat stricter user values as hard failures.

### RS-DENY-14 / RS-DENY-15 / RS-DENY-16 / RS-DENY-17 — License policy

Guardrail-owned:
- baseline allow list present
- `private.ignore = true`
- `confidence-threshold >= 0.8`

Warn / inventory:
- copyleft licenses in allow list
- license exceptions in `[licenses].exceptions`

### RS-DENY-18 / RS-DENY-19 / RS-DENY-20 — Source policy

Guardrail-owned:
- `unknown-registry = "deny"`
- `unknown-git = "deny"`
- `allow-registry` must allow only crates.io

Accepted crates.io values:
- `https://github.com/rust-lang/crates.io-index`
- `sparse+https://index.crates.io/`

Any extra registry is an error.

`allow-git` is not automatically forbidden by the plan, but it is risky enough to warn and inventory.

### RS-DENY-21 / RS-DENY-22 — Feature-ban policy

Guardrail-managed baseline:
- `tokio` must ban `full`
- `tokio` must keep the canonical explicit `allow = [...]` set from the generator baseline

Extra feature bans are inventoried.

### RS-DENY-23 / RS-DENY-24 / RS-DENY-26 / RS-DENY-29 — Exception hygiene

For `skip` and `ignore` entries:
- malformed entry shape warns
- missing or non-string `reason` warns
- valid entries inventory

Ban entries without reasons are inventoried.

Excessive advisory ignores warn once the count exceeds `5`, but this is a pressure rule, not a core correctness rule.

### RS-DENY-25 — `[bans].allow` override channel

`[bans].allow` is a direct escape hatch because allow entries override deny entries.

Policy:
- non-empty `allow` should be treated as a problem by default
- any overlap with the deny baseline is an explicit error

### RS-DENY-27 / RS-DENY-28 / RS-DENY-30 — Schema hardening

Warn on:
- duplicate deny / skip / ignore / feature-ban entries
- unknown keys in critical sections
- unsupported schema in critical sections

Validate `wrappers` deliberately:
- canonical managed wrapper expectations must be enforced where the baseline requires them
- otherwise wrappers are at least inventoried so weakening cannot hide behind crate-name-only matching

## Known implementation bugs to fix during migration

- Accept sparse crates.io URL in `allow-registry`
- Do not use `Path::exists()` directly; stay inside the filesystem abstraction
- Add proper malformed-entry handling for `skip` / `ignore`
- Add missing `.as_inventory()` consistency where inventory output is intended
- Stop validating every deny file under a workspace path bag; validate effective coverage and shadowing instead

## Test focus for implementation

Adversarial tests should try to break the checker, not confirm the happy path.

Must cover:
- uncovered Rust roots
- nested shadow `deny.toml`
- `.deny.toml` / `.cargo/deny.toml` precedence and conflicts
- missing graph keys
- sparse crates.io registry URL
- non-empty `allow-git`
- non-empty `[bans].allow`
- malformed `skip` / `ignore` entries
- duplicate deny / ignore entries
- wrapper weakening that preserves crate name
- generator-vs-validator baseline drift
