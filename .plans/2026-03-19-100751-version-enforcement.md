# Version enforcement checks

**Date:** 2026-03-19 10:07
**Task:** Add validate checks that flag outdated tool versions and version inconsistencies across package.json/Cargo.toml files.

## Minimum versions (enforced as warnings)

### Rust
| Tool | Min version | Where to check | Why this version |
|---|---|---|---|
| Rust toolchain | 1.94.0 | rust-toolchain.toml `channel` or `[toolchain] channel` | Latest stable, edition 2024 support |
| Rust edition | 2024 | Each Cargo.toml `[package] edition` or `[workspace.package] edition` | Current edition |

### TypeScript ecosystem
| Tool | Min version | Where to check | Why this version |
|---|---|---|---|
| ESLint | 10.0.0 | package.json `devDependencies.eslint` | File-based config lookup default, eslintrc removed |
| TypeScript | 5.9.0 | package.json `devDependencies.typescript` | strictInference in --strict |
| pnpm | 10.0.0 | package.json `packageManager` field | Install scripts disabled by default, security hardening |
| Stylelint | 17.0.0 | package.json `devDependencies.stylelint` | ESM-only |
| typescript-eslint | 8.57.0 | package.json `devDependencies.typescript-eslint` | checkUnknown on no-base-to-string |
| eslint-plugin-sonarjs | 4.0.0 | package.json `devDependencies.eslint-plugin-sonarjs` | Full rewrite, all SonarJS rules |
| eslint-plugin-regexp | 3.0.0 | package.json `devDependencies.eslint-plugin-regexp` | New major |
| Node.js | 20.19.0 | package.json `engines.node` | ESLint 10 requirement |

## What to check

### Per package.json (crawler finds all of them)
For each package.json found by the crawler:
1. Parse devDependencies and dependencies
2. For each tool in our minimum list:
   - If present: parse version (strip ^, ~, >= prefixes), compare to minimum
   - If below minimum: WARN with "package.json at {path}: eslint {found} is below minimum {min}"
   - If not present: might be fine (not every package needs every tool — only root needs eslint)
3. Check `packageManager` field for pnpm version
4. Check `engines.node` for Node.js version

### Cross-package.json consistency
After checking all package.json files individually:
1. For each tool, collect all versions found across all package.json files
2. If different versions found: WARN "version inconsistency: eslint 10.0.3 in root but 9.14.0 in apps/landing"
3. The ROOT package.json version is the reference — app/package versions should match or be absent (inherited from workspace)

### Per Cargo.toml
For each Cargo.toml found:
1. Check `edition` — warn if not "2024"
2. Check `rust-version` — warn if below "1.94"

### rust-toolchain.toml
If found: check `channel` is "stable" and version is current.
If NOT found: warn (missing rust-toolchain.toml means developers use whatever version they have).

## Implementation

### Data flow
```
Crawler → all package.json paths, all Cargo.toml paths, rust-toolchain.toml path
    ↓
Version checker → parse each file, extract versions, compare to minimums
    ↓
CheckResult[] → version warnings + inconsistency warnings
```

### New check IDs needed

Rust:
- R-VER-01: Rust edition below minimum
- R-VER-02: rust-version below minimum
- R-VER-03: rust-toolchain.toml missing or outdated

TypeScript:
- T-VER-01: ESLint version below minimum
- T-VER-02: TypeScript version below minimum
- T-VER-03: pnpm version below minimum
- T-VER-04: Stylelint version below minimum
- T-VER-05: typescript-eslint version below minimum
- T-VER-06: eslint-plugin version below minimum (sonarjs, regexp, unicorn)
- T-VER-07: Node.js engines below minimum

Cross-cutting:
- X-VER-01: Version inconsistency across package.json files

### Where in the code

New module: `src/app/version_checks.rs`
- Takes CrawlResult (all file paths)
- Reads and parses each relevant file
- Returns Vec<CheckResult>
- Called from both `rs validate` and `ts validate`

### Version parsing

npm versions: `"^10.0.3"` → strip prefix (`^`, `~`, `>=`, `=`), parse `major.minor.patch`
Rust editions: just string compare `"2024"` vs `"2021"`
pnpm packageManager: `"pnpm@10.32.0"` → parse after `@`
engines.node: `">=20.19.0"` → parse after `>=`

### Edge cases

1. **Version ranges:** `">=9.0.0 <11.0.0"` — take the lower bound
2. **Star versions:** `"*"` — skip, can't determine
3. **Missing version prefix:** `"10.0.3"` (exact) — use as-is
4. **workspace: protocol:** `"workspace:*"` — resolve from root package.json
5. **Catalog references (pnpm):** `"catalog:"` — resolve from pnpm-workspace.yaml catalogs
6. **Version not in devDeps but in deps:** Check both
7. **Multiple package.json with same dep at different versions:** The inconsistency check
8. **Monorepo root has the dep, apps don't:** Fine — apps inherit from workspace. But if an app DOES declare the dep, it should match root.

### What NOT to check

- Don't enforce exact versions (too strict — `10.0.1` is fine if min is `10.0.0`)
- Don't fail on missing optional tools (not every project uses stylelint)
- Don't check transitive dependency versions (only direct devDependencies)
- Don't enforce for Cargo.lock versions (only Cargo.toml declarations)

## Update canonical templates (separate task)

After version checks are implemented, also update:
- rust-toolchain.toml canonical: channel = "1.94.0" → actually should stay "stable" (latest stable auto-resolves)
- rustfmt.toml canonical: add `imports_granularity = "Crate"`, `style_edition = "2024"`
- ESLint engine: target v10 patterns
- tsconfig.base.json: add any new TS 5.9 strict flags
- .npmrc canonical: add pnpm 10 security settings if new ones exist

These are generate changes, not validate changes. Separate from version enforcement.
