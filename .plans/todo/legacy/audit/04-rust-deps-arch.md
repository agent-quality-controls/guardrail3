# Audit 04: Rust Dependency & Architecture Checks (R45-R52, R55-R57)

## Files Audited

- `apps/guardrail3/src/app/rs/validate/dependency_scan.rs` (R45-R48, R50)
- `apps/guardrail3/src/app/rs/validate/hex_arch_checks.rs` (R-ARCH-01 through R-ARCH-04, replaces R51-R52)
- `apps/guardrail3/src/app/rs/validate/dependency_allowlist.rs` (R-DEPS-01, R-DEPS-02)
- `apps/guardrail3/src/app/rs/validate/workspace_metadata.rs` (R55-R57)
- `apps/guardrail3/src/adapters/outbound/tool_runner.rs` (ToolChecker impl)
- `apps/guardrail3/src/ports/outbound.rs` (ToolChecker trait)

## Check ID Mapping

R51/R52 no longer exist as named checks. They have been replaced by R-ARCH-01 through R-ARCH-04 in `hex_arch_checks.rs`. R50 has been explicitly removed (comment on line 19 of `dependency_scan.rs` says "R50 REMOVED -- banned crate detection is cargo-deny's job"). The audit scope therefore covers: R45-R48 (tool installation), R-ARCH-01/02/03/04 (architecture), R-DEPS-01/02 (dependency allowlists), R55-R57 (workspace metadata).

---

## Findings

### F-04-01: Tool installation checks have no version validation (R45-R48)

**Severity: Medium**
**File:** `dependency_scan.rs:33`, `tool_runner.rs:13-19`

`is_installed()` runs `which <tool>` and checks exit code. This confirms the binary exists on PATH but:

1. **No minimum version check.** A user could have `cargo-deny` 0.9 installed while guardrail3's deny.toml modules assume 0.14+ features (e.g., `[bans.deny]` syntax changes between versions). The check would pass but `cargo-deny check` would fail or silently ignore rules.
2. **No version check for gitleaks either.** gitleaks v7 vs v8 have different config formats (`.gitleaks.toml` vs `.gitleaksrc`).
3. **`which` is not portable.** On some minimal Linux containers, `which` is not installed. `command -v` is the POSIX-portable alternative. However, since this is a Rust `Command::new("which")`, not a shell builtin, it will fail on systems where `which` is a shell function (rare in practice but architecturally wrong).
4. **No check for cargo-deny being the right cargo-deny.** A user could have a different binary named `cargo-deny` on PATH (unlikely but the abstraction allows no verification).

### F-04-02: R50 (banned crates in lockfile) is completely removed with no replacement

**Severity: High**
**File:** `dependency_scan.rs:19`

The comment says "banned crate detection is cargo-deny's job" and refers to deny.toml being configured (R8-R20) and cargo-deny being installed (R45). But this creates a **transitive trust gap**:

1. guardrail3 checks that deny.toml exists and has the right structure (R8-R20).
2. guardrail3 checks that cargo-deny is installed (R45).
3. But guardrail3 does NOT verify that `cargo-deny check` actually passes. It trusts the pre-commit hook to run it.
4. If the pre-commit hook is bypassed (`git commit --no-verify`), or if the hook has a bug, or if the hook is not installed -- banned crates sail through with zero detection.

guardrail3 validates the *configuration* but never validates the *outcome*. For every other config check, that delegation to the tool is fine because guardrail3 is meant to check config. But R50 was the one check that verified actual lockfile state. Removing it means guardrail3 cannot independently verify that no banned crates are present -- it can only verify that the *tool that would check* is configured.

### F-04-03: Dependency direction check (R-ARCH-02) only reads `[dependencies]`, not `[dev-dependencies]` or `[build-dependencies]`

**Severity: High**
**File:** `hex_arch_checks.rs:169`

Line 169: `table.get("dependencies").and_then(|d| d.as_table())` -- only reads `[dependencies]`.

A domain crate could add `adapters` as a `[dev-dependencies]` entry and the check would not flag it. While dev-dependencies are only used in tests, this still means:

1. The domain crate's test code can import adapter types, creating a coupling path that trains developers to think "domain can use adapters."
2. More critically, `[build-dependencies]` are completely ignored. A build script in a domain crate could depend on an adapter crate and execute arbitrary adapter logic at compile time. This is a real architecture violation, not just a test convenience.
3. `[target.'cfg(...)'.dependencies]` sections are also completely ignored. Platform-specific dependencies could violate the architecture on specific targets.

### F-04-04: Dependency direction check can be fooled by crate renaming

**Severity: Medium**
**File:** `hex_arch_checks.rs:173-178`

The check looks up `dep_name` (the key in `[dependencies]`) in the layer map. But Cargo allows renaming:

```toml
[dependencies]
my_domain_thing = { package = "adapters-http", path = "../adapters" }
```

Here `dep_name` is `my_domain_thing`, which won't be found in the layer map. The code then falls through to `extract_path_dep` and `resolve_dep_layer`, which would try to resolve the path. This path-based resolution would work IF the path contains a recognizable layer segment. But if the adapters crate lives at a path like `../http-server/` (no "adapters" segment), the check returns `None` and the violation is silently ignored.

The code never reads the `package` field from the dependency value to get the real crate name.

### F-04-05: `layer_from_path` is fragile and relies on directory naming conventions

**Severity: Medium**
**File:** `hex_arch_checks.rs:47-59`

Layer detection from path uses `contains_segment(dir, "domain")` etc. This means:

1. A crate at `crates/core/` would not be detected as any layer -- silent skip.
2. A crate at `crates/my-domain-utils/` would NOT match because `contains_segment` checks for exact segment equality (`s == segment`), so `my-domain-utils` != `domain`. This is actually correct behavior but may surprise users.
3. A crate at `src/app/domain/` would match as BOTH `app` and `domain`. The function checks `domain` first, so it returns `Domain`. But if someone has `adapters/app/` it would match `adapters` first. The precedence is: domain > ports > app > adapters, which is the right priority for inner layers to win, but this is implicit and undocumented.
4. A crate named `domain` at path `packages/domain/` would be detected as Domain layer even if it's actually a shared library that happens to be named "domain."

### F-04-06: `is_service_internal` (R-ARCH-03) is overly rigid

**Severity: Low**
**File:** `hex_arch_checks.rs:387-392`

```rust
pub fn is_service_internal(path: &str) -> bool {
    let parts: Vec<&str> = path.split('/').collect();
    parts.len() >= 4
        && parts.first().is_some_and(|s| *s == "apps")
        && parts.get(2).is_some_and(|s| *s == "crates")
}
```

This only catches `apps/<name>/crates/<layer>` structure. If a service uses `apps/<name>/src/<layer>/` structure (which R-ARCH-01 explicitly checks for on line 115-116), those paths would NOT be caught by `is_service_internal`. A library could depend on `apps/myservice/src/adapters/some_module` and it would pass.

### F-04-07: R-DEPS-01 doc comment claims it skips dev-dependencies but the mechanism is implicit

**Severity: Low**
**File:** `dependency_allowlist.rs:9-11`

The doc comment says "Skips `[dev-dependencies]`, `[build-dependencies]`" but the code achieves this by only reading `table.get("dependencies")` (line 28). While functionally correct, this means:

1. A library crate with `allowed_deps = ["serde"]` could add `tokio` as a dev-dependency and no check would flag it. For library profile crates, this defeats the purpose -- the library's test code now has access to I/O crates, and test patterns could leak into production code during refactoring.
2. `[build-dependencies]` are completely uncontrolled by the allowlist. A library crate's build script could pull in any dependency.

### F-04-08: `is_workspace_path_dep` also skips `workspace = true` dependencies

**Severity: Medium**
**File:** `dependency_allowlist.rs:58-70`

`workspace = true` dependencies are treated as "workspace path dependencies" and skipped from the allowlist check. But `workspace = true` means the dependency version is inherited from the workspace `Cargo.toml` -- it does NOT mean the dependency is a local/internal crate. It could be an external crate like `tokio` declared in `[workspace.dependencies]`.

Example: if the workspace Cargo.toml has:
```toml
[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
```

And a library crate has:
```toml
[dependencies]
tokio = { workspace = true }
```

The allowlist check skips `tokio` because `workspace = true` is treated as a "workspace path dependency." This is a **complete bypass of the allowlist for any dependency declared in workspace.dependencies**.

### F-04-09: R55-R57 are all Info severity -- they report but never enforce

**Severity: Medium**
**File:** `workspace_metadata.rs`

All three checks (R55, R56, R57) emit `Severity::Info` results via `.as_inventory()`. They are purely informational:

1. **R55** reports edition and rust-version if present but does NOT enforce that they exist. A workspace with no `edition` field would produce zero output -- no warning, no error. The 2021 edition is the implicit default but newer features require `edition = "2024"`. Not enforcing this means projects could silently use an older edition.
2. **R56** reports `publish` status but does NOT enforce `publish = false` for internal crates. An internal service crate without `publish = false` could be accidentally published to crates.io.
3. **R57** reports release profile settings but does NOT enforce any minimum optimization settings. A project could ship with `[profile.release]` having `opt-level = 0` and this check would happily report it as informational.
4. If `Cargo.toml` fails to parse (line 23: `Err(_) => return`), the function silently returns with no results. A malformed Cargo.toml produces no warnings at all.

### F-04-10: Silent failures throughout -- parse errors produce zero output

**Severity: Medium**
**Files:** `workspace_metadata.rs:21-24`, `hex_arch_checks.rs:166-168`, `dependency_allowlist.rs:23-26`

Multiple functions silently return empty results when TOML parsing fails:
- `workspace_metadata.rs:23`: `Err(_) => return`
- `hex_arch_checks.rs:166-167`: `let Ok(table) = content.parse::<toml::Value>() else { continue; }`
- `dependency_allowlist.rs:24-25`: `Err(_) => return`

If a Cargo.toml has a syntax error, ALL dependency direction, allowlist, and metadata checks silently pass. A single typo in Cargo.toml becomes an invisibility cloak for architecture violations.

### F-04-11: R-ARCH-01 only checks for `domain` and `adapters` layers, not `ports` or `app`

**Severity: Medium**
**File:** `hex_arch_checks.rs:113`

```rust
for sub in &["domain", "adapters"] {
```

The hex arch structure check only verifies that `domain` and `adapters` directories exist. It does not check for `ports` or `app`. A service could have domain and adapters but no ports layer (meaning domain types directly reference adapter implementations) and the check would pass.

### F-04-12: R-ARCH-04 cargo_path construction assumes crate name matches directory name

**Severity: Medium**
**File:** `mod.rs:242`

```rust
let cargo_path = workspace_root.join(crate_name).join("Cargo.toml");
```

In the architecture checks orchestrator, the allowlist check constructs the Cargo.toml path using the crate config key (which is the config key name from `guardrail3.toml`, e.g., `[rust.apps.my-crate]`). If the config key doesn't match the actual directory path (e.g., config key is `my-crate` but directory is `apps/my-service/crates/my-crate`), the path will be wrong and `fs.read_file` will return `None`, silently skipping the allowlist check.

### F-04-13: `normalize_path` does not handle absolute paths or leading `/`

**Severity: Low**
**File:** `hex_arch_checks.rs:228-240`

If `rel` starts with `/` (absolute path in a path dependency), the function would add an empty string segment and then the absolute path segments onto the base path, producing nonsense like `apps/my-crate//usr/local/lib/some-crate`. In practice, absolute paths in Cargo.toml path dependencies are rare but not impossible.

### F-04-14: No check for `[patch]` or `[replace]` sections

**Severity: Medium**
**Files:** `hex_arch_checks.rs`, `dependency_allowlist.rs`

Cargo supports `[patch]` and `[replace]` sections in workspace Cargo.toml that can override any dependency with a different version or local path. Neither the architecture checks nor the allowlist checks examine these sections. A project could use `[patch.crates-io]` to replace a crate with a local fork that has completely different dependencies, and all guardrail3 checks would still see the original declared dependency.

---

## Summary

| ID | Severity | Summary |
|----|----------|---------|
| F-04-01 | Medium | No tool version validation (R45-R48) |
| F-04-02 | High | R50 removed, no independent banned-crate verification |
| F-04-03 | High | dev-dependencies and build-dependencies bypass arch flow check |
| F-04-04 | Medium | Crate renaming (`package = "..."`) bypasses direction check |
| F-04-05 | Medium | Path-based layer detection is fragile and convention-dependent |
| F-04-06 | Low | `is_service_internal` misses `src/<layer>/` structure |
| F-04-07 | Low | Allowlist implicitly skips dev/build deps, allowing uncontrolled test deps |
| F-04-08 | Medium | `workspace = true` dependencies bypass allowlist entirely |
| F-04-09 | Medium | R55-R57 are informational only, enforce nothing |
| F-04-10 | Medium | TOML parse errors silently skip all checks on that file |
| F-04-11 | Medium | R-ARCH-01 only checks domain+adapters, not ports+app |
| F-04-12 | Medium | Allowlist cargo_path assumes config key = directory path |
| F-04-13 | Low | `normalize_path` doesn't handle absolute paths |
| F-04-14 | Medium | `[patch]`/`[replace]` sections completely unchecked |
