# Shared Escape-Hatch Policy

**Date:** 2026-03-30
**Status:** target-state plan
**Scope:** `RS-CODE`, `RS-TEST`, `RS-DENY`, `RS-CLIPPY`, `RS-CARGO`, `RS-FMT`, `RS-HEXARCH`, `RS-GARDE`, shared reason policy crate, shared policy-context parsing

## Goal

Every escape hatch in guardrail3 should behave the same way:

- it is always detected explicitly
- it always requires a justification
- justification quality is validated by one shared policy implementation
- documented escape hatches stay visible in normal output
- every family reports how many escape hatches it has

Inventory is reserved for clean-state proof and passive audit surfaces. It is not the right home for suspicious-but-documented bypasses.

## What Counts As An Escape Hatch

An escape hatch is any local mechanism that weakens, suppresses, skips, or overrides a guardrail-owned contract.

Current escape-hatch surfaces:

- `RS-CODE`
  - item-level `#[allow(...)]`
  - item-level `#[expect(...)]`
  - `#[garde(skip)]`
  - path/include/bypass comment channels already modeled by code-family rules
- `RS-TEST`
  - `#[ignore]`
- `RS-DENY`
  - `[[licenses.exceptions]]`
  - `[bans.skip]`
  - `[advisories].ignore`
  - `[bans].allow`
  - `[sources].allow-git`
- `RS-CLIPPY`
  - reason-bearing entries in `disallowed-methods`
  - reason-bearing entries in `disallowed-types`
  - reason-bearing entries in `disallowed-macros`
  - test-relaxation booleans such as `allow-panic-in-tests`
- `RS-CARGO`
  - manifest lint `allow` entries
- `RS-FMT`
  - `ignore = [...]` in `rustfmt.toml`
- `RS-HEXARCH`
  - `[patch.*]` and `[replace]` path overrides
- `RS-GARDE`
  - review-only bypass inventory such as `sqlx::query_as!`

## Global Policy

Every escape hatch must fall into one of two categories:

1. **Documentable and tolerated**
   - no reason: `Error`
   - weak reason: `Error`
   - valid reason: `Warn`
   - valid reason remains visible in normal output

2. **Documentable but still forbidden**
   - no reason: `Error`
   - weak reason: `Error`
   - valid reason: still `Error`
   - reason is required so the policy debt is explicit, but the hatch is not approved

Examples of likely tolerated hatches:

- local `#[ignore]`
- deny `licenses.exceptions`
- deny `bans.skip`
- deny `advisories.ignore`
- code-family local allow/expect
- code-family documented `garde(skip)`

Examples of likely forbidden hatches:

- deny `[bans].allow`
- cargo member-local allow under `[lints] workspace = true`
- in-tree hexarch `[patch]` / `[replace]`

The family decides whether a given hatch is tolerated or forbidden. The shared policy decides whether a reason is meaningful.

## Shared Reason Policy Crate

The existing shared crate lives at:

- `packages/reason-policy/crates/reason-policy`

This crate should become the only place that validates reason quality.

Target API:

```rust
pub const DEFAULT_MIN_REASON_CHARS: usize = 12;
pub const DEFAULT_MIN_REASON_WORDS: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasonIssue {
    Empty,
    TooShort {
        min_chars: usize,
        actual_chars: usize,
    },
    TooFewWords {
        min_words: usize,
        actual_words: usize,
    },
    Placeholder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReasonPolicy {
    pub min_chars: usize,
    pub min_words: usize,
}

impl Default for ReasonPolicy {
    fn default() -> Self {
        Self {
            min_chars: DEFAULT_MIN_REASON_CHARS,
            min_words: DEFAULT_MIN_REASON_WORDS,
        }
    }
}

pub fn validate_reason_text(reason: &str) -> Result<(), ReasonIssue> {
    validate_reason_text_with_policy(reason, ReasonPolicy::default())
}

pub fn validate_reason_text_with_policy(
    reason: &str,
    policy: ReasonPolicy,
) -> Result<(), ReasonIssue>;

pub fn reason_text_is_useful(reason: &str) -> bool {
    validate_reason_text(reason).is_ok()
}
```

Required behavior:

- trim leading/trailing whitespace
- reject empty text
- count words by whitespace-split non-empty tokens
- reject under `2` words
- reject under `12` characters after trim
- reject obvious placeholders

Required placeholder set:

```rust
const PLACEHOLDERS: &[&str] = &[
    "temp",
    "temporary",
    "todo",
    "tbd",
    "fixme",
    "fix later",
    "legacy",
    "reason",
    "...",
];
```

No family-local placeholder list is allowed after this migration.

## Shared Reason Carriers

There are only three accepted ways to carry a reason.

### 1. Source-attached reason

Examples:

- `#[ignore = "..."]`
- `// reason: ...`
- same-line `// reason: ...`
- previous-line `// reason: ...`

Source families own:

- finding the escape hatch
- extracting the candidate reason text

Source families must not own:

- deciding whether the extracted text is meaningful

### 2. Native config-field reason

Examples:

- TOML `reason = "..."`

Config families own:

- validating entry shape
- extracting the string value

Config families must delegate quality validation to the shared crate.

### 3. Shared sidecar registry reason

Some escape hatches have no native place to store a reason. Those must use one central policy-owned registry in `guardrail3.toml`.

Target schema:

```toml
[[escape_hatches]]
family = "cargo"
file = "apps/foo/Cargo.toml"
kind = "lint_allow"
selector = "workspace.lints.clippy.module_name_repetitions"
reason = "Macro-generated names are temporary until parser cleanup lands."

[[escape_hatches]]
family = "fmt"
file = "rustfmt.toml"
kind = "ignore"
selector = "generated/**"
reason = "Generated code is rewritten upstream and rustfmt changes break snapshots."

[[escape_hatches]]
family = "hexarch"
file = "apps/foo/Cargo.toml"
kind = "patch_replace"
selector = "backend-domain-types"
reason = "Temporary local patch while upstream crate split is in flight."
```

Matching key:

- `family`
- `file`
- `kind`
- `selector`

No fuzzy lookup, no prefix lookup, no inferred matching.

If a family uses the sidecar registry, missing match is treated exactly like missing native reason.

## Shared Escape-Hatch Reporting Model

Each escape hatch produces one primary finding.

Target severity matrix:

| State | Severity | Visible by default | Inventory |
|---|---|---|---|
| Missing reason | Error | Yes | No |
| Weak reason | Error | Yes | No |
| Valid reason, tolerated hatch | Warn | Yes | No |
| Valid reason, forbidden hatch | Error | Yes | No |

There is no hidden-inventory documented-bypass lane.

## Shared Count Reporting

Each family that owns escape hatches must also emit one count finding per owner/config/root when count > 0.

Target message shape:

```text
`apps/foo/Cargo.toml` has 4 cargo escape hatches
(1 undocumented, 1 weak-reason, 2 documented)
```

Rules for count reporting:

- visible by default
- `Warn`
- not inventory
- count all active escape hatches, including forbidden ones
- emitted once per owned container, not once per escape hatch

Families may additionally keep clean-state inventory proofs when count is zero.

## Shared Helper Types

Families should normalize escape hatches before reporting.

Recommended helper type, implemented family-local or in a later shared crate:

```rust
pub struct EscapeHatchFindingInput<'a> {
    pub family: &'static str,
    pub kind: &'static str,
    pub selector: String,
    pub file: &'a str,
    pub line: Option<usize>,
    pub native_reason: Option<String>,
    pub sidecar_reason: Option<String>,
    pub tolerated_when_documented: bool,
}
```

Decision order:

1. determine effective reason source
2. validate quality with shared crate
3. apply family policy for tolerated vs forbidden
4. emit finding
5. accumulate count stats

## Family-Specific Target State

### `RS-CODE`

Status:

- already closest to target state
- already uses shared reason validation
- already keeps documented bypasses visible

Required final state:

- stay on shared validator only
- keep undocumented and weak reasons as `Error`
- keep documented tolerated bypasses as visible `Warn`
- add family/root-level escape-hatch count summary if not already present

Relevant files:

- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lint_policy/rs_code_03_item_level_allow_without_reason.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lint_policy/rs_code_04_item_level_allow_with_reason.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/hygiene/rs_code_05_garde_skip_without_comment.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/hygiene/rs_code_06_garde_skip_with_comment.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lint_policy/rs_code_22_deny_forbid_without_reason.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/cfg_and_paths/rs_code_24_path_attr.rs`

### `RS-TEST`

Escape hatch:

- `#[ignore]`

Current gap:

- accepts presence-only reason carriers
- documented case is treated as hidden clean-path inventory, not visible warning

Target:

- bare `#[ignore]`: `Error`
- weak `#[ignore = "..."]` or weak `// reason:`: `Error`
- valid documented `#[ignore]`: visible `Warn`
- per-file/per-root count warning for ignored tests

Required code changes:

- extend AST helper extraction to return actual reason text, not only “has reason”
- make `#[ignore = "..."]` and comment paths flow into shared reason validator

Relevant files:

- `apps/guardrail3/crates/app/rs/ast/src/extra_visitors.rs`
- `apps/guardrail3/crates/app/rs/ast/src/ast_helpers.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/assertion_quality/rs_test_04_ignore_reason.rs`

Target helper contract:

```rust
pub enum IgnoreReasonState {
    Missing,
    Present(String),
}

pub fn find_ignored_tests_with_reason_state(...) -> Vec<(usize, IgnoreReasonState)>;
```

### `RS-DENY`

Escape hatches:

- `[[licenses.exceptions]]`
- `[bans.skip]`
- `[advisories].ignore`
- `[bans].allow`
- `[sources].allow-git`

Current gaps:

- reason quality is presence-only for `17`, `23`, `24`, `26`
- documented tolerated exceptions are hidden inventory
- `allow-git` and `bans.allow` have no reason path

Target:

- `RS-DENY-17`, `23`, `24`, `26` use shared validator
- valid documented `licenses.exceptions`, `skip`, and `ignore` become visible `Warn`
- missing or weak reasons remain `Error`
- `RS-DENY-CONFIG-17 allow-git` uses sidecar registry reason matching
- `RS-DENY-25 bans.allow` uses sidecar registry reason matching, but remains `Error` even when documented if policy stays “forbidden hatch”
- add deny-family escape-hatch count summary

Relevant files:

- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/licenses/rs_deny_17_license_exceptions_inventory.rs`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/sources/rs_deny_config_17_allow_git_inventory.rs`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/sources/rs_deny_config_20_skip_hygiene.rs`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/sources/rs_deny_config_21_ignore_hygiene.rs`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/sources/rs_deny_25_allow_override_channel.rs`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/bans/rs_deny_26_ban_reason_inventory.rs`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/sources/rs_deny_config_26_ignore_accumulation.rs`

### `RS-CLIPPY`

Escape hatches:

- reason-bearing ban entries in `disallowed-methods`
- reason-bearing ban entries in `disallowed-types`
- reason-bearing ban entries in `disallowed-macros`
- test-relaxation booleans

Current gaps:

- reason quality is implemented locally
- missing/trivial reasons are only `Warn`
- documented entries collapse into inventory clean-state results
- test relaxations are visible findings but have no documentation mechanism

Target:

- replace local reason policy with shared validator
- `RS-CLIPPY-08` and `RS-CLIPPY-15` should collapse into one shared-quality interpretation:
  - missing reason: `Error`
  - weak reason: `Error`
  - valid reason: visible `Warn`
- test-relaxation booleans should use sidecar registry reasoning if they remain supported as escape hatches
- add clippy-family escape-hatch count summary

Relevant files:

- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_08_reason_quality.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_15_trivial_reason.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_config_15_test_relaxations.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/clippy_support.rs`

### `RS-CARGO`

Escape hatches:

- workspace/package lint `allow` entries
- member-local `allow` entries on inherited lint policy

Current gap:

- no native reason channel exists in Cargo lint tables

Target:

- all approved root-level `allow` entries require matching sidecar registry reasons
- missing/weak sidecar reason: `Error`
- valid documented root-level `allow`: visible `Warn`
- member-local `allow` under `[lints] workspace = true` stays `Error`, but may also require a sidecar reason so the debt is explicit
- add cargo-family escape-hatch count summary

Relevant files:

- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/workspace_policy/rs_cargo_03_allow_inventory.rs`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/workspace_policy/rs_cargo_12_unapproved_allow_entries.rs`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/member_policy/rs_cargo_13_member_local_allows_forbidden.rs`

### `RS-FMT`

Escape hatch:

- `ignore = [...]`

Current gap:

- no native reason field

Target:

- every ignored pattern requires matching sidecar registry reason
- missing/weak reason: `Error`
- valid documented ignore: visible `Warn`
- add fmt-family escape-hatch count summary

Relevant file:

- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_07_ignore_escape_hatch.rs`

### `RS-HEXARCH`

Escape hatch:

- `[patch.*]` / `[replace]`

Current gap:

- no native reason field

Target:

- each in-scope patch/replace entry requires matching sidecar registry reason
- missing/weak reason: `Error`
- valid reason still `Error` if in-tree bypass stays categorically forbidden
- add hexarch-family escape-hatch count summary

Relevant file:

- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_policy/rs_hexarch_16_patch_replace_bypass.rs`

### `RS-GARDE`

Escape hatch surface:

- review-only bypass inventory such as `sqlx::query_as!`

Target:

- if this remains a review-only hatch, convert it to the shared model:
  - sidecar registry reason required
  - missing/weak reason: `Error`
  - valid documented hatch: visible `Warn`
  - count summary

Relevant file:

- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/inventory/rs_garde_ast_04_query_as_inventory.rs`

## Shared `guardrail3.toml` Parsing Changes

The sidecar registry is cross-family. Parsing should not be reimplemented per family.

Recommended home:

- shared policy parsing layer close to existing `guardrail3.toml` handling

Minimum required parsed type:

```rust
pub struct EscapeHatchRegistryEntry {
    pub family: String,
    pub file: String,
    pub kind: String,
    pub selector: String,
    pub reason: String,
}
```

Minimum required validation:

- root key must be `[[escape_hatches]]`
- `family`, `file`, `kind`, `selector`, `reason` must all be strings
- invalid schema must fail closed for families that depend on the registry

If a family depends on sidecar reasons and active `guardrail3.toml` is malformed, that family must not silently stand down.

## Test Contract

Every migrated rule must gain attack tests for:

- missing reason
- empty reason
- one-word reason
- too-short reason
- placeholder reason
- valid multi-word reason
- documented visible warning exactness
- count summary exactness
- malformed shared sidecar registry when sidecar lookup is required

For sidecar-registry families, tests must also cover:

- correct exact match
- wrong family
- wrong file
- wrong kind
- wrong selector
- duplicate registry entries

## Documentation Contract

When this policy lands:

- family READMEs must stop describing documented escape hatches as inventory
- family rule ledgers must use the same severity model
- any family-specific placeholder heuristic docs must be removed
- code comments and plans should say “shared reason policy” rather than “good enough reason text”

## Build Order

This is the order that minimizes churn and duplicated work:

1. finish the shared reason-policy API
2. migrate `RS-TEST-04`
3. migrate `RS-DENY-17`, `23`, `24`, `26`
4. migrate `RS-CLIPPY-08`, `15`
5. add shared `guardrail3.toml` escape-hatch registry parsing
6. migrate `RS-CARGO`, `RS-FMT`, `RS-HEXARCH`, and any remaining non-native-reason escape hatches onto sidecar registry reasons
7. add per-family count rules
8. align docs and rule ledgers

## Success Condition

The target state is complete only when all of the following are true:

- every escape hatch is explicit
- every escape hatch has a justification path
- every justification is validated by one shared implementation
- documented escape hatches stay visible in normal output
- every family reports escape-hatch counts
- no family keeps a private reason-quality heuristic
- no family hides documented bypasses as inventory-only output
