# deny.toml

## Location

**Where cargo-deny looks:** `<cwd>/deny.toml` only. No walk-up. No directory search. You must run `cargo deny` from the directory containing deny.toml, or pass `--config <path>`. There is also an exceptions file: `deny.exceptions.toml` / `.deny.exceptions.toml` / `.cargo/deny.exceptions.toml` — searched in the same directory.

**Per-folder/per-crate deny.toml:** NOT SUPPORTED. Issue #20 (open since 2019) requests this. cargo-deny operates on ONE config for the entire workspace. To have different rules per app, you must run cargo-deny separately per workspace with `--manifest-path`.

**In steady-parent:**
- `apps/validator-rust/deny.toml` (109 lines) — full config for the validator-rust workspace
- `apps/substack-publisher/deny.toml` (68 lines) — full config for the standalone crate
- NO root `deny.toml` — root workspace (packages/low-expectations, packages/seo-site-files) has no deny.toml

**Valid locations for guardrail3:**
1. Per-app workspace root (e.g., `apps/validator-rust/deny.toml`) — covers all crates in that workspace when `cargo deny` runs there
2. Per standalone crate (e.g., `apps/substack-publisher/deny.toml`)
3. Root workspace (for root-level packages)

**Rule: one deny.toml per Rust workspace/standalone crate. Same scoping as clippy.toml.**

**Why not one global deny.toml:** cargo-deny runs per workspace. Each workspace has its own `Cargo.lock`, its own dependency tree. A root deny.toml would only cover the root workspace's deps (packages). Each app workspace needs its own deny.toml because its dependency tree is independent.

## Contents — 6 sections

### [graph] — dependency graph construction

| Key | guardrail3 value | User-customizable? |
|---|---|---|
| all-features | true | No — guardrail forces max coverage |
| no-default-features | false | No |
| targets | (not set) | Yes — user might restrict to specific targets |
| exclude | (not set) | Yes — user might exclude specific crates |
| exclude-dev | (not set) | Yes |

**guardrail3 manages:** `all-features`, `no-default-features`
**User owns:** `targets`, `exclude`, `exclude-dev`

### [bans] — dependency restrictions

| Key | guardrail3 value | User-customizable? |
|---|---|---|
| multiple-versions | "warn" | Yes — user choice. NOTE: the guardrail3 code currently generates "deny" which is WRONG — must be fixed to "warn". Both real-world files use "warn" with project-specific reasons. |
| wildcards | "allow" | Yes — steady-parent has "warn" in substack-publisher. User choice. |
| allow-wildcard-paths | true | Yes |
| highlight | "all" | No — guardrail forces max visibility |
| deny | [...24 crate bans...] | Partially — guardrail baseline + user additions + user wrappers |
| skip | [...] | Yes — 100% user-owned (specific versions to skip) |
| skip-tree | [...] | Yes — 100% user-owned |
| features | [...] | Partially — guardrail baseline (tokio) + user additions |

**[bans].deny array — the critical one:**

Guardrail3 baseline bans (24 crates, service profile):
- JSON: simd-json, json5, sonic-rs
- HTTP: ureq, surf, isahc
- Logging: log4rs, env_logger, simple_logger, fern
- Async: async-std, smol
- Error handling: anyhow
- Serialization: bincode, rmp-serde
- Web: actix-web, rocket, warp, poem
- ORM: diesel, sea-orm
- Binary: prost, flatbuffers
- Crypto: openssl

Library profile adds: tokio, reqwest, axum, tower, hyper, sqlx (I/O crates banned in libraries)

**KNOWN ISSUE: chrono ban.** guardrail3's `DENY_BANS_DATETIME` module bans chrono, but steady-parent's validator-rust explicitly allows it with `# NOTE: chrono is intentionally allowed`. The datetime ban module should be CONDITIONAL — only applied when the project opts in, or removed from the default baseline. On merge, adding a chrono ban to a project that uses chrono would break the build. Options:
1. Remove chrono from baseline (too aggressive for a default)
2. Make datetime bans opt-in via config (`[rust.apps.X.checks] datetime_bans = true`)
3. Keep in baseline but on merge, only ADD if the crate isn't in the dependency tree (if they depend on chrono, don't ban it)
Option 3 is the smartest — it's context-aware. But option 1 is simplest.

Each deny entry has: `name` (required), `wrappers` (optional — crates allowed to depend on the banned crate), `use-instead` (optional — suggested replacement), `reason` (optional), `deny-multiple-versions` (optional).

**What we enforce:** The `name` must be present in the deny list. We do NOT enforce `wrappers` — that's per-app (validator-rust allows texting_robots to use anyhow, substack-publisher doesn't).

**What we preserve:** User's `wrappers`, `use-instead`, `reason` on existing entries. User's extra bans beyond baseline. User's `skip`, `skip-tree` arrays entirely.

### [bans.features] — per-crate feature restrictions

Guardrail3 baseline: tokio feature ban (deny "full", allow specific features). But steady-parent has this COMMENTED OUT with a note about spider/lychee pulling "full" transitively. This is a per-app decision.

**guardrail3 manages:** baseline feature ban template
**User owns:** which feature bans are active (can comment out with reason)

### [licenses] — license compliance

| Key | guardrail3 value | User-customizable? |
|---|---|---|
| allow | [...12 licenses...] | Partially — baseline + user additions |
| confidence-threshold | 0.8 | Yes — user might want stricter (validate warns if looser) |
| exceptions | (not set) | Yes — 100% user-owned |

**[licenses].allow array:**
Guardrail3 baseline: MIT, Apache-2.0, BSD-3-Clause, ISC, Unicode-3.0, BSD-2-Clause, BSL-1.0, MPL-2.0, CDLA-Permissive-2.0, OpenSSL, Zlib, CC0-1.0

Both steady-parent apps have identical license lists matching the baseline.

**What we enforce:** Baseline licenses must be present. Additional licenses are allowed (user's project may need more).
**What we preserve:** User's extra licenses, user's `[licenses] exceptions` array entirely.

### [advisories] — security advisory checks

| Key | guardrail3 value | User-customizable? |
|---|---|---|
| ignore | (not set) | Yes — 100% USER-OWNED |

**This is the most project-specific section.** Advisory ignores are completely different between apps:
- validator-rust: RUSTSEC-2025-0057 (fxhash/scraper), RUSTSEC-2024-0388 (derivative/lychee-lib)
- substack-publisher: RUSTSEC-2025-0134 (rustls-pemfile/aws-sdk), RUSTSEC-2026-0009 (serde_json/aws-sdk)

These change frequently as new advisories are published and transitive deps update.

**guardrail3 manages:** Structure only (ensure section exists with safe defaults on NEW files). On EXISTING files, LEAVE ENTIRELY — do not modify any key.
**Rationale:** Advisory ignores are 100% determined by the app's transitive dependency tree. guardrail3 can't predict which advisories a project needs to ignore.
**NOTE:** The current code generates `unmaintained = "workspace"` and `yanked = "warn"` — these should be scaffold-once defaults, not enforced on merge. If user has `unmaintained = "deny"` (stricter), we must not weaken it.

### [sources] — registry restrictions

| Key | guardrail3 value | User-customizable? |
|---|---|---|
| unknown-registry | "deny" | No — guardrail forces |
| unknown-git | "deny" | No — guardrail forces |
| allow-registry | ["https://github.com/rust-lang/crates.io-index"] | Yes — user might add private registries |
| allow-git | [] | Yes — user might allow specific git sources |

**What we enforce:** `unknown-registry = "deny"`, `unknown-git = "deny"`, crates.io in allow-registry.
**What we preserve:** User's extra registries, user's git source allowlist.

## Algorithm

### On `generate` (existing file):

```
1. Parse existing deny.toml with toml_edit (preserves comments, formatting)
2. Load per-app removal files

3. [graph] section:
   - Ensure all-features = true, no-default-features = false
   - LEAVE all other keys (targets, exclude, etc.)

4. [bans] section:
   - multiple-versions: if missing ADD "warn". If present LEAVE (validate warns if "allow")
   - wildcards: if missing ADD "allow". If present LEAVE (user choice)
   - allow-wildcard-paths: if missing ADD true. If present LEAVE
   - highlight: if missing ADD "all". If present LEAVE
   - [bans.build]: LEAVE entirely if present (advanced user config)
   - [bans.workspace-dependencies]: LEAVE entirely if present
   - deny array:
     - For each guardrail baseline crate name:
       - If name is in removals: SKIP (validate warns)
       - If name missing: ADD { name = "X", wrappers = [] }
       - If name present: LEAVE entire entry (preserve wrappers, use-instead, reason)
     - User entries not in baseline: LEAVE
   - skip array: LEAVE entirely (100% user-owned)
   - skip-tree array: LEAVE entirely
   - LEAVE all comments (especially EXCEPTION and NOTE comments)

5. [bans.features] section:
   - If missing and tokio feature ban enabled: ADD baseline
   - If present: LEAVE (user may have commented out with reason)

6. [licenses] section:
   - allow array: ensure baseline licenses present, LEAVE user extras
   - confidence-threshold: if missing ADD 0.8. If present LEAVE (validate warns if < 0.8)
   - exceptions: LEAVE entirely
   - private: ensure ignore = true

7. [advisories] section:
   - LEAVE ENTIRELY. Do not touch. 100% user-owned.

8. [sources] section:
   - Ensure unknown-registry = "deny", unknown-git = "deny"
   - Ensure crates.io in allow-registry
   - LEAVE user's extra registries and git sources

9. Write back with toml_edit
```

### On `generate` (new file):
```
1. Build from template: graph + bans (profile baseline) + feature bans + licenses + advisories (empty ignore) + sources
2. Write complete file
```

### On `generate --dry-run`:
```
For existing file:
- Show missing baseline crate bans
- Show missing baseline licenses
- Show relaxed settings (multiple-versions = "allow", confidence-threshold < 0.8)
- Show "no changes needed" if fully compliant
- Do NOT show advisory ignores (not our business)

For new file:
- Show "would create" with profile and ban count
```

## Override mechanism

Same as clippy.toml: merge-managed means user content is preserved in the file itself. Overrides only needed for removals.

`.guardrail3/overrides/apps/{name}/deny-bans-remove.toml` — baseline crate bans to skip for this app.

Example: a project that legitimately needs anyhow:
```toml
# .guardrail3/overrides/apps/my-app/deny-bans-remove.toml
{ name = "anyhow", reason = "Used for CLI error reporting — no wrappers needed" }
```

Generate skips adding anyhow ban. Validate warns "guardrail relaxed: anyhow ban removed — reason: ..."

**Wrappers are NOT overrides.** If anyhow is banned with `wrappers = ["texting_robots"]`, the ban still exists — texting_robots is just exempted from the ban check. This is a per-app configuration in the deny.toml itself, not in an override file.

## Edge cases

1. **Different TOML formats for deny entries:** Some projects use `"crate-name"` (bare string) instead of `{ name = "crate-name", wrappers = [] }` (inline table). cargo-deny accepts both. Example: `deny = ["anyhow"]` vs `deny = [{ name = "anyhow" }]`. The merge algorithm must handle both when checking if a name is present.

2. **deny.exceptions.toml:** cargo-deny supports a separate exceptions file. guardrail3 should NOT touch this file — it's user-owned. But guardrail3 should be AWARE it exists (don't flag license exceptions as missing if they're in the exceptions file).

3. **Commented-out sections:** validator-rust has a commented-out `[[bans.features]]` section with a 4-line note about spider/lychee. The merge must preserve this — it's documentation of a deliberate decision.

4. **NOTE and EXCEPTION comments:** validator-rust has inline comments like `# NOTE: chrono is intentionally allowed` and `# NOTE: once_cell is NOT banned`. These are project-specific documentation. toml_edit preserves comments attached to the nearest item.

5. **[bans.build] and [bans.workspace-dependencies] sections:** These cargo-deny features exist but the plan doesn't mention them. On merge: if present, LEAVE entirely. These are advanced user configs. guardrail3 doesn't generate them but must not destroy them.

6. **"DO NOT EDIT" header on merge-managed files:** The current generated header says "DO NOT EDIT — regenerate with guardrail3 generate." This is wrong for merge-managed files where users ARE expected to add wrappers, advisory ignores, skip entries. Header should say: "Baseline managed by guardrail3 — your additions are preserved on regenerate. Baseline entries are enforced; do not remove them without a removal override."

7. **Library profile has more bans:** Library profile bans I/O crates (tokio, reqwest, axum, etc.) that service profile allows. If an app switches from service to library profile, the merge must ADD these extra bans. If switching from library to service, the extra bans become user entries (we don't remove them — removing bans weakens security).

6. **No root deny.toml:** Root packages in steady-parent have no deny.toml. If `[rust.packages]` exists in guardrail3.toml, generate should create one with library profile. But root packages have their own dependency tree (from root Cargo.lock) — the deny.toml needs to match THAT tree's advisories, not the apps' trees.

## Parser

Same as clippy.toml: `toml_edit` crate for comment-preserving merge.

Key difference from clippy.toml: deny.toml has multiple SECTIONS with different merge strategies (bans: merge deny array; advisories: leave alone; licenses: merge allow array; sources: ensure keys). The parser needs section-aware logic.

Entry indexing: by `name` field in deny array entries. Handle both string format (`"crate-name"`) and table format (`{ name = "crate-name" }`).

## Summary of what we own vs preserve

| Section | guardrail3 owns | User owns |
|---|---|---|
| [graph] | all-features, no-default-features | targets, exclude, exclude-dev |
| [bans] top-level | highlight | multiple-versions, wildcards, allow-wildcard-paths |
| [bans].deny | baseline crate names | wrappers, use-instead, reason, extra bans |
| [bans].skip | nothing | everything |
| [bans].skip-tree | nothing | everything |
| [bans].features | baseline template | active/commented state, extra feature bans |
| [licenses].allow | baseline 12 licenses | extra licenses |
| [licenses].confidence-threshold | 0.8 (warn if looser) | value choice |
| [licenses].exceptions | nothing | everything |
| [licenses].private | ignore = true | nothing |
| [advisories] | NOTHING | EVERYTHING |
| [sources] | unknown-registry, unknown-git, crates.io | extra registries, git sources |
