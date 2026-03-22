# clippy.toml

## How clippy finds its config (empirically verified 2026-03-19)

When `cargo clippy` runs, cargo compiles each member crate separately. For each crate, cargo sets `CARGO_MANIFEST_DIR` to that crate's own directory (where its `Cargo.toml` is).

Clippy then looks for `clippy.toml` or `.clippy.toml` starting from `CARGO_MANIFEST_DIR` and walking UP through parent directories. It checks each directory. First file found wins. No merging — the first one found completely shadows anything higher up.

### Walk-up does NOT stop at workspace boundaries

Verified: if `apps/validator-rust/Cargo.toml` has `[workspace]` and there's NO `clippy.toml` at `apps/validator-rust/`, clippy walks up PAST it — through `apps/`, through project root, all the way to filesystem root (then `$HOME`, then `$XDG_CONFIG_HOME/clippy/`). The `[workspace]` boundary is irrelevant for clippy config resolution. Only the filesystem directory tree matters.

### Per-crate resolution is independent

Verified: with `cargo clippy --workspace` run from `apps/validator-rust/`, each member crate independently resolves its own config:
- Put `clippy.toml` with `too-many-lines-threshold = 1` at `crates/domain/` only
- `cargo clippy --workspace` → domain gets 61 errors (threshold 1), other 4 crates get 0 errors (they walk up past domain's dir to `apps/validator-rust/clippy.toml` with threshold 75)
- The per-crate file shadows the workspace file FOR THAT CRATE ONLY

### What covers what — verified on steady-parent

- `apps/validator-rust/clippy.toml` — each of the 5 member crates walks up and finds this. Covers all 5 crates.
- `apps/substack-publisher/clippy.toml` — substack-publisher walks up and finds this immediately (it's in its own dir). Covers 1 crate.
- `packages/low-expectations/` — no `clippy.toml` in its dir, none at `packages/`, none at project root. Walks all the way up. UNCOVERED (uses clippy defaults — no bans).
- `packages/seo-site-files/` — same. UNCOVERED.
- A `clippy.toml` at PROJECT ROOT would cover EVERYTHING — every workspace, every standalone crate. The walk-up goes past all workspace boundaries.

### Shadowing danger

A `clippy.toml` at ANY intermediate directory (e.g., `crates/adapters/clippy.toml`) would shadow the policy-root file for all crates below that directory. All guardrail bans silently lost for those crates. guardrail3 must ERROR if it finds clippy.toml files outside the allowed policy roots: validation root, workspace roots, or standalone package roots.

### Intermediate directory shadowing — verified

Tested: `clippy.toml` at `crates/adapters/` (between workspace root and `crates/adapters/outbound/` and `crates/adapters/inbound/api/`):
- `adapters` crate: SHADOWED (57 errors with threshold 1). Walk-up: `crates/adapters/outbound/` → `crates/adapters/` → found.
- `api` crate: SHADOWED (57 errors). Walk-up: `crates/adapters/inbound/api/` → `crates/adapters/inbound/` → `crates/adapters/` → found.
- `domain` crate: NOT shadowed (0 errors). Walk-up: `crates/domain/` → `crates/` → `apps/validator-rust/` → found workspace-level file. Domain's path never goes through `crates/adapters/`.
- `app` crate: NOT shadowed. Same — sibling path.

Tested: `clippy.toml` at `crates/` (one level above all member crates):
- ALL crates shadowed (62 errors each after `cargo clean`). Every member walks through `crates/`.

### Caching gotcha — verified

Clippy results are CACHED. Changing or adding a `clippy.toml` does NOT automatically recompile affected crates. A stale cache can hide the effect of a rogue config file. Only `cargo clean` or touching source files forces re-checking. This means a developer can add a per-crate `clippy.toml` and not see its effect (positive or negative) until a clean build.

### Recommendation

Treat `clippy.toml` as a Rust policy-root file.

Allowed locations:
- validation root / repo root
- Rust workspace roots
- standalone package roots that are NOT members of a workspace

Forbidden locations:
- any deeper member-crate path below one of those roots
- intermediate shadow configs that are not themselves workspace roots or standalone package roots

Coverage rule:
- every Rust workspace root and every standalone package root must be covered by an allowed `clippy.toml`
- coverage can come from:
  - its own local `clippy.toml`, or
  - an allowed ancestor `clippy.toml` (for example the validation root)
- uncovered Rust units are an ERROR, not a warning

## Why not per inner crate

Per-crate `clippy.toml` SHADOWS the workspace one completely. If a crate has its own `clippy.toml`, it loses ALL workspace bans.

Could we generate per-crate files with duplication? Yes, but there's no enforcement benefit:

1. **std library bans (HashMap, std::fs, std::env, etc.):** Apply to ALL crates equally. No crate needs different std bans than another. One workspace file covers it.

2. **Workspace dep bans (reqwest::Client::new, serde_json::from_str, etc.):** Clippy bans on non-std paths only fire if the crate has the dependency in its Cargo.toml. If `reqwest::Client::new` is banned workspace-wide, domain is unaffected (reqwest not in its deps — the ban is silently ignored). Adapters has reqwest, so the ban fires there. No per-crate difference needed.

3. **"But what if adapters needs from_str but domain shouldn't?":** Both have serde_json. The garde-deserialization ban covers `serde_json::from_str` workspace-wide. The garde wrapper function uses `#[allow(clippy::disallowed_methods)] // reason: validated deserialization` — per-function escape hatch with documented reason. This is the intended model: ban by default, allow with reason.

4. **Cargo.toml deps handle real isolation:** Domain literally can't call reqwest because it's not a dependency. R-DEPS/R-ARCH validate that dependency declarations are correct. This is compile-time enforcement, stronger than clippy bans.

So: policy-root `clippy.toml` + Cargo.toml dependency isolation + per-function `#[allow]` with reason covers every scenario. Inner member-crate `clippy.toml` adds shadowing risk with no enforcement benefit.

## Contents

clippy.toml has many possible configuration keys (thresholds, lists, booleans, enums). guardrail3 should manage the hardening-critical subset that can be applied universally and sanely. Other keys remain user-owned and must be preserved.

**guardrail3-managed keys**
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
- arrays:
  - `disallowed-methods`
  - `disallowed-types`
  - `disallowed-macros`

**User-owned keys we must NOT touch (examples from real projects):**
- `msrv` — project's minimum Rust version
- `allowed-duplicate-crates` — crates with unavoidable duplicates (e.g., `windows-sys`)
- `arithmetic-side-effects-allowed` — custom numeric types exempt from overflow checks
- `allow-expect-in-tests`, `allow-unwrap-in-tests` — user-specific test relaxations not currently managed
- `doc-valid-idents` — project-specific API terms (e.g., `["OAuth", "MyAPI"]`)
- `await-holding-invalid-types` — async safety guards
- Any other key from the ~70 we don't manage

**Rule: guardrail3 manages the keys above. All other recognized user keys are passed through untouched. Truly unknown / typo-looking keys should warn in validate.**

### Thresholds

| Key | clippy default | guardrail3 value | stricter? |
|---|---|---|---|
| too-many-lines-threshold | 100 | 75 | YES — 25% tighter |
| cognitive-complexity-threshold | 25 | 15 | YES — 40% tighter |
| too-many-arguments-threshold | 7 | 7 | same |
| type-complexity-threshold | 250 | 75 | YES — 70% tighter |
| max-struct-bools | 3 | 3 | same |
| max-fn-params-bools | n/a here | 3 | managed by guardrail |
| excessive-nesting-threshold | n/a here | 4 | managed by guardrail |

Enforcement:
- NEW file: set guardrail3 values
- EXISTING file, key missing: ADD with guardrail3 value
- validate should enforce exact-match for guardrail-managed thresholds
- generate should write the exact guardrail value
- if users relax later, validate errors or warns according to the checker rule contract

### disallowed-methods

**Paths are the enforcement. Reasons are informational.**

Guardrail3 baseline methods (service profile): env-vars (3), env-mutation (2), process-control (2), blocking-sleep (1), filesystem (15 — bans direct std::fs usage, enforces centralized fs module pattern), http-client (2 — bans direct client construction, enforces shared clients), garde-deserialization. Total ~25.

NOTE: filesystem bans don't forbid filesystem ACCESS — they force it through a centralized module (e.g., `crate::fs::*`). The app CAN read/write files, just through one controlled module. For packages/libraries, filesystem is fully forbidden (libraries should be pure).

Steady-parent reality:
- Both apps ban the same 21 paths (env, process, sleep, fs, Command)
- validator-rust adds: `reqwest::Client::builder`, `reqwest::Client::new` (per-app override)
- substack-publisher adds: `std::io::stdout`, `std::io::stderr` (per-app override)
- Reasons differ on every shared entry (project-specific context)

**What we enforce: the PATH must be present, and every guardrail-managed ban entry must have a real reason.**
- If `std::fs::write` is banned with reason "All state lives in R2" — that's fine. The ban exists and the reason is meaningful.
- If a guardrail baseline path is MISSING — add on generate, error on validate.
- If user has EXTRA paths beyond baseline — preserve them.

**Service profile baseline paths (27 methods):**
env-vars: `std::env::var`, `std::env::var_os`, `std::env::vars`
env-mutation: `std::env::set_var`, `std::env::remove_var`
process: `std::process::exit`, `std::process::Command::new`
sleep: `std::thread::sleep`
filesystem (15): `std::fs::read_to_string`, `read`, `read_dir`, `read_link`, `write`, `remove_file`, `remove_dir_all`, `create_dir_all`, `rename`, `copy`, `metadata`, `symlink_metadata`, `canonicalize`, `set_permissions`, `hard_link`
http-client: `reqwest::Client::new`, `reqwest::Client::builder`
garde-deserialization (8): `serde_json::from_str`, `from_slice`, `from_value`, `from_reader`, `reqwest::Response::json`, `toml::from_str`, `serde_yaml::from_str`, `serde_yaml::from_reader`

**Service profile baseline paths (7 types, or 11 with pure layer):**
collections: `std::collections::HashMap`, `std::collections::HashSet`
sync: `std::sync::Mutex`, `std::sync::RwLock`
filesystem: `std::fs::File`
garde-extractors: `axum::extract::Json`, `axum::Json`, `axum::extract::Query`, `axum::extract::Form`
pure-layer-only (4): `std::sync::LazyLock`, `std::sync::OnceLock`, `once_cell::sync::Lazy`, `once_cell::sync::OnceCell`

**Library profile:** same methods + ALL types including global-state (all crates are pure)

**Garde module is conditional:** only included when `[rust.apps.X.checks] garde = true`.
When `garde = false`: EXCLUDE `METHOD_GARDE_DESERIALIZATION` (8 paths: serde_json::from_str/slice/value/reader, reqwest::Response::json, toml::from_str, serde_yaml::from_str/from_reader) and `TYPE_GARDE_EXTRACTORS` (4 paths: axum::extract::Json, axum::Json, axum::extract::Query, axum::extract::Form) from baseline.
Implementation: `build_clippy_toml` needs a `garde_enabled: bool` parameter. Thread from config: `crate_cfg.checks.as_ref().and_then(|c| c.garde).unwrap_or(true)`.

### disallowed-types

Same pattern. 9 types identical in both apps:
- HashMap, HashSet (use BTreeMap/BTreeSet)
- Mutex, RwLock (use parking_lot or tokio)
- File (use centralized io)
- LazyLock, OnceLock, once_cell::OnceCell, once_cell::Lazy (no global state)

Reasons differ. Same rule: enforce PATH presence and require a real reason field.

### disallowed-macros

This is guardrail-managed too.

Required baseline macro bans:
- `println!`
- `eprintln!`
- `dbg!`
- `todo!`
- `unimplemented!`

Same rule: enforce macro presence and require a real reason field.

## Algorithm

### On `generate` (for an existing file):

```
1. Parse existing clippy.toml with toml_edit (preserves comments, formatting)
2. Load removals: read per-app + global remove files, collect paths to skip
3. For thresholds:
   - For each guardrail threshold:
     - If missing: ADD with guardrail value
     - If present and value differs: REWRITE to the guardrail value
     - Guardrail-managed thresholds are exact-match, not lower-bound / upper-bound suggestions
4. For disallowed-methods array:
   - Parse each entry, key by `path` field
   - For each guardrail baseline path:
     - If path is in removals: SKIP only when that removal is an explicitly allowed override with a real reason
     - If path missing from array: ADD entry with guardrail reason
     - If path present: LEAVE (don't touch reason — user's is more specific)
   - Everything else in the array: LEAVE (user's own bans, not our business)
5. For disallowed-types array:
   - Same as methods
6. Write back with toml_edit (preserves comments, ordering, user entries)
```

### On `generate` (for a new file):
```
1. Build from template: thresholds + managed booleans + profile methods + profile types + macro bans + overrides
2. Write complete file (current behavior, correct)
```

### On `generate --dry-run`:
```
For existing file:
- Show which baseline entries are MISSING
- Show which managed scalar settings are MISSING or wrong
- Show user's extra entries (informational, not errors)
- Show "no changes needed" if fully compliant

For new file:
- Show "would create" with profile info
```

## Override mechanism

With merge-managed, the file IS the source of truth. User adds extra bans directly to clippy.toml — guardrail3 won't touch them (they're not baseline paths). No need for addition override files.

**The only override needed: removals.**

`.guardrail3/overrides/apps/{name}/clippy-methods-remove.toml` — baseline paths to SKIP for this app.
`.guardrail3/overrides/apps/{name}/clippy-types-remove.toml` — baseline types to SKIP for this app.

Example: substack-publisher needs `std::collections::HashMap` for performance.
```toml
# .guardrail3/overrides/apps/substack-publisher/clippy-types-remove.toml
{ path = "std::collections::HashMap", reason = "Measured: BTreeMap 3x slower for our lookup pattern" }
```

Generate skips adding HashMap ban for substack-publisher. Validate warns "guardrail relaxed: std::collections::HashMap ban removed for substack-publisher — reason: Measured: BTreeMap 3x slower for our lookup pattern"

Global removals (`.guardrail3/overrides/clippy-methods-remove.toml`) also possible but should be rare — if you're removing a baseline ban for ALL apps, maybe the baseline is wrong.

## Edge cases

1. **Inner crate has clippy.toml (R-CLIP-SHADOW — new check):** ERROR. It shadows the policy-root config completely. ALL guardrail bans silently lost for that subtree. validate must allow `clippy.toml` only at:
   - validation root
   - workspace roots
   - standalone package roots not belonging to a workspace
   The fix: remove the nested file and move custom entries to the nearest allowed policy root (or use per-function `#[allow]` with reason).

2. **Root packages with no clippy.toml:** Generate one at root with library profile (packages are libraries). If `[rust.packages]` is in guardrail3.toml, generate.

3. **Standalone crate not in guardrail3.toml:** it still counts as a valid package-level policy root if it is a package root not belonging to a workspace. Clippy placement/coverage must be derived from Rust project structure, not hex-arch folder naming.

4. **User's file has no guardrail3 header comment:** First merge adds the entries but does NOT add the "GENERATED by guardrail3" header. The file remains user-owned but guardrail-compliant.

5. **User removes a baseline ban:** Next `generate` adds it back. This is intentional — the baseline is enforced. To legitimately skip a ban, use the `-remove.toml` override.

## Parser

`toml_edit` crate. It preserves:
- Comments (inline and standalone)
- Formatting (whitespace, alignment)
- Key ordering

For the `disallowed-methods` and `disallowed-types` arrays: these can be EITHER:
- Inline table syntax: `disallowed-methods = [{ path = "...", reason = "..." }]`
- Table array syntax: `[[disallowed-methods]]\npath = "..."\nreason = "..."`

toml_edit distinguishes these (`Array` vs `ArrayOfTables`). The merge algorithm must:
- Detect which format the existing file uses
- Append new entries in the SAME format
- Never mix formats (appending inline to table-array or vice versa)

Clippy entries support 4 fields: `path` (required), `reason` (optional), `replacement` (optional — suggests fix in lint output), `allow-invalid` (optional — suppresses "path not found" warning).

Key operations:
- Index existing entries by `path` value to detect presence/absence
- When an entry exists: LEAVE the ENTIRE entry (all fields including replacement/allow-invalid)
- When adding new entry: use `{ path, reason }` (guardrail3 doesn't set replacement/allow-invalid)

## Dependencies

- `toml_edit` crate (add to Cargo.toml)
- Existing `generate_helpers.rs` override loading
- New: per-app override directory resolution
