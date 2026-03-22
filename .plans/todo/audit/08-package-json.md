# Adversarial Audit: package.json Validation (T15-T18, T55-T58, T-PLUG-*, T-TOOL-*, T-PKG-*)

**Files audited:**
- `apps/guardrail3/src/app/ts/validate/package_check.rs`
- `apps/guardrail3/src/app/ts/validate/package_deps.rs`
- `apps/guardrail3/src/app/ts/validate/tool_config_checks.rs`

**Reference package.json files:**
- `/Users/tartakovsky/Projects/websmasher/websmasher/package.json`
- `/Users/tartakovsky/Projects/ts-rust-railway/package.json`

---

## Findings

### F-PKG-01: T15 does NOT validate override version values — wildcards pass silently

**Check:** T15 (pnpm.overrides)
**Severity:** GAP
**Location:** `package_check.rs:62-63`

T15 checks `ov_obj.contains_key("zod")` — presence only. Someone can set `"zod": "*"` or `"zod": ""` or `"zod": "latest"` and the check passes. A wildcard override defeats the purpose of pinning (prevents version deduplication, allows arbitrary transitive versions).

**What's needed:** Validate that the override value is a specific semver range (not `*`, `latest`, empty string, or unbounded ranges like `>=0`).

---

### F-PKG-02: T17 does NOT check peerDependencies or optionalDependencies

**Check:** T17 (banned deps)
**Severity:** GAP
**Location:** `package_check.rs:156`

The loop iterates only `["dependencies", "devDependencies"]`. A banned package in `peerDependencies` or `optionalDependencies` is invisible to T17. While less common, a package like `lodash` in `peerDependencies` would still get installed and be importable at runtime.

**What's needed:** Add `"peerDependencies"` and `"optionalDependencies"` to the section list.

---

### F-PKG-03: T17 banned list does not catch scoped variants or forks

**Check:** T17 (banned deps)
**Severity:** GAP
**Location:** `package_check.rs:133-153`

The banned list uses exact name matching. Bypass vectors:
- `@types/lodash` — the type package implies lodash is used somewhere (or encourages its adoption)
- `lodash-es`, `lodash.merge`, `lodash.get` — per-method lodash packages are NOT caught
- `moment-timezone` — extends moment, equally banned-worthy
- `axios-retry`, `axios-cache-interceptor` — imply axios is used
- `node-fetch` is banned but `whatwg-fetch` (another polyfill) is not
- `cross-env` — not banned but considered deprecated/unnecessary on modern Node

The `banned_prefixes` mechanism exists (`"embla-carousel"`) but is only used for embla. No prefix bans for `lodash.`, `moment-`, `axios-`.

**What's needed:** Add prefix bans for `lodash.`, `lodash-`, `moment-`, `axios-`. Consider banning `@types/lodash`, `@types/moment`.

---

### F-PKG-04: T55 preinstall check uses substring match — bypassable

**Check:** T55 (preinstall pnpm enforcement)
**Severity:** GAP
**Location:** `package_check.rs:219`

The check is `script.contains("only-allow pnpm")`. This passes for:
- `"preinstall": "npx only-allow pnpm"` (correct)
- `"preinstall": "echo only-allow pnpm"` (does nothing)
- `"preinstall": "# only-allow pnpm"` (comment, does nothing)
- `"preinstall": "npx only-allow pnpm || true"` (silences failure)
- `"preinstall": "true # only-allow pnpm"` (no-op with comment)

Any of these bypass the enforcement while passing the check.

**What's needed:** Validate that the script starts with `npx only-allow pnpm` or `npx -y only-allow pnpm` and does NOT contain `|| true`, `; true`, or trailing commands that could suppress the exit code.

---

### F-PKG-05: T18 does NOT validate packageManager format — npm/yarn pass

**Check:** T18 (packageManager field)
**Severity:** GAP
**Location:** `package_check.rs:182-210`

T18 only checks `json.get("packageManager").is_some()`. These all pass:
- `"packageManager": "npm@10.0.0"` — wrong package manager entirely
- `"packageManager": "yarn@4.0.0"` — wrong package manager
- `"packageManager": "pnpm"` — no version pinned (corepack won't enforce version)
- `"packageManager": ""` — empty string
- `"packageManager": 42` — not even a string (`.is_some()` passes for any JSON type)
- `"packageManager": "pnpm@latest"` — not a pinned version

**What's needed:** Validate the value is a string matching `pnpm@<semver>` format (e.g., regex `^pnpm@\d+\.\d+\.\d+`).

---

### F-PKG-06: T57 engines does NOT validate version range reasonableness

**Check:** T57 (engines field)
**Severity:** GAP
**Location:** `package_check.rs:347-375`

T57 only checks `json.get("engines").is_some()`. These pass:
- `"engines": { "node": ">=0" }` — allows any node version including ancient ones
- `"engines": { "node": "*" }` — no constraint at all
- `"engines": {}` — empty object, no constraints
- `"engines": { "bun": ">=1" }` — wrong runtime, no node constraint
- `"engines": 42` — not even an object

The check doesn't verify that `engines.node` exists, nor that its version range is reasonable (e.g., `>=18` or `>=20`).

**What's needed:** Validate that `engines` is an object, contains a `node` key, and the node version constraint is `>=18` or higher.

---

### F-PKG-07: T-PKG-01 correctly handles string "true" — NOT a gap

**Check:** T-PKG-01 (private field)
**Location:** `package_check.rs:23-26`

The check uses `as_bool()` which returns `None` for `"true"` (string). So `"private": "true"` correctly fails. This is properly handled.

---

### F-PKG-08: T-PLUG checks only verify presence, not version constraints

**Check:** T-PLUG-01 through T-PLUG-19
**Severity:** GAP
**Location:** `package_deps.rs:11-45`

`check_dev_dep` only checks `d.contains_key(pkg)`. The version value is never inspected. A project could have `"eslint": "^7.0.0"` (ancient, incompatible with flat config) and T-PLUG-12 would pass. Same for `"typescript": "^4.0.0"` (missing modern features).

**What's needed:** For critical packages (eslint, typescript, typescript-eslint), validate minimum version constraints. At minimum: eslint >=9 (flat config), typescript >=5.

---

### F-PKG-09: T-PLUG checks ONLY check devDependencies — packages in dependencies are invisible

**Check:** T-PLUG-01 through T-PLUG-19
**Severity:** GAP
**Location:** `package_deps.rs:64`

`check_dev_dep` only reads `json.get("devDependencies")`. If someone puts `eslint` or `typescript` in `dependencies` instead of `devDependencies`, the check fires as "missing" (false positive). More importantly, if a plugin is in `dependencies`, the check doesn't flag this as wrong — it just says "missing from devDependencies" while the package IS installed. This could confuse users.

**What's needed:** Also check `dependencies` — if found there, emit a different error: "X found in dependencies but should be in devDependencies."

---

### F-PKG-10: T-TOOL-08/09/10 script checks verify existence only, not content

**Check:** T-TOOL-08, T-TOOL-09, T-TOOL-10
**Severity:** GAP
**Location:** `tool_config_checks.rs:132-167`

`check_script` only checks `s.contains_key(script_name)` — the script value is never inspected. A project can have:
- `"type-coverage": "echo skipped"` — T-TOOL-08 passes
- `"license-check": "true"` — T-TOOL-09 passes
- `"audit": "echo no"` — T-TOOL-10 passes

The `example` parameter passed to `check_script` is only used in the error message for missing scripts — it's never validated against the actual script content.

**What's needed:** At minimum, validate that `type-coverage` script contains `type-coverage`, `license-check` contains `license-checker`, and `audit` contains `pnpm audit` or `npm audit`.

---

### F-PKG-11: T58 onlyBuiltDependencies has no error path — missing config is silently accepted

**Check:** T58 (onlyBuiltDependencies)
**Severity:** GAP
**Location:** `package_check.rs:411-427`

T58 only emits an `Info` result when `onlyBuiltDependencies` IS present. When it's absent, nothing is emitted — no error, no warning. This means a project without supply chain protection via `onlyBuiltDependencies` gets a clean bill of health. The `websmasher/package.json` and `ts-rust-railway/package.json` both have this configured, proving it's expected.

**What's needed:** Add an Error or Warn result when `pnpm.onlyBuiltDependencies` is missing.

---

### F-PKG-12: No workspace package validation

**Check:** All package.json checks
**Severity:** GAP
**Location:** `package_check.rs` (entire file)

`check_package_json` is called once for the root `path.join("package.json")`. There is no iteration over workspace packages (e.g., `apps/web/package.json`, `packages/types/package.json`). Workspace packages can have their own banned dependencies, missing private fields, wrong engines, etc.

The `websmasher/package.json` shows this is a pnpm workspace (has `pnpm.overrides`), but the individual workspace member package.json files are never validated by these checks.

**What's needed:** Either document that these checks are root-only by design, or iterate over `pnpm-workspace.yaml` to find and validate workspace member package.json files.

---

### F-PKG-13: No test coverage for any package.json check

**Check:** All checks in all three files
**Severity:** GAP
**Location:** `package_check.rs`, `package_deps.rs`, `tool_config_checks.rs`

None of the three files have a `#[cfg(test)] mod tests` block. Zero unit tests. Every bypass vector listed above is untested. Per the project's own CLAUDE.md: "Every new check has adversarial test fixtures that try to break it."

**What's needed:** Tests for each check, especially adversarial cases (wildcard overrides, wrong packageManager, bypassed preinstall, banned deps in peerDependencies, etc.).

---

### F-PKG-14: package.json is read and parsed independently 3 times

**Check:** Architecture
**Severity:** WASTE (not a correctness gap)
**Location:** `package_check.rs:8-20`, `package_deps.rs:56-62`, `tool_config_checks.rs:29-31`

Each function independently reads and parses `package.json`. In a single validation run, the same file is read from disk 3 times and parsed 3 times. While not a correctness issue, this is wasteful and creates a theoretical TOCTOU window where the file could change between reads (though practically unlikely).

**What's needed:** Parse once, pass the `serde_json::Value` to all check functions.

---

### F-PKG-15: Silent return on invalid JSON — no error reported

**Check:** All checks
**Severity:** GAP
**Location:** `package_check.rs:17-20`, `package_deps.rs:60-62`, `tool_config_checks.rs:31`

If `package.json` exists but contains invalid JSON, all three functions silently return with no results. The user gets zero errors AND zero inventory — a completely clean report for a broken file. This hides a broken `package.json` from the validation report.

**What's needed:** Emit an Error result when `package.json` exists but fails to parse.

---

### F-PKG-16: Missing banned packages that SHOULD be on the list

**Check:** T17 (banned deps)
**Severity:** GAP
**Location:** `package_check.rs:133-153`

Notable omissions from the banned list:
- `left-pad` — infamous supply chain incident, no reason to allow
- `colors` — sabotaged by maintainer (v1.4.1 infinite loop)
- `faker` — sabotaged by maintainer
- `event-stream` — historic supply chain attack vector
- `ua-parser-js` — had malicious versions published
- `coa` — had malicious versions published
- `rc` — had malicious versions published
- `rimraf` — unnecessary on modern Node (fs.rm with recursive)
- `mkdirp` — unnecessary on modern Node (fs.mkdir with recursive)
- `glob` — unnecessary on modern Node (fs.glob in Node 22+)
- `chalk` — unnecessary on modern Node (util.styleText in Node 22+)

Some of these are supply-chain-attacked packages that should never appear in a dependency tree.

**What's needed:** Review and expand the banned list, distinguishing between "use alternative" bans and "known malicious" bans.

---

### F-PKG-17: T56 prepare script check verifies existence only — husky bypass possible

**Check:** T56 (prepare script)
**Severity:** GAP
**Location:** `package_check.rs:249-280`

T56 checks `prepare.is_some()` — any value passes. A project can have `"prepare": "echo no hooks"` and T56 reports success. The prepare script should contain either `husky` or `git config core.hooksPath` to actually set up git hooks.

**What's needed:** Validate the prepare script content contains a hook setup command.

---

### F-PKG-18: T-PKG-02 and T-PKG-03 verify script existence only — empty/noop scripts pass

**Check:** T-PKG-02 (lint script), T-PKG-03 (typecheck script)
**Severity:** GAP
**Location:** `package_check.rs:282-344`

Same pattern as other script checks: `scripts.contains_key("lint")` passes for `"lint": "true"` or `"lint": "echo done"`. The lint script should contain `eslint` and the typecheck script should contain `tsc`.

**What's needed:** Validate script content contains the expected tool name.

---

## Summary

| ID | Check | Gap | Impact |
|----|-------|-----|--------|
| F-PKG-01 | T15 | Override values not validated | Wildcard overrides defeat deduplication |
| F-PKG-02 | T17 | peerDeps/optionalDeps not checked | Banned packages hide in other sections |
| F-PKG-03 | T17 | No scoped/variant ban matching | lodash.merge, moment-timezone bypass |
| F-PKG-04 | T55 | Substring match on preinstall | `echo only-allow pnpm` bypasses |
| F-PKG-05 | T18 | No format validation | npm/yarn/empty string pass |
| F-PKG-06 | T57 | No version range validation | `>=0` or empty engines pass |
| F-PKG-07 | T-PKG-01 | None (correctly handles string) | N/A |
| F-PKG-08 | T-PLUG-* | No version constraint check | Ancient eslint/typescript pass |
| F-PKG-09 | T-PLUG-* | Only checks devDependencies | Misplaced packages invisible |
| F-PKG-10 | T-TOOL-08/09/10 | Script content not validated | `echo skipped` passes |
| F-PKG-11 | T58 | No error when missing | Missing supply chain protection silent |
| F-PKG-12 | All | No workspace member validation | Sub-packages unchecked |
| F-PKG-13 | All | Zero unit tests | All bypasses untested |
| F-PKG-14 | All | Triple file read/parse | TOCTOU window, wasted I/O |
| F-PKG-15 | All | Silent on invalid JSON | Broken package.json invisible |
| F-PKG-16 | T17 | Missing banned packages | Supply-chain-attacked packages allowed |
| F-PKG-17 | T56 | Prepare content not validated | `echo no hooks` passes |
| F-PKG-18 | T-PKG-02/03 | Script content not validated | `echo done` passes |

**Total: 17 gaps found (1 confirmed non-gap).**
