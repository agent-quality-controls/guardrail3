# Adversarial Audit: deny.toml Validation (R8-R20)

## Verdict: Multiple exploitable gaps, missing checks, and logic weaknesses

---

## FINDING 01 — `_profile` parameter is accepted but completely ignored (deny_bans.rs:9)

**Severity: HIGH**

`check_ban_list` accepts `_profile: Option<&str>` but never uses it. The comment on line 113 says "All profiles use the same expected bans. Unknown/missing defaults to service." But `deny.rs` defines TWO profiles with DIFFERENT ban lists:

- `service_profile_ban_entries()` — 26 crates
- `library_profile_ban_entries()` — 26 + 13 library-IO bans (axum, tokio, reqwest, sqlx, hyper, warp, rocket, actix-web, poem, ureq, surf, isahc — 13 additional bans)

**The validator only checks the service profile's 26 bans regardless of which profile the project uses.** A library project could have a deny.toml missing ALL library-IO bans (axum, tokio, reqwest, sqlx, hyper) and the validator would report zero errors. The entire library profile enforcement is a no-op.

**Fix:** When `profile == Some("library")`, the expected bans set must include `DENY_BANS_LIBRARY_IO` crates.

---

## FINDING 02 — No check for `[graph]` section settings

**Severity: MEDIUM**

The generated deny.toml includes `[graph] all-features = true` and `no-default-features = false`. These are critical for cargo-deny to work correctly — `all-features = true` ensures all feature-gated dependencies are checked. But `check()` in `deny_audit.rs` never validates the `[graph]` section.

An attacker (or sloppy agent) could remove `[graph]` entirely or set `all-features = false`, causing cargo-deny to silently skip feature-gated dependencies. A crate hidden behind a feature flag would never be checked.

**Fix:** Add a check that `[graph].all-features == true`.

---

## FINDING 03 — No check for `[bans].wildcards` or `allow-wildcard-paths`

**Severity: MEDIUM**

The generated deny.toml has `wildcards = "allow"` and `allow-wildcard-paths = true` (needed for workspace path deps). These settings are not validated. Someone could change `wildcards = "deny"` (breaking workspace deps) or remove them entirely (cargo-deny defaults may differ across versions).

More importantly, there's no check that these settings EXIST AT ALL.

---

## FINDING 04 — Advisory `ignore` entries don't require a `reason` field

**Severity: HIGH**

`check_advisory_ignores` (deny_inventory.rs:49-72) reports ignored advisories at `Severity::Info` but NEVER checks for a `reason` field. The project philosophy states "Every escape hatch is documented" and "every `#[allow]` must have a reason."

Advisory ignores are security escape hatches — suppressing known vulnerability alerts. They should REQUIRE a reason. The steady-parent deny.toml even shows the expected format: `{ id = "RUSTSEC-2025-0057", reason = "transitive dep..." }`.

But the validator accepts plain string entries like `"RUSTSEC-2024-0001"` without any reason. An agent could silently suppress critical vulnerabilities with no documentation trail.

**Fix:** If an advisory ignore entry has no `reason` field (or `reason` is empty), emit a `Severity::Warn` or `Severity::Error`.

---

## FINDING 05 — Advisory `ignore` entry format is only partially parsed

**Severity: MEDIUM**

`check_advisory_ignores` does `entry.as_str().unwrap_or("unknown")` (line 60). This only handles plain string entries like `"RUSTSEC-2024-0001"`. But cargo-deny 0.19+ supports table entries: `{ id = "RUSTSEC-2025-0057", reason = "..." }`.

If someone uses the table format (as shown in steady-parent's comments), `entry.as_str()` returns `None` and the ID is reported as "unknown". The actual advisory ID is lost. The check doesn't try `entry.get("id").and_then(|v| v.as_str())`.

**Fix:** Handle both string and table formats for advisory ignore entries, extracting `id` and `reason` from table entries.

---

## FINDING 06 — Skip entries are only inventoried, never validated

**Severity: HIGH**

`check_skip_entries` (deny_inventory.rs:5-47) emits every skip entry as `Severity::Info` with `.as_inventory()`. It NEVER:
- Checks if the skip entry has a `reason` field (escape hatch documentation principle)
- Warns about skip entries without reasons
- Limits the number of skip entries
- Validates that the skipped crate actually exists in the dependency tree

A malicious agent could add 50 skip entries to effectively disable the `multiple-versions = "deny"` check for the entire dependency tree. The validator would silently pass, emitting only Info-level inventory items.

**Fix:** Skip entries without a `reason` field should emit `Severity::Warn`. Consider a threshold warning for excessive skip entries.

---

## FINDING 07 — `unmaintained` accepts "allow" without error

**Severity: MEDIUM**

`check_unmaintained_value` (deny_audit.rs:167-223) handles "workspace", "deny", and "other". The "other" branch correctly errors. But the match is `Some("workspace")` and `Some("deny")` — meaning `Some("allow")` falls into the `Some(other)` error branch. Good.

HOWEVER, there's a subtle gap: if `unmaintained` is set to a non-string value (e.g., `unmaintained = true` or `unmaintained = 1`), `and_then(|v| v.as_str())` returns `None`, and the check reports "unmaintained missing" — which is misleading. The field IS present but has wrong type. The error message should distinguish "missing" from "wrong type."

---

## FINDING 08 — No check for `advisories.db-urls` or `advisories.git-fetch-with-cli`

**Severity: LOW-MEDIUM**

The validator checks `unmaintained` and `yanked` but ignores other advisory settings. Notably:
- `db-urls` — could be pointed at a malicious advisory database
- `git-fetch-with-cli` — affects how the advisory DB is fetched

These are less critical but represent unchecked surface area.

---

## FINDING 09 — `check_allow_registry` only checks for non-crates.io, doesn't verify crates.io IS present

**Severity: MEDIUM**

`check_allow_registry` (deny_licenses.rs:204-220) checks if `allow-registry` contains non-crates.io URLs. But it does NOT check:
1. That `allow-registry` EXISTS
2. That it contains the crates.io registry URL

If `allow-registry` is missing entirely, no error is emitted. Cargo-deny's default behavior when `allow-registry` is absent may vary by version — it could allow ALL registries.

**Fix:** If `allow-registry` is missing or empty, emit an error. If it doesn't contain the crates.io URL, warn.

---

## FINDING 10 — `confidence-threshold` check accepts any float value without warning

**Severity: LOW**

`check_confidence_threshold` (deny_licenses.rs:95-135) checks if the value differs from 0.8 but only emits `Severity::Info`. A threshold of `0.1` (accepting nearly anything as a license match) would pass with just an Info note. A very low confidence threshold effectively disables license checking.

Also, `confidence-threshold = 1` (integer, not float) would fall into the `_ => {}` catch-all and be silently ignored — no error, no warning, nothing.

**Fix:** Threshold values below some minimum (e.g., 0.5) should emit `Severity::Warn`. Integer values should be handled (TOML `1` vs `1.0`).

---

## FINDING 11 — License allow list content is never validated

**Severity: MEDIUM**

`check_license_allow_list` checks that `[licenses].allow` exists and is non-empty. But it NEVER validates the contents. Someone could set `allow = ["WTFPL", "AGPL-3.0"]` and the check would pass with `Severity::Info`.

The generated deny.toml has a specific set of 13 permissive licenses (MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Unicode-DFS-2016, Unicode-3.0, Zlib, CC0-1.0, OpenSSL, BSL-1.0, MPL-2.0, CDLA-Permissive-2.0). The validator should at least check that no copyleft/viral licenses (GPL-2.0, GPL-3.0, AGPL-3.0, SSPL-1.0) are in the allow list.

**Fix:** Maintain a DENIED_LICENSES list and error if any appear in the allow list. At minimum, flag GPL/AGPL/SSPL.

---

## FINDING 12 — Steady-parent deny.toml has `unsound = "all"` which is a deprecated field

**Severity: LOW (real-world)**

The steady-parent deny.toml (websmasher/apps/backend/deny.toml) has `unsound = "all"` on line 105. The guardrail3 validator correctly warns about deprecated fields (`check_deprecated_advisory_fields` checks for "unsound"). However, the generated deny.toml from guardrail3's `DENY_ADVISORIES` module does NOT include `unsound` at all — it was removed because it's deprecated.

This is actually correct behavior. Noted for completeness.

---

## FINDING 13 — Steady-parent deny.toml has `unmaintained = "all"` — not a valid value

**Severity: MEDIUM (real-world)**

The steady-parent deny.toml has `unmaintained = "all"` (line 104). The guardrail3 validator would error on this with "Expected 'workspace', got 'all'" — which is correct. But `"all"` is not even a valid cargo-deny value for `unmaintained`. Valid values are "deny", "warn", "allow", "workspace". This means cargo-deny itself would reject this file.

This is an existing bug in the steady-parent's deny.toml that guardrail3 would correctly flag.

---

## FINDING 14 — `check_tokio_feature_ban` doesn't validate the `allow` list

**Severity: MEDIUM**

The tokio feature ban check (deny_bans.rs:148-228) verifies that `deny = ["full"]` exists but NEVER validates the `allow` list. The generated deny.toml allows specific features: `["rt-multi-thread", "macros", "net", "sync", "signal", "bytes", "default", "io-util", "time"]`.

An agent could add `allow = ["full", "rt-multi-thread", ...]` — banning "full" but also explicitly allowing it. Or add dangerous features like `"process"` (spawns child processes) or `"fs"` (filesystem access) to the allow list.

The interaction between `deny` and `allow` in cargo-deny is: if a feature is in both, `deny` wins. So `allow = ["full"]` + `deny = ["full"]` would still ban it. But the allow list could be expanded to include features that shouldn't be allowed for the project's profile.

**Fix:** At minimum, warn if the tokio allow list contains features not in the expected set.

---

## FINDING 15 — No check for `[licenses].deny` list (inverse of allow)

**Severity: LOW**

cargo-deny supports `[licenses].deny` in addition to `[licenses].allow`. Someone could add `deny = []` (explicitly empty deny list) alongside a broad allow list. The validator only checks the allow list.

More importantly, the absence of a `deny` list is fine (the allow-list-only approach is correct), but if someone adds `deny = ["MIT"]` alongside `allow = ["MIT"]`, that's a contradiction the validator doesn't catch.

---

## FINDING 16 — `check_sources` doesn't verify `allow-registry` contains EXACTLY the expected URL

**Severity: LOW-MEDIUM**

The check only flags non-crates.io entries but doesn't verify the exact URL format. The expected URL is `https://github.com/rust-lang/crates.io-index`. An attacker could use `https://crates.io` (different URL) or a typosquat URL containing "crates.io" as a substring (e.g., `https://evil-crates.io-index.com`), and the `r.contains("crates.io")` check on line 207 would pass.

**Fix:** Check for exact URL match against the expected registry URL, not substring contains.

---

## FINDING 17 — EXPECTED_BANS count is correct (26 entries)

**Verification:** Counting the entries in `EXPECTED_BANS`:
simd-json, json5, sonic-rs (3) + openssl, openssl-sys (5) + ureq, surf, isahc (8) + log4rs, env_logger, simple_logger, fern (12) + async-std, smol (14) + anyhow (15) + actix-web, rocket, warp, poem (19) + chrono (20) + diesel, sea-orm (22) + bincode, rmp-serde, prost, flatbuffers (26).

All 26 match the service profile ban modules in `deny.rs`. This is correct.

---

## FINDING 18 — TOML parsing is correct (not grep-based)

**Verification:** The implementation uses `toml::Value` throughout, parsed from `content.parse()` at deny_audit.rs:108. This is proper TOML parsing. Ban entries cannot be hidden by TOML formatting tricks (multi-line strings, different quoting styles, unicode escapes, etc.) because the TOML parser normalizes everything.

HOWEVER — the `entry.get("name")` on deny_bans.rs:106 means ban entries MUST use the `name` key. If cargo-deny also supports `crate` as a key (like it does for skip entries in 0.19+ format), then `{ crate = "simd-json" }` would NOT be detected by the ban list parser. The skip entry parser (deny_inventory.rs:13-14) correctly checks for `crate` first, then falls back to `name`. The ban list parser does NOT check for `crate`.

**Fix:** Ban list parsing should also try `entry.get("crate")` as a fallback, matching the skip entry parser pattern.

---

## FINDING 19 — `check_bans_settings` doesn't emit inventory Info for correct values

**Severity: LOW**

When `multiple-versions = "deny"` and `highlight = "all"` are correct, no Info result is emitted. Other checks (unmaintained, yanked, sources) DO emit `.as_inventory()` results for correct values. This is inconsistent and means `--inventory` output won't show these settings were verified.

---

## FINDING 20 — Feature ban check doesn't verify `deny` array contains ONLY "full"

**Severity: LOW**

`check_tokio_feature_ban` checks if `deny` contains "full" using `.any(|x| x == "full")`. It doesn't check if `deny` contains OTHER entries that might conflict or be suspicious. For example, `deny = ["full", "rt-multi-thread"]` would pass (tokio full is banned) but also bans the recommended runtime feature.

---

## Summary Table

| # | Finding | Severity | Exploitable? |
|---|---------|----------|--------------|
| 01 | Profile ignored — library bans never enforced | HIGH | YES — library projects missing all IO bans |
| 02 | No `[graph]` section validation | MEDIUM | YES — feature-gated deps skip checking |
| 03 | No `wildcards`/`allow-wildcard-paths` check | MEDIUM | Minor — affects workspace builds |
| 04 | Advisory ignores don't require reason | HIGH | YES — silent vulnerability suppression |
| 05 | Advisory ignore table format not parsed | MEDIUM | YES — IDs reported as "unknown" |
| 06 | Skip entries only inventoried, never validated | HIGH | YES — unlimited skip = disable bans |
| 07 | Wrong-type advisory values report misleading error | MEDIUM | No — still errors, just bad message |
| 08 | No db-urls/git-fetch-with-cli check | LOW-MEDIUM | Unlikely |
| 09 | allow-registry existence not verified | MEDIUM | YES — missing = possible open registry |
| 10 | confidence-threshold accepts dangerously low/integer values | LOW | Edge case |
| 11 | License allow list contents never validated | MEDIUM | YES — copyleft licenses pass silently |
| 12 | steady-parent has deprecated `unsound` | LOW | Correctly flagged |
| 13 | steady-parent has invalid `unmaintained = "all"` | MEDIUM | Correctly flagged |
| 14 | Tokio feature ban allow list not validated | MEDIUM | YES — dangerous features addable |
| 15 | No check for contradictory license deny+allow | LOW | Edge case |
| 16 | Registry URL checked by substring, not exact match | LOW-MEDIUM | YES — typosquat possible |
| 17 | EXPECTED_BANS count correct (26) | PASS | N/A |
| 18 | Ban list doesn't try `crate` key (only `name`) | MEDIUM | YES — 0.19+ format bypasses detection |
| 19 | Missing inventory emissions for correct ban settings | LOW | No — cosmetic |
| 20 | Feature deny array not validated for conflicts | LOW | Edge case |

## Critical Path to Exploitation

The most dangerous combination: A "library" profile project where an agent:
1. Removes library-IO bans from deny.toml (Finding 01 — validator won't notice)
2. Adds 20 skip entries without reasons (Finding 06 — validator emits only Info)
3. Adds advisory ignores without reasons (Finding 04 — validator emits only Info)
4. Uses `{ crate = "openssl" }` format instead of `{ name = "openssl" }` (Finding 18 — bypasses ban detection)
5. Sets `all-features = false` in `[graph]` (Finding 02 — validator doesn't check)
6. Adds `"AGPL-3.0"` to license allow list (Finding 11 — validator doesn't validate contents)

Result: A project that passes all guardrail3 deny.toml checks but has effectively no security enforcement.
