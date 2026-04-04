# Extract toolchain checks package

## Package

`packages/guardrail3-checks-toolchain/`

Dependencies: `guardrail3-check-types`, `toml` (for parsing rust-toolchain.toml)

Note: NOT using `rust-toolchain-file` external crate. The rules currently
do raw TOML navigation and the file format is simple enough that we don't
need an external parser. We parse with `toml::from_str` internally.

## Public interface

```rust
use guardrail3_check_types::{GrdzCheckResult, GrdzProfile};

pub fn check(input: &GrdzToolchainChecksInput) -> Vec<GrdzCheckResult>

pub struct GrdzToolchainChecksInput {
    /// Raw content of rust-toolchain.toml, if the file exists.
    pub toolchain_content: Option<String>,
    /// Repo-relative path to rust-toolchain.toml.
    pub toolchain_rel_path: Option<String>,
    /// Whether a legacy rust-toolchain (no .toml) file exists.
    pub legacy_toolchain_exists: bool,
    /// Repo-relative path to the legacy file, if it exists.
    pub legacy_toolchain_rel_path: Option<String>,
    /// rust-version from Cargo.toml, pre-extracted by the app.
    pub cargo_rust_version: Option<String>,
    /// Repo-relative path to the Cargo.toml that owns this root.
    pub cargo_rel_path: String,
    /// Whether Cargo.toml failed to parse (blocks MSRV check).
    pub cargo_parse_error: Option<String>,
    /// Workspace profile.
    pub profile: GrdzProfile,
}
```

## Rules (4 total, all move to package)

### RS-TOOLCHAIN-01: rust-toolchain.toml exists
```rust
fn check_exists(
    toolchain_rel_path: Option<&str>,
    rel_dir: &str,
) -> Vec<GrdzCheckResult>
```
- If Some → Info inventory
- If None → Error "missing"

### RS-TOOLCHAIN-02: channel and components
```rust
fn check_channel_and_components(
    parsed: &toml::Value,
    rel_path: &str,
) -> Vec<GrdzCheckResult>
```
- Checks [toolchain].channel is "stable" or pinned stable version
- Checks [toolchain].components contains "clippy" and "rustfmt"
- Errors on nightly, beta, missing channel
- Skips if legacy file exists (rule 04 handles that)

### RS-TOOLCHAIN-03: MSRV consistency
```rust
fn check_msrv_consistency(
    parsed: &toml::Value,
    toolchain_rel_path: &str,
    cargo_rust_version: Option<&str>,
    cargo_rel_path: &str,
    cargo_parse_error: Option<&str>,
) -> Vec<GrdzCheckResult>
```
- Compares pinned toolchain version against Cargo.toml rust-version
- Warns if toolchain < MSRV
- Skips if not pinned, if Cargo.toml missing, or if legacy file exists

### RS-TOOLCHAIN-04: legacy file conflict
```rust
fn check_legacy_file(
    legacy_rel_path: Option<&str>,
    toolchain_rel_path: Option<&str>,
) -> Vec<GrdzCheckResult>
```
- Error if both files exist (legacy overrides modern)
- Warn if only legacy exists (migrate to .toml)

## Top-level dispatch

```rust
pub fn check(input: &GrdzToolchainChecksInput) -> Vec<GrdzCheckResult> {
    let mut results = Vec::new();

    // Rule 01: existence
    results.extend(check_exists(
        input.toolchain_rel_path.as_deref(),
        // derive rel_dir from cargo_rel_path
    ));

    // Rule 04: legacy conflict (check early, rules 02/03 skip if legacy present)
    results.extend(check_legacy_file(
        input.legacy_toolchain_rel_path.as_deref(),
        input.toolchain_rel_path.as_deref(),
    ));

    // Parse toolchain content
    if input.legacy_toolchain_exists {
        return results; // rules 02/03 skip when legacy present
    }
    let Some(content) = &input.toolchain_content else {
        return results;
    };
    let parsed = match toml::from_str::<toml::Value>(content) {
        Ok(v) => v,
        Err(e) => {
            // emit parse error result
            return results;
        }
    };

    // Rule 02: channel + components
    if let Some(rel) = &input.toolchain_rel_path {
        results.extend(check_channel_and_components(&parsed, rel));
    }

    // Rule 03: MSRV consistency
    if let Some(rel) = &input.toolchain_rel_path {
        results.extend(check_msrv_consistency(
            &parsed,
            rel,
            input.cargo_rust_version.as_deref(),
            &input.cargo_rel_path,
            input.cargo_parse_error.as_deref(),
        ));
    }

    results
}
```

## What stays in the app

Nothing. All 4 toolchain rules are content validation. There's no coverage
check ("every root must have rust-toolchain.toml") because the runner
already iterates workspace roots and calls check() per-root. If a root has
no toolchain file, the input has `toolchain_content: None` and rule 01 fires.

## Migration steps

1. Create `packages/guardrail3-checks-toolchain/` with the types and rules
2. Add `guardrail3-check-types` and `toml` as dependencies
3. Implement all 4 rules as internal functions
4. Implement public `check()` that dispatches
5. Write tests (port from existing toolchain test suite)
6. In the app's toolchain runner: replace internal rule calls with
   `guardrail3_checks_toolchain::check(input)` where input is built
   from the current facts
7. Verify all 48 toolchain tests still pass
8. Run guardrails on the new package
