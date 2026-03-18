# deny.toml

## Location

**Where cargo-deny looks:** In the current working directory (same dir as the Cargo.toml being checked). Does NOT walk up parent directories. Falls back to default config if not found. Can be overridden with `--config <path>`.

**Verified:** Running `cargo deny check` from `apps/validator-rust/` finds `apps/validator-rust/deny.toml`. Running from repo root with no deny.toml warns "unable to find a config path, falling back to default config."

**In steady-parent:**
- `apps/validator-rust/deny.toml` (109 lines)
- `apps/substack-publisher/deny.toml` (68 lines)
- NO root deny.toml — root workspace packages have no deny checking

**Rule: one deny.toml per Rust workspace/standalone crate. Same as clippy.toml.**

## Contents — what cargo-deny supports

deny.toml has 5 top-level sections: `[graph]`, `[bans]`, `[licenses]`, `[advisories]`, `[sources]`.

### What guardrail3 manages (sections and specific keys):

**`[graph]`** — fully guardrail-owned:
- `all-features = true`
- `no-default-features = false`

**`[bans]` settings** — guardrail defaults, user can relax with EXCEPTION:
- `multiple-versions` — guardrail3 says `"deny"`. validator-rust has `"warn"`. substack-publisher has `"warn"` with EXCEPTION comment about AWS SDK. This is like a threshold — user can relax, we warn.
- `wildcards` — guardrail3 says `"allow"`. substack-publisher has `"warn"`. This one is interesting — the user is STRICTER. Leave it.
- `allow-wildcard-paths = true` — guardrail default
- `highlight = "all"` — guardrail default

**`[bans] deny` array** — guardrail baseline ban list (23 crates):
simd-json, json5, sonic-rs (JSON), ureq, surf, isahc (HTTP), log4rs, env_logger, simple_logger, fern (logging), async-std, smol (async), anyhow (error), bincode, rmp-serde (serialization), actix-web, rocket, warp, poem (web), diesel, sea-orm (ORM), prost, flatbuffers (binary), openssl (crypto)

Each entry has `name` and `wrappers`. The `wrappers` field is the key per-app customization — validator-rust has `anyhow` with `wrappers = ["texting_robots"]` while substack-publisher has `wrappers = []`.

**`[bans] skip` array** — user-owned. Crates to skip in duplicate checking. guardrail3 doesn't have baseline skip entries.

**`[[bans.features]]`** — guardrail3 has tokio feature restriction (deny "full", allow specific features). User may disable this (commented out in validator-rust because spider/lychee pull "full" transitively).

**`[licenses]`** — guardrail baseline:
- `allow` array: 12 licenses (MIT, Apache-2.0, BSD-3-Clause, ISC, Unicode-3.0, BSD-2-Clause, BSL-1.0, MPL-2.0, CDLA-Permissive-2.0, OpenSSL, Zlib, CC0-1.0)
- `confidence-threshold = 0.8`
- `[licenses.private] ignore = true`
- User may need ADDITIONAL licenses (e.g., `"Unicode-DFS-2016"` for ICU deps)

**`[advisories]`** — 100% user-owned:
- `ignore` array: completely different per app. These are transitive dep security advisories that the user has reviewed and accepted. Changes with every dep update. guardrail3 has NO baseline here.
- validator-rust: RUSTSEC-2025-0057 (fxhash/scraper), RUSTSEC-2024-0388 (derivative/lychee-lib)
- substack-publisher: RUSTSEC-2025-0134 (rustls-pemfile/aws-sdk), RUSTSEC-2026-0009 (serde_json/aws-sdk)

**`[sources]`** — guardrail baseline:
- `unknown-registry = "deny"`
- `unknown-git = "deny"`
- `allow-registry = ["https://github.com/rust-lang/crates.io-index"]`
- `allow-git = []`
- User may need to add git sources for private crates

### User-owned keys guardrail3 must NOT touch:
- `[advisories] ignore` — entirely project-specific, changes frequently
- `[bans] skip` — project-specific duplicate exceptions
- `[bans] deny` entries with `wrappers` — the wrapper list is per-app (anyhow allowed via texting_robots)
- `[[bans.features]]` user modifications — commented-out sections with NOTEs explaining why
- Comments throughout — "NOTE: chrono is intentionally allowed", "EXCEPTION: 18 AWS SDK transitive duplicates"
- `[licenses] exceptions` — per-crate license exceptions (not present in steady-parent but common)
- `[sources] allow-git` entries — private crate repos

## Algorithm

### On `generate` (existing file):

```
1. Parse existing deny.toml with toml_edit
2. [graph]: ensure all-features and no-default-features present with guardrail values
3. [bans] settings:
   - multiple-versions: if missing, add "deny". If present, LEAVE (validate warns if not "deny")
   - wildcards: if missing, add "allow". If present, LEAVE
   - allow-wildcard-paths: ensure true
   - highlight: ensure "all"
4. [bans] deny array:
   - Index existing entries by `name` field
   - For each guardrail baseline crate:
     - If in removals: SKIP (validate warns)
     - If name missing: ADD with `wrappers = []`
     - If name present: LEAVE entire entry (preserve wrappers, any other fields)
   - User entries not in baseline: LEAVE
5. [bans] skip array: LEAVE entirely (user-owned)
6. [[bans.features]]:
   - If guardrail3 tokio feature ban should be present AND section missing: ADD
   - If section exists (even commented out): LEAVE (user may have disabled with NOTE)
7. [licenses]:
   - allow array: ensure all 12 baseline licenses present. User extras: LEAVE
   - confidence-threshold: ensure 0.8 or stricter
   - [licenses.private]: ensure ignore = true
   - [licenses] exceptions: LEAVE (user-owned)
8. [advisories]: LEAVE ENTIRELY. 100% user-owned.
9. [sources]:
   - ensure unknown-registry = "deny"
   - ensure unknown-git = "deny"
   - ensure allow-registry contains crates.io
   - allow-git: LEAVE (user may have private repos)
10. Write back with toml_edit
```

### On `generate` (new file):
```
Build from template: all sections with guardrail defaults.
Empty [advisories] ignore = [].
Empty [bans] skip = [].
```

### On `generate --dry-run`:
```
Show per-section status:
- Which baseline ban crates are MISSING
- Which [bans] settings differ from guardrail (with current value)
- Whether [licenses] allow list is complete
- [advisories] ignore count (informational — not an error)
- "no changes needed" if fully compliant
```

## Override mechanism

Same as clippy.toml — with merge-managed, the file IS the source of truth. User adds entries directly.

**Only removals needed:**
- `.guardrail3/overrides/apps/{name}/deny-bans-remove.toml` — baseline crate bans to skip for this app
- Example: app needs `anyhow` directly (not via wrapper): put `{ name = "anyhow" }` in remove file

**Wrappers are NOT removals.** If validator-rust needs anyhow via texting_robots, that's handled by the `wrappers` field on the ban entry itself. The user edits the entry directly in deny.toml: `{ name = "anyhow", wrappers = ["texting_robots"] }`. guardrail3 sees anyhow is present and leaves it.

## Key differences from clippy.toml

1. **`[advisories] ignore` is 100% user-owned.** clippy.toml has no equivalent — every section has guardrail content. deny.toml has an entire section (advisories) that guardrail3 never touches.

2. **Wrappers field.** clippy entries have `path, reason, replacement, allow-invalid`. deny ban entries have `name, wrappers, version-req`. The `wrappers` field is the per-app customization — same crate banned everywhere, but with different wrapper exceptions.

3. **`[bans] settings are like thresholds.** `multiple-versions = "warn"` vs guardrail3's `"deny"` is a relaxation. Same enforcement model: LEAVE but validate warns.

4. **Feature bans are tricky.** The `[[bans.features]]` section uses TOML array-of-tables syntax. Users may comment it out entirely with notes. The merge algorithm needs to detect both active and commented-out feature ban sections and not re-add what the user deliberately removed. If the section is missing entirely (never was there), add it. If it was there and was commented out, leave it.

## Edge cases

1. **User has `[licenses] exceptions` for a specific crate.** LEAVE — these are per-crate license exceptions like `{ allow = ["AGPL-3.0"], name = "my-internal-crate" }`. guardrail3 has no opinion on these.

2. **`deny.exceptions.toml` file.** cargo-deny supports a separate exceptions file that overrides deny.toml. guardrail3 should be aware this exists but doesn't need to manage it.

3. **User adds licenses to the allow list.** If user adds "Unicode-DFS-2016" to licenses.allow, guardrail3 LEAVES it. The license allow list is additive — more allowed licenses is fine.

4. **User REMOVES a baseline license.** Like removing "MPL-2.0" from the allow list. Next generate adds it back. To legitimately remove: use a deny-licenses-remove.toml override (but this is very unlikely — removing a license from allow makes the project MORE restrictive, which is the user's choice). Actually — removing a license from allow means some deps might fail the check. That's a user choice. Should we re-add it? No — license policy is the user's call. If they remove MPL-2.0, they decided they don't want MPL-2.0 deps. We should only WARN if a baseline license is missing, not force it.

5. **Commented-out sections.** validator-rust has commented-out `[[bans.features]]` with a 4-line NOTE. toml_edit preserves comments. The merge algorithm must not re-add a section that exists as a comment — treat commented-out sections as "user deliberately disabled."

6. **`openssl-sys` vs `openssl`.** guardrail3 bans both. steady-parent only bans `openssl`. If `openssl-sys` is missing, generate adds it. Not destructive — just adds a ban.

## Parser

Same as clippy.toml: `toml_edit` crate.

Key operations:
- Parse `[bans] deny` as array, index by `name` field
- Parse `[licenses] allow` as array of strings
- Parse `[advisories] ignore` as array — but NEVER modify
- Detect table-array syntax (`[[bans.features]]`) vs inline
- Preserve ALL comments (especially EXCEPTION and NOTE comments)
