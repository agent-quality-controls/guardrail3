# deny.toml

## How cargo-deny finds its config (empirically verified 2026-03-19)

cargo deny walks UP from CWD looking for `deny.toml`, `.deny.toml`, or `.cargo/deny.toml`. Nearest wins. No merging.

### Walk-up verified

- From `apps/validator-rust/`: finds `apps/validator-rust/deny.toml` immediately.
- From `apps/validator-rust/crates/domain/`: walks up through `crates/`, finds `apps/validator-rust/deny.toml`.
- From `apps/validator-rust/crates/adapters/outbound/`: with intermediate `crates/adapters/deny.toml`, walks up to `crates/adapters/` and finds IT — shadows workspace root deny.toml.

### Walk-up crosses workspace boundaries

With no `deny.toml` at `apps/validator-rust/`, running from `apps/validator-rust/` found `deny.toml` at project root. Walk-up does NOT stop at `[workspace]` Cargo.toml boundary — same as clippy.

### Intermediate shadowing verified

`deny.toml` at `crates/adapters/` shadows workspace root for `crates/adapters/outbound/` and `crates/adapters/inbound/api/` but NOT for `crates/domain/` or `crates/app/` (siblings, not children). Exact same behavior as clippy.

### Priority: deny.toml > .deny.toml > .cargo/deny.toml

Verified: when both `deny.toml` and `.deny.toml` exist in the same directory, `deny.toml` (without dot) wins. `.deny.toml` is silently ignored.

All three variants work independently: `deny.toml`, `.deny.toml`, `.cargo/deny.toml`.

### CWD must have Cargo.toml

cargo deny errors if CWD has no `Cargo.toml`. It does NOT walk up to find `Cargo.toml` — only the config file walks up. `--manifest-path` can override.

### Difference from clippy

Config resolution is identical (walk-up, nearest wins, crosses workspace boundaries). The operational difference: cargo deny checks the ENTIRE workspace dependency tree, while clippy checks per-crate with per-crate `CARGO_MANIFEST_DIR`. But for config file resolution, the behavior is the same.

**In steady-parent:**
- `apps/validator-rust/deny.toml` (109 lines)
- `apps/substack-publisher/deny.toml` (68 lines)
- NO root deny.toml — root workspace packages have no deny checking

**Checker rule:** deny placement follows the same root/coverage model as clippy:
- allowed only at validation root, workspace roots, and standalone package roots not inside a workspace
- every Rust root must be covered by an effective deny config
- nested deny configs below an allowed root are forbidden unless the deeper directory is itself another allowed root

## Contents — what cargo-deny supports

deny.toml has 5 top-level sections: `[graph]`, `[bans]`, `[licenses]`, `[advisories]`, `[sources]`.

### What guardrail3 manages (sections and specific keys):

**`[graph]`** — fully guardrail-owned:
- `all-features = true`
- `no-default-features = false`

**`[bans]` settings** — split by invariant:
- `multiple-versions` — guardrail3 says `"deny"`. validator-rust has `"warn"`. substack-publisher has `"warn"` with EXCEPTION comment about AWS SDK. This is like a threshold — user can relax, we warn.
- `wildcards` — guardrail3 says `"allow"`. substack-publisher has `"warn"`. This one is interesting — the user is STRICTER. Leave it.
- `allow-wildcard-paths = true` — hard guardrail requirement
- `highlight = "all"` — guardrail default

**`[bans] deny` array** — guardrail baseline ban list (23 crates):
simd-json, json5, sonic-rs (JSON), ureq, surf, isahc (HTTP), log4rs, env_logger, simple_logger, fern (logging), async-std, smol (async), anyhow (error), bincode, rmp-serde (serialization), actix-web, rocket, warp, poem (web), diesel, sea-orm (ORM), prost, flatbuffers (binary), openssl (crypto)

Each entry has `name` and `wrappers`. The `wrappers` field is the key per-app customization — validator-rust has `anyhow` with `wrappers = ["texting_robots"]` while substack-publisher has `wrappers = []`.

**`[bans] skip` array** — user-owned. Crates to skip in duplicate checking. guardrail3 doesn't have baseline skip entries.

**`[[bans.features]]`** — guardrail3 has tokio feature restriction (deny "full", allow specific features). The checker treats both the deny side and the explicit allow-list as canonical policy. Extra feature-ban entries remain user-owned and are inventoried separately.

**`[licenses]`** — guardrail baseline:
- `allow` array: 12 licenses (MIT, Apache-2.0, BSD-3-Clause, ISC, Unicode-3.0, BSD-2-Clause, BSL-1.0, MPL-2.0, CDLA-Permissive-2.0, OpenSSL, Zlib, CC0-1.0)
- `confidence-threshold = 0.8` minimum; stricter is fine
- `[licenses.private] ignore = true`
- User may need ADDITIONAL licenses (e.g., `"Unicode-DFS-2016"` for ICU deps)
- `[licenses].exceptions` is user-owned for generate but should be inventoried by validate

**`[advisories]`** — split model:
- guardrail-managed baseline:
  - `unmaintained = "workspace"`
  - `yanked = "warn"`
- user-owned audit surface:
  - `ignore` array is still completely different per app. These are transitive dep security advisories that the user has reviewed and accepted. Changes with every dep update.
- validator-rust: RUSTSEC-2025-0057 (fxhash/scraper), RUSTSEC-2024-0388 (derivative/lychee-lib)
- substack-publisher: RUSTSEC-2025-0134 (rustls-pemfile/aws-sdk), RUSTSEC-2026-0009 (serde_json/aws-sdk)

**`[sources]`** — guardrail baseline:
- `unknown-registry = "deny"`
- `unknown-git = "deny"`
- `allow-registry` must contain crates.io, accepting either:
  - `https://github.com/rust-lang/crates.io-index`
  - `sparse+https://index.crates.io/`
- `allow-git = []`
- User may need to add git sources for private crates

### User-owned keys guardrail3 must NOT touch during generate:
- `[advisories] ignore` — entirely project-specific, changes frequently
- `[bans] skip` — project-specific duplicate exceptions
- `[bans] deny` entries with `wrappers` — the wrapper list is per-app (anyhow allowed via texting_robots)
- `[[bans.features]]` user modifications — commented-out sections with NOTEs explaining why
- Comments throughout — "NOTE: chrono is intentionally allowed", "EXCEPTION: 18 AWS SDK transitive duplicates"
- `[licenses] exceptions` — per-crate license exceptions (not present in steady-parent but common)
- `[sources] allow-git` entries — private crate repos

The checker may still warn or inventory these because they weaken policy or create audit surface.

## Algorithm

### On `generate` (existing file):

```
1. Parse existing deny.toml with toml_edit
2. [graph]: ensure all-features and no-default-features present with guardrail values
3. [bans] settings:
   - multiple-versions: if missing, add "deny". If present, LEAVE (validate warns if weaker)
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
   - [licenses] exceptions: LEAVE (user-owned, checker inventories)
8. [advisories]: LEAVE ENTIRELY. 100% user-owned.
9. [sources]:
   - ensure unknown-registry = "deny"
   - ensure unknown-git = "deny"
   - ensure allow-registry contains crates.io
   - allow-git: LEAVE (user may have private repos; checker warns/inventories)
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

4. **User REMOVES a baseline license.** This is a real policy decision, not necessarily a bug. Generate should avoid silently broadening license policy by re-adding removed licenses. Validate can still warn or error according to the current `RS-DENY` contract.

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

## Checker-specific notes

The checker should be stricter than generate in a few places:
- validate effective coverage, not just file existence
- detect nested shadow deny configs
- warn on unknown keys in critical sections
- warn on duplicate entries in `deny`, `skip`, `ignore`, and `[[bans.features]]`
- monitor `[bans].allow`, `[licenses].exceptions`, and `allow-git` as escape hatches
- derive the canonical deny baseline from the generator modules instead of duplicating a prose count
