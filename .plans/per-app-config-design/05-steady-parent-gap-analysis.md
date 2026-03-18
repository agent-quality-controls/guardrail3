# Gap Analysis: guardrail3 vs steady-parent monorepo

**Date:** 2026-03-18
**Method:** Code trace through guardrail3 source against actual steady-parent structure

## Steady-Parent Structure Summary

```
Root Cargo.toml: [workspace] members=["packages/low-expectations","packages/seo-site-files"]
                             exclude=["apps/validator-rust","apps/substack-publisher"]
Root package.json: pnpm@10.32.0, workspaces=["apps/*","packages/*"]
guardrail3.toml: [rust.apps.validator-rust] type="service", [rust.packages] type="library"
                 NO [typescript] section. NO substack-publisher entry.

Rust:
  apps/validator-rust/  — nested workspace with 5 crates (hex arch), custom clippy/deny/rustfmt
  apps/substack-publisher/ — standalone crate, NOT in guardrail3.toml, has own clippy/deny/rustfmt
  packages/low-expectations/, packages/seo-site-files/ — root workspace members

TypeScript:
  apps/landing/ — Next.js content site (Velite, next-intl), per-app eslint.config.mjs
  apps/admin/ — Next.js service (hex arch), standalone tsconfig.json
  packages/ — 12 TS packages (content-constraints, design-tokens, generator, etc.)
  tools/freebie-renderer/ — standalone TS tool (not in apps/ or packages/)

Root configs:
  eslint.config.mjs — 487 lines, project-specific boundary rules
  .stylelintrc.mjs — custom a11y + architecture exceptions
  .npmrc — full supply chain settings
  tsconfig.base.json — strict settings
  .jscpd.json — threshold: 10
  .githooks/pre-commit — 283 lines, monorepo-aware (per-app Rust checks, migration guards, etc.)

Legacy:
  legacy/ — in .gitignore, should be ignored
```

---

## 1. `guardrail3 rs init --dry-run`

### Gap 1.1: substack-publisher NOT discovered as an app

**What happens:** `detect_project` reads root `Cargo.toml`, finds `members=["packages/low-expectations","packages/seo-site-files"]` and `exclude=["apps/validator-rust","apps/substack-publisher"]`. The root workspace yields 2 members (the packages). Then `discover_nested_workspaces` scans `apps/*/Cargo.toml`:
- `apps/validator-rust/Cargo.toml` has `[workspace]` -> discovered as nested workspace with 5 crates
- `apps/substack-publisher/Cargo.toml` has `[package]` but NO `[workspace]` -> **SKIPPED** by `discover_nested_workspaces` (line 109: `let Some(workspace) = table.get("workspace") else { continue; }`)

**What should happen:** substack-publisher should be discovered as a standalone Rust app.

**Impact:** `rs init` would generate config with only `[rust.apps.validator-rust]` and `[rust.packages]`, omitting substack-publisher entirely.

**Fix complexity:** Moderate. `discover_nested_workspaces` needs a fallback for `apps/*/Cargo.toml` files that have `[package]` but no `[workspace]` — treat them as single-crate apps.

### Gap 1.2: Root workspace members classified incorrectly for init

**What happens:** Root workspace has `members=["packages/low-expectations","packages/seo-site-files"]`. In `generate_rs_config_content`, the loop iterates workspace members. For `packages/low-expectations`, `dir.starts_with("packages/")` is true, so `has_packages = true`. The `packages/` members are not added as apps — they get `[rust.packages]`. This is **correct**.

**No gap here.** Packages are correctly handled.

### Gap 1.3: validator-rust app name extraction

**What happens:** validator-rust's nested workspace has members like `crates/domain`, `crates/app`, etc. Their dirs relative to project root are `apps/validator-rust/crates/domain`, etc. In `generate_rs_config_content`, the code extracts `app_name` by checking `dir.starts_with("apps/")` and taking the first component after `apps/` — so `app_name = "validator-rust"`. All 5 crates map to the same app name. `[rust.apps.validator-rust]` is generated once.

**No gap here.** This works correctly.

---

## 2. `guardrail3 ts init --dry-run`

### Gap 2.1: tools/freebie-renderer NOT discovered

**What happens:** `discover_ts_apps` only scans `root.join("apps")`. `tools/freebie-renderer/` is never checked.

**What should happen:** TS tools should be discoverable, or at minimum the user should be able to add them to config.

**Impact:** freebie-renderer won't appear in the generated `[typescript]` config. Source scans will still walk it (walkdir from project root), but arch checks and per-app type detection won't apply.

**Fix complexity:** Moderate. Either expand discovery to also check `tools/`, or document that tools must be manually added to config.

### Gap 2.2: TS packages not discovered by ts init

**What happens:** `discover_ts_apps` only scans `apps/`. The 12 TS packages in `packages/` are not discovered or represented in the generated config.

**What should happen:** TS packages should be discoverable and configurable (at minimum as `type = "library"`).

**Impact:** No per-package type configuration. Source scans still walk packages (walkdir), but architecture checks won't apply. Missing: package-specific checks like ensuring libraries don't import from apps, etc.

**Fix complexity:** Moderate. Add package discovery to `generate_ts_section`, analogous to how Rust `[rust.packages]` works.

### Gap 2.3: Auto-detection works correctly for landing and admin

**What happens:**
- `apps/landing/`: `auto_detect_app_type` finds `content/` directory -> returns `Content`. Correct.
- `apps/admin/`: checks `src/modules/domain/` -> if exists, returns `Service`. If admin has hex arch, detected correctly.

**No gap here** (assuming admin has `src/modules/domain/`).

### Gap 2.4: Rust apps (validator-rust, substack-publisher) skipped by TS discovery

**What happens:** `discover_ts_apps` checks if each `apps/` entry has `package.json` or `.ts/.tsx` files. validator-rust and substack-publisher have no package.json and no TS files, so they are correctly skipped.

**No gap here.**

---

## 3. `guardrail3 rs generate --dry-run`

### Gap 3.1: substack-publisher gets NO generated configs

**What happens:** `generate_rust_files` iterates `cfg.rust.apps` — only `validator-rust` is listed. substack-publisher is not in guardrail3.toml, so no configs are generated for it.

**What should happen:** Either substack-publisher should be in guardrail3.toml (via init discovering it — Gap 1.1), or generate should warn about undiscovered Rust apps.

**Impact:** substack-publisher's existing clippy.toml/deny.toml/rustfmt.toml are manually maintained, not guardrail3-managed. Drift is possible and undetected.

**Fix complexity:** Trivial (once Gap 1.1 is fixed — init discovers it, user adds it to config, generate produces configs).

### Gap 3.2: validator-rust configs placed at correct paths

**What happens:** `resolve_app_paths` maps `"validator-rust"` to `"apps/validator-rust"` via workspace member discovery. Configs go to:
- `apps/validator-rust/clippy.toml`
- `apps/validator-rust/deny.toml`
- `apps/validator-rust/rustfmt.toml`

This is correct — validator-rust's own workspace picks up clippy.toml from its root.

**No gap here.**

### Gap 3.3: Root packages get library-profile configs at project root

**What happens:** `[rust.packages]` exists and `generated_dirs` doesn't contain `"."`, so root-level `clippy.toml`, `deny.toml`, `rustfmt.toml` are generated with library profile.

**No gap here.** This is correct — the root workspace (`packages/low-expectations`, `packages/seo-site-files`) needs its own clippy/deny/rustfmt at the root.

### Gap 3.4: Custom clippy bans in validator-rust would be overwritten

**What happens:** validator-rust has a custom `clippy.toml` with project-specific bans. `generate` would overwrite it with the guardrail3-generated version. The `warn_if_overwriting` function prints a warning, but proceeds.

**What should happen:** Custom bans should be preserved via `.guardrail3/overrides/` mechanism. However, overrides are currently global (one set per project), not per-app. If validator-rust needs different overrides than the root packages, there's no mechanism for that.

**Impact:** Running `rs generate` would clobber validator-rust's custom clippy bans. The warning message says "use .guardrail3/overrides/" but overrides are global — they'd also apply to root package configs.

**Fix complexity:** Major. Per-app overrides require new config structure (`.guardrail3/overrides/validator-rust/clippy-methods.toml` or similar).

### Gap 3.5: deny.toml custom exceptions not preservable per-app

**What happens:** validator-rust's deny.toml has a custom anyhow exception. substack-publisher's deny.toml has `aws-sdk-*` exceptions. The override mechanism is global — one `deny-skip.toml` for the whole project.

**What should happen:** Per-app deny overrides.

**Impact:** Same as 3.4 — global overrides can't represent per-app exceptions without affecting all apps.

**Fix complexity:** Major (same solution as 3.4).

### Gap 3.6: rust-toolchain.toml always generated at project root

**What happens:** `rust-toolchain.toml` is always generated at the project root. steady-parent doesn't have one at root (each app manages its own edition in Cargo.toml).

**What should happen:** This is actually fine — a root `rust-toolchain.toml` pins the Rust version for the whole monorepo.

**No gap here** — this is desirable behavior.

---

## 4. `guardrail3 ts generate --dry-run`

### Gap 4.1: NO [typescript] section — generate exits with "no files to generate"

**What happens:** `generate_ts_files` checks `cfg.typescript.as_ref()` — returns None since guardrail3.toml has no `[typescript]` section. `run_ts` prints "No TypeScript files to generate."

**What should happen:** This is expected — user needs to run `ts init` first.

**No gap here** — working as designed.

### Gap 4.2: After `ts init` — eslint.config.mjs would be overwritten with generic version

**What happens:** `generate_ts_files` always generates `eslint.config.mjs`. The generated version is a generic starter with core guardrail rules. The real steady-parent eslint config is 487 lines with:
- Per-app boundary rules using `eslint-plugin-boundaries`
- Tailwind ban plugin
- Next.js-specific rules
- Fine-grained import restrictions per workspace package

The generated version would clobber all of this.

**What should happen:** For projects with existing complex eslint configs, guardrail3 should either:
1. Not overwrite (generate only if file doesn't exist), or
2. Support eslint.config.mjs as a "validate-only" file (check rules are present without generating), or
3. Support `[typescript.canonical] eslint = false` to skip generation

**Impact:** Critical. Running `ts generate` would destroy a carefully crafted 487-line eslint config.

**Fix complexity:** Moderate. Add `eslint` flag to `CanonicalConfig` (like npmrc/tsconfig_base/jscpd), defaulting to true but allowing `false`.

### Gap 4.3: .stylelintrc.mjs would be overwritten with generic version

**What happens:** `generate_ts_files` generates `.stylelintrc.mjs` if a content app exists. The generated version is a generic a11y config. The real one has custom CSS notation rules and architecture-driven exceptions.

**What should happen:** Same as 4.2 — need a way to skip generation.

**Impact:** Would destroy custom stylelint config.

**Fix complexity:** Moderate. Add `stylelint` flag to CanonicalConfig.

### Gap 4.4: .jscpd.json threshold mismatch

**What happens:** guardrail3 generates `.jscpd.json` with `"threshold": 0`. steady-parent has `"threshold": 10`.

**What should happen:** Either guardrail3 should match (threshold 10) or there should be a config option.

**Impact:** With threshold 0, jscpd reports ALL duplicates. With threshold 10, it only reports above 10%. The validation check would flag this as a diff.

**Fix complexity:** Trivial. The `validate` check should probably just ensure the config exists and has reasonable settings rather than enforcing exact match.

### Gap 4.5: tsconfig.base.json — minor differences

**What happens:** guardrail3's version includes `"declaration": true, "declarationMap": true, "noEmit": true` and `"lib": ["ES2022", "DOM", "DOM.Iterable"]`. steady-parent's version has `"lib": ["ES2022"]` (no DOM). steady-parent doesn't have declaration/declarationMap.

**What should happen:** The difference is intentional — steady-parent uses Node.js packages that don't need DOM types. guardrail3 assumes web projects.

**Impact:** Overwriting would add DOM types to packages that shouldn't have them. `declaration` and `declarationMap` additions are benign.

**Fix complexity:** Moderate. The tsconfig generator could be smarter about lib targets, or respect `[typescript.canonical] tsconfig_base = false`.

### Gap 4.6: .npmrc content matches almost exactly

**What happens:** guardrail3's generated .npmrc and steady-parent's actual .npmrc have the same settings. The only difference: steady-parent has a comment about `trust-policy` being blocked by `undici-types`, and uses `trust-policy=warn` instead of `trust-policy=no-downgrade`.

**Impact:** Minimal. The overwrite would lose the explanatory comment.

**Fix complexity:** Trivial.

### Gap 4.7: Per-app eslint configs (apps/landing/eslint.config.mjs) not managed

**What happens:** guardrail3 only generates/validates the ROOT eslint config. apps/landing has its own eslint.config.mjs that is completely independent.

**What should happen:** guardrail3 should be aware of per-app eslint configs. At minimum, validate should check them. ESLint flat config supports per-directory configs, and monorepos commonly have per-app configs.

**Impact:** Per-app eslint rules are not validated. landing's module boundary rules could be removed without guardrail3 noticing.

**Fix complexity:** Major. Requires per-app eslint config discovery and validation.

---

## 5. `guardrail3 rs validate`

### Gap 5.1: workspace_root points to root, but config checks need nested workspace root

**What happens:** `primary_workspace_root()` returns the first workspace root — which is the PROJECT ROOT (where root Cargo.toml lives with `members=["packages/*"]`). The RS validate then checks:
- R1: clippy.toml at workspace_root (project root) — found (the library-profile one for packages)
- R21: rustfmt.toml at workspace_root (project root) — found
- R24: rust-toolchain.toml at workspace_root (project root) — **NOT found** (steady-parent doesn't have one at root)
- R26: Cargo.toml workspace lints at workspace_root (project root) — the ROOT Cargo.toml has NO `[workspace.lints]`, only `[workspace] members=[...]`

**What should happen:** For multi-workspace monorepos, config checks should run per-workspace. validator-rust's workspace at `apps/validator-rust/` has its own Cargo.toml with extensive `[workspace.lints]`. The root workspace for packages may or may not need lints.

**Impact:** Critical.
- R26-R29 (workspace lints): Would ERROR because root Cargo.toml has no `[workspace.lints]`, even though validator-rust has complete lint config. False negative for root packages (they genuinely lack workspace lints).
- R1/R21: Would check root configs (correct for packages), but miss validator-rust's per-app configs or report them as "per-crate" only.
- R24: Would ERROR for missing rust-toolchain.toml at root.

**Fix complexity:** Major redesign. Validate needs per-workspace-root checking. Each workspace should be validated independently.

### Gap 5.2: Per-crate clippy check looks at wrong paths

**What happens:** `check_per_crate_clippy` iterates `member_dirs` and checks `workspace_root.join(member).join("clippy.toml")`. The member dirs include:
- From root workspace: `packages/low-expectations`, `packages/seo-site-files`
- From nested workspace: `apps/validator-rust/crates/domain`, `apps/validator-rust/crates/app`, etc.

For nested workspace members, the path is `project_root/apps/validator-rust/crates/domain/clippy.toml`. These individual crates likely DON'T have clippy.toml — the workspace-level clippy.toml at `apps/validator-rust/clippy.toml` covers them all.

**What should happen:** Per-crate clippy is Warn severity (not Error), so individual missing crate clippy.toml files would generate warnings. But the actual structure is: workspace-level clippy.toml covers all crates. guardrail3 doesn't understand "clippy.toml at workspace root covers all workspace members."

**Impact:** 5+ warnings for "missing per-crate clippy.toml" for each of validator-rust's crates. These are false warnings.

**Fix complexity:** Moderate. clippy.toml inheritance follows Cargo workspace — if the workspace root has clippy.toml, all member crates inherit it. The check should verify workspace-root clippy.toml, not per-crate.

### Gap 5.3: deny.toml checked at wrong path

**What happens:** `deny_audit::check` reads `workspace_root.join("deny.toml")` — that's the project root. The root deny.toml (library profile for packages) would be checked. validator-rust's deny.toml at `apps/validator-rust/deny.toml` would NOT be checked.

**What should happen:** Each workspace's deny.toml should be checked independently.

**Impact:** validator-rust's deny configuration is not validated. Root packages' deny config IS validated (if it exists).

**Fix complexity:** Major (same per-workspace redesign as 5.1).

### Gap 5.4: clippy_coverage checks root clippy.toml only

**What happens:** `clippy_coverage::check` reads `workspace_root.join("clippy.toml")` — the root one (library profile). validator-rust's service-profile clippy.toml is not checked for ban coverage.

**What should happen:** Per-workspace clippy ban completeness checks.

**Impact:** validator-rust's clippy bans are not validated for completeness.

**Fix complexity:** Major (per-workspace redesign).

### Gap 5.5: substack-publisher completely invisible to validate

**What happens:** substack-publisher is not in any workspace that guardrail3 discovers (it's excluded from root workspace, and has no `[workspace]` section for nested discovery). Its source files ARE walked by walkdir (source_scan starts from project root), so source-level checks (R30-R44, R58) DO apply.

But config checks (R1-R29), architecture checks, and dependency checks do NOT apply because:
- It's not a discovered workspace member
- It has no workspace lints to check
- Its clippy.toml/deny.toml are at `apps/substack-publisher/` which is never checked

**What should happen:** All Rust apps should be validated, whether they're workspace members or standalone crates.

**Impact:** substack-publisher's config compliance is completely unverified.

**Fix complexity:** Major (requires standalone crate detection in addition to workspace detection).

### Gap 5.6: Architecture checks — crate_configs keyed by name, not path

**What happens:** In `run_architecture_checks`, `crate_configs` is built from `cfg.rust.apps` keys (e.g., `"validator-rust"`) and then packages are added by `member.name` (e.g., `"low-expectations"`). When `check_dependency_allowlist` runs, it looks up `workspace_root.join(crate_name).join("Cargo.toml")` — that would be `project_root/validator-rust/Cargo.toml`, which doesn't exist. The actual path is `apps/validator-rust/Cargo.toml`.

**What should happen:** Architecture checks need the resolved app paths (via `resolve_app_paths`), not just the config keys.

**Impact:** Dependency allowlist checks would fail to find Cargo.toml files. Architecture checks may silently skip apps.

**Fix complexity:** Moderate. Pass `resolve_app_paths` result into architecture check functions.

### Gap 5.7: Source scan walks entire project tree including substack-publisher

**What happens:** `collect_rs_files` walks from `workspace_root` (project root) recursively. It finds ALL .rs files including substack-publisher's. Source-level checks (allow hygiene, file length, use count, std::fs usage) run on all files.

**No gap here** for source scan — this is correct and desirable. All Rust source should be scanned regardless of workspace membership.

---

## 6. `guardrail3 ts validate`

### Gap 6.1: Without [typescript] in config — still works via auto-detection

**What happens:** `build_ts_categories` falls back to defaults when no `[typescript.checks]` exists. `has_content_app` auto-detects by scanning discovered apps. Validation runs with default categories.

**No gap here** — this works correctly.

### Gap 6.2: Root eslint.config.mjs validated but per-app ones are not

**What happens:** `eslint_check::check_eslint_config` checks `path.join("eslint.config.mjs")` — the root config. `eslint_plugin_checks` also checks the root config. apps/landing/eslint.config.mjs is never explicitly checked.

**What should happen:** Per-app eslint configs should be validated, at minimum for required plugins and rules.

**Impact:** Per-app eslint rules could be removed or weakened without detection. Landing's module boundary rules are not validated.

**Fix complexity:** Major. Requires per-app eslint validation framework.

### Gap 6.3: tsconfig.base.json checked at root — per-app tsconfig.json not validated

**What happens:** `tsconfig_check::check_tsconfig` checks `path.join("tsconfig.base.json")`. Per-app tsconfig.json files (some standalone, some extending base) are not validated.

**What should happen:** At minimum, validate should check that per-app tsconfigs either extend the base or have equivalent strict settings.

**Impact:** admin's standalone tsconfig (doesn't extend base, targets ES2017) could have weaker settings without detection.

**Fix complexity:** Moderate. Add per-app tsconfig discovery and validation.

### Gap 6.4: Source scan correctly walks all TS files (including tools/ and packages/)

**What happens:** `collect_ts_files` walks from project root. It finds files in apps/, packages/, tools/, etc. `legacy/` is excluded via gitignore. Source checks (T23-T35) run on all discovered TS files.

**No gap here.**

### Gap 6.5: ESLint plugin checks expect plugins in ROOT config

**What happens:** `check_core_plugins` checks the ROOT eslint.config.mjs for plugins like unicorn, sonarjs, regexp. These ARE present in steady-parent's root config.

However, `apps/landing/eslint.config.mjs` uses `eslint-config-next` with its own plugin setup. guardrail3 doesn't check whether the per-app config has equivalent plugin coverage.

**Impact:** Minor in practice — the root config covers most files. But landing's separate config could lack important plugins.

**Fix complexity:** Moderate (per-app eslint validation).

### Gap 6.6: package_check runs only on root package.json

**What happens:** `check_package_json` checks `path.join("package.json")` — root only. Per-app package.json files are not individually validated for devDependencies, scripts, etc.

**What should happen:** Per-app package.json checks (overrides, banned deps, required scripts).

**Impact:** Per-app dependency hygiene not enforced.

**Fix complexity:** Moderate.

---

## 7. `guardrail3 hooks validate`

### Gap 7.1: Pre-commit hook exists and is found

**What happens:** `.githooks/pre-commit` exists. H1 passes. H2 checks `core.hooksPath` — steady-parent's `package.json` has `"prepare": "git config core.hooksPath .githooks"`, so this should be configured.

**No gap here.**

### Gap 7.2: Monorepo-aware steps in steady-parent not expected by guardrail3

**What happens:** steady-parent's pre-commit has monorepo-specific steps:
- Migration consistency check
- Content-constraints staleness check
- Per-app TS type checking (iterates apps/*/tsconfig.json)
- Per-app Rust checks (cd into each app, run fmt/clippy/deny/machete/test)
- Copy-paste detection with threshold 10
- Guardrail tamper detection (eslint-disable, #[allow], config relaxation)

guardrail3's hook_script_checks look for specific patterns like `gitleaks`, `cargo fmt`, `cargo clippy`, `cargo-deny`, `jscpd`, etc. Most of these are present in steady-parent's hook.

**Potential gaps:**
- guardrail3 may not recognize the `cd "$rust_app" && cargo clippy` pattern (it's inside a for loop, not at top level)
- guardrail3 expects `guardrail3 validate` in the hook (for tamper detection) but steady-parent uses grep-based tamper detection instead

**Impact:** Some hook pattern checks may report warnings about missing steps that are actually present but in a different form.

**Fix complexity:** Moderate. Hook checks should be more flexible about how steps are structured.

### Gap 7.3: guardrail3 expects guardrail3 in pre-commit, steady-parent uses grep-based detection

**What happens:** guardrail3's generated pre-commit runs `guardrail3 rs validate --staged` and `guardrail3 ts validate --staged` for tamper detection. steady-parent's pre-commit uses manual grep for `#[allow(` without comments and `eslint-disable` without reason.

**Impact:** hook_checks may flag this as "missing guardrail tamper detection step."

**Fix complexity:** Trivial. The check should recognize both approaches.

### Gap 7.4: Duplication tool check — steady-parent uses jscpd for BOTH Rust and TS

**What happens:** H12 (`check_duplication_tools`) checks which duplication tool is used. guardrail3 expects cargo-dupes for Rust-only, jscpd for TS-only, both for mixed. steady-parent uses `jscpd` with `"format": ["typescript", "rust"]` for everything.

**Impact:** H12 may warn "wrong duplication tool for Rust — use cargo-dupes instead of jscpd."

**Fix complexity:** Trivial. This is a valid stylistic difference — jscpd supporting Rust format is legitimate.

---

## 8. Cross-Cutting Gaps

### Gap 8.1: Single workspace_root assumption

**Root cause of gaps 5.1, 5.2, 5.3, 5.4:** guardrail3 assumes one primary workspace root. Monorepos with multiple Rust workspaces (root for packages, nested for apps) need per-workspace validation.

**Fix:** The validate orchestrator should iterate `project.workspaces` and run config checks for each workspace root independently. This is the single biggest architectural gap.

### Gap 8.2: Config vs app scope mismatch

guardrail3.toml has `[rust.apps.validator-rust]` but validate doesn't use this config to determine WHERE to check configs. It always checks at `primary_workspace_root()`. The config describes per-app settings but the check logic ignores them.

**Fix:** validate should resolve each app's workspace root and run config checks there. The `resolve_app_paths` function exists in generate_helpers — validate should use equivalent logic.

### Gap 8.3: No TS packages in config model

`TypeScriptConfig` has `apps: Option<TsAppMap>` but no `packages` field. TS packages need to be discoverable and configurable, similar to how `[rust.packages]` works.

**Fix:** Add `packages: Option<TsAppMap>` to TypeScriptConfig. Update ts init to discover and generate package entries.

### Gap 8.4: Canonical file generation lacks per-file skip flags

Only 3 files have skip flags in `CanonicalConfig`: npmrc, tsconfig_base, jscpd. The most critical files to skip — eslint.config.mjs and .stylelintrc.mjs — have no skip flag.

**Fix:** Add `eslint` and `stylelint` flags to CanonicalConfig.

### Gap 8.5: No per-app override mechanism

Overrides in `.guardrail3/overrides/` are global. In a monorepo with validator-rust (service, needs anyhow exception) and packages (library, needs different deny rules), there's no way to express per-app overrides.

**Fix:** Support `.guardrail3/overrides/{app-name}/` directories, or per-app override paths in guardrail3.toml.

---

## Summary: Priority-Ordered Gap List

| # | Gap | Severity | Fix Complexity |
|---|-----|----------|---------------|
| 5.1 | Single workspace_root — config checks run against wrong workspace | Critical | Major |
| 5.3 | deny.toml checked at wrong path for nested workspaces | Critical | Major |
| 5.4 | clippy_coverage checks wrong clippy.toml | Critical | Major |
| 4.2 | eslint.config.mjs overwritten with generic version | Critical | Moderate |
| 1.1 | Standalone crate apps (substack-publisher) not discovered | High | Moderate |
| 5.5 | Undiscovered apps completely invisible to validate | High | Major |
| 5.6 | Architecture checks use config keys not resolved paths | High | Moderate |
| 4.3 | .stylelintrc.mjs overwritten with generic version | High | Moderate |
| 8.4 | No skip flags for eslint/stylelint generation | High | Moderate |
| 5.2 | Per-crate clippy false warnings for workspace members | Medium | Moderate |
| 6.2 | Per-app eslint configs not validated | Medium | Major |
| 6.3 | Per-app tsconfig.json not validated | Medium | Moderate |
| 8.5 | No per-app override mechanism | Medium | Major |
| 8.3 | No TS packages in config model | Medium | Moderate |
| 2.1 | tools/ directory not discovered | Low | Moderate |
| 2.2 | TS packages not discovered by ts init | Low | Moderate |
| 4.4 | .jscpd.json threshold mismatch (0 vs 10) | Low | Trivial |
| 4.5 | tsconfig.base.json lib target differences | Low | Moderate |
| 7.2 | Monorepo hook patterns not recognized | Low | Moderate |
| 7.3 | grep-based tamper detection not recognized | Low | Trivial |
| 7.4 | jscpd for Rust not recognized as valid | Low | Trivial |
| 6.6 | Per-app package.json not validated | Low | Moderate |

### The Core Architectural Issue

The single biggest problem is **Gap 8.1: the single workspace_root assumption**. This causes a cascade of false results:

1. Config checks (R1, R21, R24) check the wrong directory
2. Workspace lint checks (R26-R29) read the wrong Cargo.toml
3. deny.toml checks read the wrong deny.toml
4. clippy coverage checks read the wrong clippy.toml

The fix is to make `rs validate` iterate over all discovered workspaces and run config checks per-workspace-root. This is the prerequisite for all other Rust validate fixes.

For TypeScript, the situation is less severe because TS monorepos typically share a single root config. But per-app eslint and tsconfig validation would be a valuable addition.
