# guardrail3 — 4 guardrail domains, parallel implementation

**Date:** 2026-03-15 18:31
**Task:** Add domain flags to validate command and implement 4 guardrail domains in parallel.

## Goal

```
guardrail3 validate                    # everything
guardrail3 validate --code             # clippy, deny, cargo lints, structural health, centralized fs
guardrail3 validate --architecture     # hex arch deps, import boundaries, Garde boundary validation
guardrail3 validate --release          # publish readiness, semver, changelog, CI workflows
guardrail3 validate --tests            # mutation testing config, test quality
guardrail3 validate --code --release   # combine any subset
guardrail3 rs validate --code          # Rust code quality only
guardrail3 ts validate --tests         # TS test quality only
```

No flags = run all domains. Flags are additive filters.

## Current state

Everything currently in `rs validate` and `ts validate` falls under `--code` and `--architecture`. The domain split is:

**--code (existing, just needs tagging):**
- R1-R29: Config file completeness (clippy, deny, rustfmt, toolchain, cargo lints)
- R30-R44: Source scan (allows, garde skip, file length, use count, unsafe, todo, unwrap)
- R45-R50: Tool installation + Cargo.lock audit
- R55-R57: Workspace metadata
- R58: Direct std::fs detection
- T1-T22: ESLint, tsconfig, npmrc, package.json, jscpd config
- T23-T35: TS source scan (eslint-disable, ts-ignore, process.env, any, file length)
- T40-T83: ESLint rule presence checks
- H1-H12: Hook existence + configuration
- D1-D5: Deployment checks

**--architecture (existing, just needs tagging):**
- R51-R53: Dependency direction, graph inventory, unsafe_code=forbid
- T36-T39: ESLint import boundaries (zones, direction, entry-point, per-zone bans)

**--release (NEW):**
- R-PUB-01 through R-PUB-10: Per-crate publish metadata
- R-REL-01 through R-REL-07: Repo-level release config
- R-BIN-01 through R-BIN-03: Binary distribution

**--tests (NEW):**
- R-TEST-01 through R-TEST-0N: Rust mutation testing config
- T-TEST-01 through T-TEST-0N: TypeScript mutation testing config

## Step 1: Add domain flags to CLI (shared work, do first)

**File:** src/cli.rs

Add domain flags to ValidateArgs:
```rust
#[derive(Parser, Debug, Clone)]
pub struct ValidateArgs {
    // existing flags...

    /// Only run code quality checks
    #[arg(long)]
    pub code: bool,

    /// Only run architecture checks
    #[arg(long)]
    pub architecture: bool,

    /// Only run release readiness checks
    #[arg(long)]
    pub release: bool,

    /// Only run test quality checks
    #[arg(long)]
    pub tests: bool,
}
```

**File:** src/commands/validate.rs, src/main.rs

When no flags set, run all domains. When flags set, only run selected domains.

Logic:
```rust
let run_all = !args.code && !args.architecture && !args.release && !args.tests;
let run_code = run_all || args.code;
let run_arch = run_all || args.architecture;
let run_release = run_all || args.release;
let run_tests = run_all || args.tests;
```

Pass these booleans through to the rs/ts/hooks validate orchestrators so they can skip sections.

**File:** src/rs/validate/mod.rs

Add a `ValidateDomains` struct:
```rust
pub struct ValidateDomains {
    pub code: bool,
    pub architecture: bool,
    pub release: bool,
    pub tests: bool,
}
```

The orchestrator checks which domains are active and only calls the relevant check modules.

**Estimated effort:** Small. Just CLI flags + conditional orchestration. Do this first so all 4 domains can develop against the interface.

## Step 2: Tag existing checks with domains (shared work, do second)

Existing checks need domain tags so the filter knows what to include/exclude.

**Approach:** Each check module function already returns `Vec<CheckResult>`. The orchestrator decides which functions to call based on active domains. No changes to check functions themselves — just to the orchestrator's calling logic.

In `src/rs/validate/mod.rs`:
```rust
if domains.code {
    results.extend(config_files::check(workspace_root));
    results.extend(clippy_coverage::check(workspace_root, profile));
    results.extend(deny_audit::check(workspace_root));
    // ... all existing code checks
}
if domains.architecture {
    results.extend(source_scan::check_dependency_directions(workspace_root, &project));
    results.extend(source_scan::check_unsafe_code_forbid(workspace_root));
}
if domains.release {
    results.extend(release_checks::check(workspace_root, &project));
}
if domains.tests {
    results.extend(test_checks::check(workspace_root));
}
```

Same pattern for ts/validate/mod.rs.

## Step 3: Implement 4 domains in parallel

### Domain A: Garde boundary validation (--architecture)

**New files:**
- `src/modules/clippy_garde.rs` — Garde-specific clippy ban entries
- `src/rs/validate/garde_checks.rs` — validate Garde adoption

**New clippy method bans (add to modules/clippy.rs):**
```
serde_json::from_str
serde_json::from_slice
serde_json::from_value
serde_json::from_reader
reqwest::Response::json
toml::from_str
serde_yaml::from_str
serde_yaml::from_reader
```

**New clippy type bans:**
```
axum::extract::Json
axum::Json
axum::extract::Query
axum::extract::Form
```

**New validate checks:**
- R-GARDE-01: garde in workspace dependencies (Info if missing — not all projects use it)
- R-GARDE-02: serde_json::from_* bans present in clippy.toml (Warn if missing)
- R-GARDE-03: axum::Json ban present in clippy.toml (Warn if missing)
- R-GARDE-04: reqwest::Response::json ban present (Warn if missing)
- R-GARDE-05: #[derive(garde::Validate)] on boundary structs (Info — inventory of structs with/without)

**Profile integration:**
- Add `clippy_garde_methods()` and `clippy_garde_types()` to module system
- Service and monorepo profiles include Garde bans
- Library profile includes Garde bans (dormant since axum is banned entirely, but belt-and-suspenders)

**Pre-commit hook:** Add #[garde(skip)] reason check (already done — R34/R35)

**Touches existing code:**
- `src/modules/clippy.rs` — add new method/type ban modules
- `src/rs/validate/clippy_coverage.rs` — add Garde bans to expected lists (or make them optional based on whether garde is a dependency)
- `src/rs/validate/mod.rs` — wire in garde_checks under architecture domain

**Dependencies:** None. Can develop independently.

---

### Domain B: Release readiness (--release)

**New files:**
- `src/rs/validate/release_checks.rs` — all publish/release checks
- `src/ts/validate/release_checks.rs` — npm publish checks (if applicable)

**Rust release checks:**

| ID | Check | Severity | What |
|---|---|---|---|
| R-PUB-01 | description present | Error | Cargo.toml [package].description |
| R-PUB-02 | license present | Error | Cargo.toml [package].license or license-file |
| R-PUB-03 | repository present | Error | Cargo.toml [package].repository |
| R-PUB-04 | readme present | Warn | Cargo.toml [package].readme |
| R-PUB-05 | version is valid semver | Error | Parse version field |
| R-PUB-06 | keywords present | Warn | Cargo.toml [package].keywords (max 5) |
| R-PUB-07 | categories present | Warn | Cargo.toml [package].categories |
| R-PUB-08 | no path deps in published crates | Error | Scan [dependencies] for path = |
| R-PUB-09 | version consistency | Error | Workspace members that depend on each other have compatible versions |
| R-PUB-10 | publish not false (for publishable crates) | Info | Inventory of publish status |
| R-REL-01 | LICENSE file exists | Error | Check for LICENSE, LICENSE-MIT, LICENSE-APACHE at repo root |
| R-REL-02 | release-plz.toml exists | Warn | Release automation config |
| R-REL-03 | release-plz.toml valid | Warn | Parse and check for workspace config |
| R-REL-04 | cliff.toml exists | Warn | Changelog generation config |
| R-REL-05 | Release workflow exists | Warn | .github/workflows/ contains release* |
| R-REL-06 | CI publish job | Warn | Workflow contains cargo publish or release-plz |
| R-REL-07 | CARGO_REGISTRY_TOKEN | Warn | `gh secret list` contains it |
| R-BIN-01 | Binary release workflow | Info | Workflow builds binaries |
| R-BIN-02 | Linux x86_64 target | Info | Workflow includes linux target |
| R-BIN-03 | Binstall metadata | Info | Cargo.toml has [package.metadata.binstall] |
| R-REL-08 | cargo-semver-checks installed | Warn | Tool available |
| R-REL-09 | cargo publish --dry-run | Error | Actually run dry-run (optional, slow) |

**Implementation notes:**
- R-REL-07 uses `std::process::Command` to run `gh secret list` — needs `#[allow(clippy::disallowed_methods)]` with reason
- R-REL-09 (`cargo publish --dry-run`) is slow — make it opt-in via `--thorough` flag or similar
- For workspace projects, run per-crate checks on each publishable member

**Touches existing code:**
- `src/rs/validate/mod.rs` — wire in under release domain
- `src/cli.rs` — already done in Step 1

**Dependencies:** None. Can develop independently.

---

### Domain C: Test quality (--tests)

**New files:**
- `src/rs/validate/test_checks.rs` — Rust test quality checks
- `src/ts/validate/test_checks.rs` — TypeScript test quality checks

**Rust test checks:**

| ID | Check | Severity | What |
|---|---|---|---|
| R-TEST-01 | cargo-mutants installed | Warn | Tool available for mutation testing |
| R-TEST-02 | .cargo/mutants.toml exists | Warn | Mutation testing configured |
| R-TEST-03 | [profile.mutants] in Cargo.toml | Info | Optimized profile for mutation testing |
| R-TEST-04 | Tests exist | Error | At least one #[test] function found in the workspace |
| R-TEST-05 | Test coverage of public functions | Info | Inventory — count public fns vs test fns |
| R-TEST-06 | Integration tests exist | Info | /tests/ directory with .rs files |
| R-TEST-07 | No #[ignore] without reason | Warn | #[ignore] must have a comment explaining why |
| R-TEST-08 | Mutation test hook configured | Info | .claude/hooks or pre-commit includes mutation testing |

**TypeScript test checks:**

| ID | Check | Severity | What |
|---|---|---|---|
| T-TEST-01 | Stryker installed (stryker.config.json) | Warn | Mutation testing configured |
| T-TEST-02 | Test files exist | Error | *.test.ts or *.spec.ts files present |
| T-TEST-03 | Test runner configured | Warn | vitest.config or jest.config present |
| T-TEST-04 | No .skip() without reason | Warn | test.skip or describe.skip must have comment |
| T-TEST-05 | No .only() in committed code | Error | test.only or describe.only blocks other tests |

**Implementation notes:**
- R-TEST-04 scans .rs files for `#[test]` attribute — lightweight
- R-TEST-05 is heuristic — counts `pub fn` in src/ vs `#[test] fn` in tests/ and src/
- T-TEST-05 is critical — `.only()` in committed code means only one test runs in CI

**Touches existing code:**
- `src/rs/validate/mod.rs` — wire in under tests domain
- `src/ts/validate/mod.rs` — wire in under tests domain

**Dependencies:** None. Can develop independently.

---

### Domain D: Semver automation config (part of --release)

This is NOT a separate domain — it's part of release guardrails. The checks in Domain B already cover whether release-plz.toml, cliff.toml, and workflow files exist.

What this plan adds beyond checking:
- `guardrail3 init --profile service` should also scaffold release-plz.toml and cliff.toml
- `guardrail3 generate` should produce/update these files from modules
- New modules: `src/modules/release.rs` — release-plz.toml, cliff.toml canonical content

**New files:**
- `src/modules/release.rs` — embedded release config content

**Touches existing code:**
- `src/commands/generate.rs` — generate release config files
- `src/commands/init.rs` — scaffold release configs
- `src/modules/mod.rs` — register release modules

## Parallel development plan

All 4 can be developed simultaneously since they touch different files:

| Agent | What | New files | Modified files |
|---|---|---|---|
| **Agent 0** (shared) | CLI flags + domain orchestration | — | cli.rs, validate.rs, rs/validate/mod.rs, ts/validate/mod.rs |
| **Agent 1** | Garde boundary validation | garde_checks.rs, clippy_garde module | modules/clippy.rs, clippy_coverage.rs |
| **Agent 2** | Release readiness | rs/validate/release_checks.rs, modules/release.rs | rs/validate/mod.rs, commands/generate.rs, commands/init.rs |
| **Agent 3** | Test quality | rs/validate/test_checks.rs, ts/validate/test_checks.rs | rs/validate/mod.rs, ts/validate/mod.rs |

**Execution order:**
1. Agent 0 runs first (10 min) — adds CLI flags and domain filtering infrastructure
2. Agents 1-3 run in parallel (each independent)
3. Integration: wire each domain into the orchestrator

**Conflict risk:** Low. Agent 0 modifies the orchestrators, but Agents 1-3 only ADD new modules and wire them in at a single call site. The main conflict point is `rs/validate/mod.rs` — each agent adds one line to the orchestrator. Easy to merge.

## Files to create

| File | Agent | Domain |
|---|---|---|
| `src/rs/validate/garde_checks.rs` | 1 | architecture |
| `src/rs/validate/release_checks.rs` | 2 | release |
| `src/rs/validate/test_checks.rs` | 3 | tests |
| `src/ts/validate/test_checks.rs` | 3 | tests |
| `src/modules/release.rs` | 2 | release |

## Files to modify

| File | Agent(s) | What changes |
|---|---|---|
| `src/cli.rs` | 0 | Add --code, --architecture, --release, --tests flags |
| `src/commands/validate.rs` | 0 | Pass domain flags to orchestrators |
| `src/rs/validate/mod.rs` | 0, 1, 2, 3 | Domain filtering + wire new modules |
| `src/ts/validate/mod.rs` | 0, 3 | Domain filtering + wire test_checks |
| `src/modules/clippy.rs` | 1 | Add Garde ban modules |
| `src/rs/validate/clippy_coverage.rs` | 1 | Add Garde bans to expected lists |
| `src/commands/generate.rs` | 2 | Generate release config files |
| `src/commands/init.rs` | 2 | Scaffold release configs |
| `src/modules/mod.rs` | 2 | Register release modules |
