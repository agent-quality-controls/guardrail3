# clippy.toml

## Location

**Where clippy looks:** Walks UP from `CARGO_MANIFEST_DIR` (the directory of the crate being compiled). First `clippy.toml` or `.clippy.toml` found wins. No merging — nearest shadows completely.

**In steady-parent:**
- `apps/validator-rust/clippy.toml` — covers all 5 crates in the workspace (domain, ports/outbound, app, adapters/outbound, adapters/inbound/api). None of the inner crates have their own clippy.toml.
- `apps/substack-publisher/clippy.toml` — covers the single crate.
- NO root clippy.toml — `packages/low-expectations` and `packages/seo-site-files` have NO clippy.toml covering them at all.

**Valid locations for guardrail3:**
1. Per-app workspace root (e.g., `apps/validator-rust/clippy.toml`) — covers all crates in that workspace
2. Per standalone crate (e.g., `apps/substack-publisher/clippy.toml`)
3. Root workspace (for root-level packages with no app workspace)
4. Per inner crate — technically possible but clippy doesn't support per-crate overrides within a workspace (issue #7353). A per-crate clippy.toml SHADOWS the workspace one entirely. Dangerous — avoid.

**Rule: one clippy.toml per Rust workspace/standalone crate. WARN if per-crate clippy.toml files exist (they shadow the workspace one and could silently drop all guardrail bans).**

**Why not per inner crate — fully analyzed:**

Per-crate clippy.toml SHADOWS the workspace one completely (clippy has no merging). If a crate has its own clippy.toml, it loses ALL workspace bans. This is the primary danger.

Could we generate per-crate files with duplication? Yes, but there's no enforcement benefit:

1. **std library bans (HashMap, std::fs, std::env, etc.):** Apply to ALL crates equally. No crate needs different std bans than another. One workspace file covers it.

2. **Workspace dep bans (reqwest::Client::new, serde_json::from_str, etc.):** Clippy bans on non-std paths only fire if the crate has the dependency in its Cargo.toml. If `reqwest::Client::new` is banned workspace-wide, domain is unaffected (reqwest not in its deps — the ban is silently ignored). Adapters has reqwest, so the ban fires there. No per-crate difference needed.

3. **"But what if adapters needs from_str but domain shouldn't?":** Both have serde_json. The garde-deserialization ban covers `serde_json::from_str` workspace-wide. The garde wrapper function uses `#[allow(clippy::disallowed_methods)] // reason: validated deserialization` — per-function escape hatch with documented reason. This is the intended model: ban by default, allow with reason.

4. **Cargo.toml deps handle real isolation:** Domain literally can't call reqwest because it's not a dependency. R-DEPS/R-ARCH validate that dependency declarations are correct. This is compile-time enforcement, stronger than clippy bans.

So: workspace-level clippy.toml + Cargo.toml dependency isolation + per-function `#[allow]` with reason covers every scenario. Per-crate would add duplication with zero enforcement benefit.

## Contents

clippy.toml has ~80 possible configuration keys (thresholds, lists, booleans, enums). guardrail3 manages 7 of them. The rest are user-owned and MUST be preserved.

**guardrail3-managed keys (7):**
- 5 thresholds: `too-many-lines-threshold`, `cognitive-complexity-threshold`, `too-many-arguments-threshold`, `type-complexity-threshold`, `max-struct-bools`
- 2 arrays: `disallowed-methods`, `disallowed-types`

**User-owned keys we must NOT touch (examples from real projects):**
- `msrv` — project's minimum Rust version
- `allowed-duplicate-crates` — crates with unavoidable duplicates (e.g., `windows-sys`)
- `arithmetic-side-effects-allowed` — custom numeric types exempt from overflow checks
- `allow-expect-in-tests`, `allow-unwrap-in-tests`, `allow-dbg-in-tests` — test relaxations
- `doc-valid-idents` — project-specific API terms (e.g., `["OAuth", "MyAPI"]`)
- `disallowed-macros` — project-specific macro bans
- `await-holding-invalid-types` — async safety guards
- Any other key from the ~70 we don't manage

**Rule: guardrail3 ONLY reads/writes its 7 keys. All other keys are passed through untouched.**

### Thresholds (5 values)

| Key | clippy default | guardrail3 value | stricter? |
|---|---|---|---|
| too-many-lines-threshold | 100 | 75 | YES — 25% tighter |
| cognitive-complexity-threshold | 25 | 15 | YES — 40% tighter |
| too-many-arguments-threshold | 7 | 7 | same |
| type-complexity-threshold | 250 | 75 | YES — 70% tighter |
| max-struct-bools | 3 | 3 | same |

Enforcement:
- NEW file: set guardrail3 values
- EXISTING file, key missing: ADD with guardrail3 value
- EXISTING file, value stricter (lower): LEAVE
- EXISTING file, value looser (higher): LEAVE but validate WARNS ("guardrail relaxed: too-many-lines-threshold is 200, baseline is 75")
- Same model as `#[allow]` — user can relax, we always report

### disallowed-methods

**Paths are the enforcement. Reasons are informational.**

Guardrail3 baseline methods (service profile): env-vars (3), env-mutation (2), process-control (2), blocking-sleep (1), filesystem (15 — bans direct std::fs usage, enforces centralized fs module pattern), http-client (2 — bans direct client construction, enforces shared clients), garde-deserialization. Total ~25.

NOTE: filesystem bans don't forbid filesystem ACCESS — they force it through a centralized module (e.g., `crate::fs::*`). The app CAN read/write files, just through one controlled module. For packages/libraries, filesystem is fully forbidden (libraries should be pure).

Steady-parent reality:
- Both apps ban the same 21 paths (env, process, sleep, fs, Command)
- validator-rust adds: `reqwest::Client::builder`, `reqwest::Client::new` (per-app override)
- substack-publisher adds: `std::io::stdout`, `std::io::stderr` (per-app override)
- Reasons differ on every shared entry (project-specific context)

**What we enforce: the PATH must be present. We do NOT enforce reasons.**
- If `std::fs::write` is banned with reason "All state lives in R2" — that's fine. The ban exists.
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

Reasons differ. Same rule: enforce PATH presence, don't enforce reasons.

## Algorithm

### On `generate` (for an existing file):

```
1. Parse existing clippy.toml with toml_edit (preserves comments, formatting)
2. Load removals: read per-app + global remove files, collect paths to skip
3. For thresholds:
   - For each guardrail threshold:
     - If missing: ADD with guardrail value
     - If present and value <= guardrail value (stricter): LEAVE
     - If present and value > guardrail value (looser): LEAVE (validate warns separately)
4. For disallowed-methods array:
   - Parse each entry, key by `path` field
   - For each guardrail baseline path:
     - If path is in removals: SKIP (validate warns "guardrail relaxed: {path} — {reason}")
     - If path missing from array: ADD entry with guardrail reason
     - If path present: LEAVE (don't touch reason — user's is more specific)
   - Everything else in the array: LEAVE (user's own bans, not our business)
5. For disallowed-types array:
   - Same as methods
6. Write back with toml_edit (preserves comments, ordering, user entries)
```

### On `generate` (for a new file):
```
1. Build from template: thresholds + profile methods + profile types + overrides
2. Write complete file (current behavior, correct)
```

### On `generate --dry-run`:
```
For existing file:
- Show which baseline entries are MISSING
- Show which thresholds are LOOSER than guardrail
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

1. **Inner crate has clippy.toml (R-CLIP-SHADOW — new check):** ERROR. It shadows the workspace one completely. ALL guardrail bans silently lost for that crate. validate must scan for `clippy.toml` or `.clippy.toml` in every crate directory below the workspace root. Implementation: iterate workspace members, check each member dir for clippy.toml presence (excluding the workspace root itself). The fix: remove the per-crate file and move any custom entries to the workspace-level file (or per-function `#[allow]` with reason).

2. **Root packages with no clippy.toml:** Generate one at root with library profile (packages are libraries). If `[rust.packages]` is in guardrail3.toml, generate.

3. **Standalone crate not in guardrail3.toml (substack-publisher):** `rs init` must discover it. Currently doesn't because `discover_nested_workspaces` only finds `[workspace]` Cargo.tomls. Fix: also check for `[package]` Cargo.tomls in `apps/*/`.

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
