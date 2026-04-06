# Extract toolchain content checks package

## Scope

Content checks only. Filetree checks (existence, legacy conflict) stay
in the app's family runtime.

## Package

`packages/guardrail3-checks-toolchain/`

Dependencies: `guardrail3-check-types`, `cargo_toml`, `toml`

## Public interface

```rust
use guardrail3_check_types::{G3CheckResult, G3Profile};
use cargo_toml::Manifest;

/// Validate rust-toolchain.toml contents.
/// The file is known to exist — the app handles missing file errors.
pub fn check(input: &G3ToolchainChecksInput) -> Vec<G3CheckResult>

pub struct G3ToolchainChecksInput {
    /// Parsed rust-toolchain.toml. The app parsed it with toml crate.
    /// Package reads [toolchain].channel, [toolchain].components.
    pub toolchain_parsed: toml::Value,
    /// Repo-relative path for error reporting.
    pub toolchain_rel_path: String,
    /// Parsed Cargo.toml manifest. Package reads rust-version.
    pub cargo_manifest: Manifest,
    /// Repo-relative path to Cargo.toml for error reporting.
    pub cargo_rel_path: String,
    /// Workspace profile.
    pub profile: G3Profile,
}
```

## Rules that move (2)

### RS-TOOLCHAIN-CONFIG-01: channel and components
```rust
fn check_channel_and_components(
    parsed: &toml::Value,
    rel_path: &str,
) -> Vec<G3CheckResult>
```
- Validates [toolchain].channel is stable or pinned stable
- Validates [toolchain].components contains clippy + rustfmt
- Errors on nightly, beta, unsupported

### RS-TOOLCHAIN-CONFIG-02: MSRV consistency
```rust
fn check_msrv_consistency(
    parsed: &toml::Value,
    toolchain_rel_path: &str,
    cargo_manifest: &Manifest,
    cargo_rel_path: &str,
) -> Vec<G3CheckResult>
```
- Extracts rust-version from Manifest
- Compares pinned toolchain version against MSRV
- Warns if toolchain < MSRV

## Rules that stay in app (2)

### RS-TOOLCHAIN-01: existence
Stays in app. The app checks whether rust-toolchain.toml was found at
this root. If not, emits "missing" error. If yes, calls the content
checks package.

### RS-TOOLCHAIN-04: legacy file conflict
Stays in app. Checks whether both rust-toolchain and rust-toolchain.toml
exist. This is filetree/placement, not content validation.

## Top-level dispatch in package

```rust
pub fn check(input: &G3ToolchainChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    results.extend(check_channel_and_components(
        &input.toolchain_parsed,
        &input.toolchain_rel_path,
    ));
    results.extend(check_msrv_consistency(
        &input.toolchain_parsed,
        &input.toolchain_rel_path,
        &input.cargo_manifest,
        &input.cargo_rel_path,
    ));
    results
}
```

## App-side migration

The app's toolchain runner currently calls all 4 rules via the internal
family crate. After extraction:

```rust
fn run_toolchain(ctx: &RustRunContext<'_>) -> Vec<CheckResult> {
    // ... existing discovery + facts collection ...
    
    for input in all_from_facts(&facts) {
        // Rules 01 + 04 stay inline
        rs_toolchain_01_exists::check(&input, &mut results);
        rs_toolchain_04_legacy_file::check(&input, &mut results);
        
        // Rules 02 + 03 delegate to package
        if let (Some(parsed), Some(rel_path)) = (&input.parsed, input.toolchain_toml_rel) {
            let checks_input = G3ToolchainChecksInput {
                toolchain_parsed: parsed.clone(),
                toolchain_rel_path: rel_path.to_owned(),
                cargo_manifest: /* parse from content */,
                cargo_rel_path: input.cargo_rel_path.to_owned(),
                profile: /* resolve from guardrail3.toml */,
            };
            let package_results = guardrail3_checks_toolchain::check(&checks_input);
            // convert G3CheckResult → CheckResult and extend
            results.extend(convert(package_results));
        }
    }
    results
}
```

Note: there's a G3CheckResult → CheckResult conversion needed since
the app uses its own CheckResult type. Either:
- Make check-types' G3CheckResult THE CheckResult (big refactor)
- Add a conversion function (pragmatic for migration)

## Migration steps

1. Create package with types and 2 rules
2. Write tests for rules 02 + 03 using the new input type
3. Add package as dependency to the app's toolchain family
4. Update runner to call package for rules 02 + 03
5. Remove rules 02 + 03 from the internal family crate
6. Verify all 48 tests pass
7. Run guardrails on the new package
