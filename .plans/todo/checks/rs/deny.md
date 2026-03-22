# RS-DENY — deny.toml checker (20 rules)

**Input:** deny.toml (one per workspace)
**Parser:** TOML (`toml::Value`)
**Current code:** `deny_audit.rs`, `deny_bans.rs`, `deny_licenses.rs`, `deny_inventory.rs`

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-DENY-01 | R8 | Error | deny.toml exists at workspace root | Implemented |
| RS-DENY-02 | R9 | Warn | Deprecated fields in [advisories] (vulnerability, notice, unsound) | Implemented |
| RS-DENY-03 | R10 | Error | [advisories] section: unmaintained + yanked settings | Implemented |
| RS-DENY-04 | R11 | Info | Advisory settings stricter than expected (unmaintained=deny, yanked=deny) | Implemented |
| RS-DENY-05 | R12 | Error | [bans] section: multiple-versions=deny, deny array with 35 expected bans | Implemented |
| RS-DENY-06 | R13 | Info | Extra bans beyond baseline (inventory) + highlight setting | Implemented |
| RS-DENY-07 | R14 | Error | [licenses] section: allow list, private.ignore=true | Implemented |
| RS-DENY-08 | R15 | Info | confidence-threshold setting | Implemented |
| RS-DENY-09 | R16 | Error | [sources] section: unknown-registry=deny, unknown-git=deny, registries, git sources | Implemented |
| RS-DENY-10 | R17 | Warn | [[bans.features]] tokio "full" feature banned | Implemented |
| RS-DENY-11 | R18 | Info | Extra feature bans beyond tokio (inventory) | Implemented |
| RS-DENY-12 | R19 | Warn/Info | [bans.skip] entries: warn if no reason, info inventory | Implemented |
| RS-DENY-13 | R20 | Warn/Info | [advisories.ignore] entries: warn if no reason, info inventory | Implemented |
| RS-DENY-14 | — | Error | [graph] section: all-features=true must be set | Todo |
| RS-DENY-15 | — | Warn | [bans] wildcards and allow-wildcard-paths consistency | Todo |
| RS-DENY-16 | — | Error | Library profile: deny list must include IO crate bans (axum, tokio, reqwest, sqlx, hyper, etc.) | Todo |
| RS-DENY-17 | — | Warn | [licenses] allow list must not contain copyleft licenses (GPL, AGPL, LGPL, SSPL, EUPL) | Todo |
| RS-DENY-18 | — | Info | [bans] deny entries without a `reason` field (inventory) | Todo |
| RS-DENY-19 | — | Warn | [advisories.ignore] excessive count (threshold: 5) | Todo |
| RS-DENY-20 | — | Warn | [bans].allow is non-empty — may override deny entries | Todo |

## Expected bans additions

| Crate | Why | Status |
|-------|-----|--------|
| `lazy_static` | Legacy lazy init macro. `std::sync::LazyLock` (stable since 1.80) is the replacement. Unnecessary dependency. | NOT in EXPECTED_BANS — must add |

## Bug fixes on existing rules

| Target | Bug | What |
|--------|-----|------|
| RS-DENY-09 | M9 | Accept sparse protocol URL `sparse+https://index.crates.io/` as valid crates.io registry (in addition to existing git URL) |
| RS-DENY-12 | B1 | Warn on malformed skip entries (missing both `crate` and `name` fields) instead of silently reporting "unknown" |
| RS-DENY-12 | B2 | Warn when `reason` field is present but not a string instead of treating as empty |
| RS-DENY-13 | B1 | Same malformed-entry handling as RS-DENY-12 |
| RS-DENY-13 | B3 | Add missing `.as_inventory()` on advisory ignore Info entries for `--inventory` consistency |

## New rule details

### RS-DENY-14 — `[graph]` section validation (Error)

**What:** The `[graph]` section must exist with `all-features = true`. Without this, cargo-deny only checks the default feature set, missing feature-gated dependencies entirely. This is a silent correctness hole — cargo-deny runs but inspects a subset of the dependency tree.

**Check:**
1. `[graph]` section exists → if missing, Error
2. `all-features` key exists and is `true` → if missing or `false`, Error

**Message:** "deny.toml [graph] section must set all-features = true to check all feature-gated dependencies"

### RS-DENY-15 — Wildcards consistency (Warn)

**What:** The generated template sets `wildcards = "allow"` and `allow-wildcard-paths = true` in `[bans]`. If `wildcards` is set to `"deny"` without `allow-wildcard-paths = true`, workspace path dependencies are rejected. If both are removed, behavior depends on cargo-deny version defaults.

**Check:**
1. If `wildcards` is present and not `"allow"`, and `allow-wildcard-paths` is not `true` → Warn
2. If neither `wildcards` nor `allow-wildcard-paths` is present → Warn (relying on cargo-deny defaults is fragile)

**Message:** "deny.toml [bans] should set wildcards = \"allow\" with allow-wildcard-paths = true for workspace path dependency compatibility"

### RS-DENY-16 — Library profile IO crate bans (Error)

**What:** When the project profile is `library`, the deny list must include IO crate bans that prevent libraries from pulling in runtime dependencies. The generator adds these via `DENY_BANS_LIBRARY_IO`, but the validator ignores the `_profile` parameter entirely.

**Fix approach:** When `profile == Some("library")`, extend the expected ban set with the library-IO crates: `axum`, `tokio`, `reqwest`, `sqlx`, `hyper`, `tonic`, `tower`, `warp`, `actix-web`, `rocket`, `tide`, `poem`, `salvo`. Crates already in the service baseline are naturally deduplicated.

**Check:** Same logic as RS-DENY-05 but with the extended set when profile is `library`.

**Message:** "Library profile: deny.toml is missing ban for IO crate '{name}' — libraries must not depend on runtime/IO crates"

### RS-DENY-17 — Copyleft license detection (Warn)

**What:** The `[licenses].allow` list should only contain permissive licenses. Adding copyleft licenses (GPL, AGPL, LGPL, SSPL, EUPL) defeats the purpose of license checking for most commercial projects.

**Blocked licenses:** `GPL-2.0-only`, `GPL-2.0-or-later`, `GPL-3.0-only`, `GPL-3.0-or-later`, `AGPL-3.0-only`, `AGPL-3.0-or-later`, `LGPL-2.1-only`, `LGPL-2.1-or-later`, `LGPL-3.0-only`, `LGPL-3.0-or-later`, `SSPL-1.0`, `EUPL-1.2`

Also match short forms without suffixes: `GPL-2.0`, `GPL-3.0`, `AGPL-3.0`, `LGPL-2.1`, `LGPL-3.0`

**Check:** For each entry in `[licenses].allow`, if it matches a copyleft identifier → Warn

**Message:** "deny.toml [licenses] allow list contains copyleft license '{license}' — this may have viral licensing implications"

### RS-DENY-18 — Ban entry reason inventory (Info)

**What:** Each entry in `[bans].deny` should have a `reason` field explaining why the crate is banned. Consistent with RS-DENY-12 (skip reasons) and RS-DENY-13 (advisory ignore reasons).

**Check:** For each entry in `[bans].deny` that is a table (not just a string), if `reason` field is missing or empty → Info inventory

**Message:** "deny.toml ban entry '{name}' has no reason field"

### RS-DENY-19 — Advisory ignore accumulation (Warn)

**What:** If `[advisories].ignore` has more than 5 entries, the project is suppressing a concerning number of security advisories. This suggests either the dependencies need updating or the ignore list needs pruning.

**Check:** Count entries in `[advisories].ignore`. If > 5 → Warn

**Message:** "deny.toml [advisories].ignore has {count} entries (threshold: 5) — consider updating dependencies or auditing suppressions"

### RS-DENY-20 — Ban allow-list detection (Warn)

**What:** cargo-deny supports `[bans].allow` to whitelist specific crates, which overrides deny entries. If both `allow` and `deny` contain the same crate, cargo-deny resolves in favor of `allow`, silently permitting a banned crate.

**Check:** If `[bans].allow` exists and is a non-empty array → Warn. If any entry in `allow` also appears in `deny` → Error (explicit override of a ban).

**Message (Warn):** "deny.toml [bans].allow is non-empty ({count} entries) — allow entries override deny entries"
**Message (Error):** "deny.toml [bans].allow contains '{name}' which is also in the deny list — this silently overrides the ban"
