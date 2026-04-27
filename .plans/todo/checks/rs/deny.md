# RS-DENY — `deny.toml` checker

> Superseded as the primary family plan by [`.plans/by_family/rs/deny.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/deny.md).
> Keep this file as a detailed rule ledger and migration/history reference.

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
| g3rs-deny/deprecated-advisories | R9 | Warn | Deprecated `[advisories]` fields (`vulnerability`, `notice`, `unsound`) | Implemented |
| g3rs-deny/advisories-baseline | R10 | Error | `[advisories]` must set `unmaintained = "workspace"` and `yanked = "warn"` | Implemented |
| g3rs-deny/stricter-advisories-inventory | R11 | Info | Advisory settings stricter than baseline are inventoried | Implemented |
| g3rs-deny/graph-all-features | — | Error | `[graph].all-features = true` must be set | Implemented |
| g3rs-deny/graph-no-default-features | — | Error | `[graph].no-default-features = false` must be set | Implemented |
| RS-DENY-09 | R12 | Error | `[bans].deny` must contain the full canonical baseline ban set for the active profile | Implemented |
| g3rs-deny/highlight-inventory | R12 | Warn | `[bans].multiple-versions` weaker than `"deny"` | Implemented |
| g3rs-deny/allow-wildcard-paths | R13 | Info | `[bans].highlight` setting inventoried when it differs from `"all"` | Implemented |
| g3rs-deny/wildcards-inventory | — | Error | `[bans].allow-wildcard-paths = true` must be set | Implemented |
| g3rs-deny/license-allow-baseline | — | Warn | `[bans].wildcards` missing / default-reliant / weaker-than-expected is inventoried | Implemented |
| g3rs-deny/confidence-threshold | R14 | Error | `[licenses]` must contain the baseline allow list and `[licenses.private].ignore = true` | Implemented |
| g3rs-deny/copyleft-allowlist | R15 | Warn/Info | `confidence-threshold` must be `0.8` or stricter; stricter values are inventoried | Implemented |
| g3rs-deny/unknown-sources-policy | — | Warn | `[licenses].allow` must not include copyleft licenses | Implemented |
| RS-DENY-17 | — | Error/Info | `[licenses].exceptions` entries must be documented with reasons; valid entries are inventoried | Implemented |
| g3rs-deny/allow-git-inventory | R16 | Error | `[sources].unknown-registry = "deny"` and `unknown-git = "deny"` | Implemented |
| g3rs-deny/tokio-full-ban | R16 | Error | `[sources].allow-registry` must contain exactly one canonical crates.io sparse URL | Implemented |
| g3rs-deny/extra-feature-bans-inventory | — | Warn/Info | `[sources].allow-git` entries are warned and inventoried | Implemented |
| g3rs-deny/skip-hygiene | R17 | Warn | `[[bans.features]]` must ban `tokio` feature `full` and keep the canonical tokio allow list | Implemented |
| g3rs-deny/ignore-hygiene | R18 | Info | Extra feature bans beyond tokio are inventoried | Implemented |
| g3rs-deny/duplicate-entries | R19 | Error/Info | `[bans.skip]` entries must use documented table form with a string `reason`; valid entries inventory | Implemented |
| g3rs-deny/unknown-keys | R20 | Error/Info | `[advisories].ignore` entries must use documented table form with a string `reason`; valid entries inventory | Implemented |
| RS-DENY-25 | — | Error/Warn | `[bans].allow` is forbidden; overlap with deny baseline is an explicit error | Implemented |
| RS-DENY-26 | — | Error | `[bans].deny` entries without `reason` are errors | Implemented |
| g3rs-deny/license-exceptions-inventory | — | Warn | Duplicate entries in `deny`, `skip`, `ignore`, or `[[bans.features]]` are warned | Implemented |
| g3rs-deny/allow-override-channel | — | Warn | Unknown keys / unsupported schema in critical deny sections are warned | Implemented |
| g3rs-deny/extra-deny-bans-inventory | — | Warn | Advisory ignore accumulation over threshold `5` is warned | Implemented |
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

Malformed active `guardrail3.toml` policy context is also an error under this family entrypoint,
because deny cannot safely choose the correct profile-sensitive baseline when profile routing is invalid.

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

### g3rs-deny/graph-all-features / g3rs-deny/graph-no-default-features — Graph coverage (Error)

Required:
- `[graph]` exists
- `all-features = true`
- `no-default-features = false`

These are not cosmetic defaults. They control which dependency graph cargo-deny inspects.

### RS-DENY-09 — Canonical ban coverage (Error)

`[bans].deny` must contain the full canonical deny baseline for the active profile.

That expected set must be derived from the generator baseline, not duplicated manually in prose.

For `library` profile, the baseline includes library-IO bans from the canonical module.

### g3rs-deny/highlight-inventory — `multiple-versions` floor (Warn)

`multiple-versions = "deny"` is the preferred hardening baseline.

If the value is weaker, warn rather than error.
This is a deliberate design choice: it behaves like a threshold/floor, not like a must-match invariant.

### g3rs-deny/wildcards-inventory / g3rs-deny/license-allow-baseline — Wildcard policy

Hard requirement:
- `allow-wildcard-paths = true`

Inventory / warning:
- `wildcards` missing
- `wildcards` relying on defaults
- `wildcards` weaker than expected

Do not treat stricter user values as hard failures.

### g3rs-deny/confidence-threshold / g3rs-deny/copyleft-allowlist / g3rs-deny/unknown-sources-policy / RS-DENY-17 — License policy

Guardrail-owned:
- baseline allow list present
- `private.ignore = true`
- `confidence-threshold >= 0.8`

Warn / inventory:
- copyleft licenses in allow list
- license exceptions in `[licenses].exceptions`

### g3rs-deny/allow-git-inventory / g3rs-deny/tokio-full-ban / g3rs-deny/extra-feature-bans-inventory — Source policy

Guardrail-owned:
- `unknown-registry = "deny"`
- `unknown-git = "deny"`
- `allow-registry` must contain exactly one canonical crates.io entry

Canonical crates.io value:
- `sparse+https://index.crates.io/`

Any extra registry is an error.
The legacy GitHub crates.io index form is rejected to keep the policy exact.

`allow-git` is not automatically forbidden by the plan, but it is risky enough to warn and inventory.

### g3rs-deny/skip-hygiene / g3rs-deny/ignore-hygiene — Feature-ban policy

Guardrail-managed baseline:
- `tokio` must ban `full`
- `tokio` must keep the canonical explicit `allow = [...]` set from the generator baseline

Extra feature bans are inventoried.

### g3rs-deny/duplicate-entries / g3rs-deny/unknown-keys / RS-DENY-26 / g3rs-deny/extra-deny-bans-inventory — Exception hygiene

For `licenses.exceptions`, `skip`, and `ignore` entries:
- malformed entry shape errors
- bare string shortcut forms are forbidden because they cannot carry justification
- missing or non-string `reason` errors
- valid documented entries inventory

Ban entries without reasons are errors.

Excessive advisory ignores warn once the count exceeds `5`, but this is a pressure rule, not a core correctness rule.

### RS-DENY-25 — `[bans].allow` override channel

`[bans].allow` is a direct escape hatch because allow entries override deny entries.

Policy:
- non-empty `allow` should be treated as a problem by default
- any overlap with the deny baseline is an explicit error
- mixed-profile local roots must compare against the effective routed profile, not an implicit service fallback

### g3rs-deny/license-exceptions-inventory / g3rs-deny/allow-override-channel / RS-DENY-30 — Schema hardening

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
- malformed `guardrail3.toml` policy context for profile-sensitive deny rules
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
