# RS-HEXARCH — Hex arch structure + dependency direction checker (27 rules)

> Superseded as the primary family plan by [`.plans/by_family/rs/hexarch.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/hexarch.md).
> Keep this file as a detailed rule ledger and migration/history reference.

**Input:** Directory structure + Cargo.toml files (workspace + per-crate) + *.rs files (for ports/adapter content checks)
**Parser:** TOML + filesystem + syn AST
**Current code:** `arch/rs_arch_01/`, `hex_arch_checks.rs`

## Structure rules (from old RS-ARCH-01 sub-rules — now individual rules)

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-HEXARCH-01 | Error | crates/ exists at app level | Implemented |
| RS-HEXARCH-02 | Error | Required contents {adapters, app, domain, ports} in crates/; optional macros/ allowed | Implemented |
| RS-HEXARCH-03 | Error | {inbound, outbound} in adapters/ and ports/ | Implemented |
| RS-HEXARCH-04 | Error | Loose files in structural/container dirs (only .gitkeep allowed) | Implemented |
| RS-HEXARCH-05 | Error | Container dirs not empty (must have subdirs or .gitkeep) | Implemented |
| RS-HEXARCH-06 | Error | Leaf valid (Cargo.toml or crates/ hex-in-hex or .gitkeep) | Implemented |
| RS-HEXARCH-07 | Error | Workspace members cover all live app-local Cargo roots | Implemented |
| RS-HEXARCH-08 | Error | App Cargo.toml is workspace | Implemented |
| RS-HEXARCH-09 | Error | No extra workspace members | Implemented |
| RS-HEXARCH-10 | Error | Members within app boundary | Implemented |
| RS-HEXARCH-11 | Error | Root workspace doesn't include apps | Implemented |
| RS-HEXARCH-12 | Error | src/ banned at app level | Implemented |
| RS-HEXARCH-27 | Error | Nested workspace forbidden under app root | Implemented |

Current migrated structural coverage:
- the old `rs_arch_01` golden fixture is now exercised by the new family
- rule-specific test modules preserve the old sharp edge cases for `01..12`
- total current family coverage is 47 `hexarch` tests in the new architecture

## Dependency direction rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-HEXARCH-13 | R51 | Error | Dependency direction violation (adapters→domain OK, domain→adapters ERROR) | Implemented |
| RS-HEXARCH-14 | R52 | Info | Dependency graph inventory | Implemented |
| RS-HEXARCH-15 | R53 | Warn | Missing per-app `rust.apps.*` boundary configuration | Implemented |

## Direction check hardening (audit round 1)

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-HEXARCH-16 | Error | `[patch.*]` and `[replace]` sections bypass direction check. Scan workspace root Cargo.toml — if a patch replacement path resolves to a layer-violating location, Error. | Implemented |
| RS-HEXARCH-17 | Error | `workspace = true` dependencies invisible to direction check. Resolve to actual `[workspace.dependencies]` entry. If resolved entry has a `path`, apply layer direction check. | Implemented |
| RS-HEXARCH-18 | Error | Crate renaming via `package` field evades name-based layer lookup. Resolve using both alias AND `package` value AND `path`. | Implemented |
| RS-HEXARCH-19 | Error | Cycle detection in workspace path dependency graph. Same-layer logical cycles (A→B→C→A all in crates/app/) compile fine but indicate confused architecture. | Implemented |
| RS-HEXARCH-20 | Warn | dev-dependency direction violations as separate rule. Test fixtures legitimately need cross-layer deps. Warn not Error. Separate ID allows configuration. | Implemented |

## Layer content enforcement (audit round 2)

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-HEXARCH-21 | Error | Domain crate purity: domain-layer crates may only depend on workspace path deps (domain/ports), a built-in pure-crate allowlist (serde, thiserror, chrono, uuid, etc.), and user-configured `allowed_deps`. ANY other dependency is Error. Must NOT skip `optional = true` entries. | Implemented |
| RS-HEXARCH-22 | Warn | Ports public-surface rule: crates in `crates/ports/` must not expose public behavior outside trait items. Warn if a ports crate defines any `pub fn` outside trait items or any public inherent methods on concrete types. Passive DTO/error/type declarations and trait impls are allowed. | Implemented |
| RS-HEXARCH-23 | Error | Adapter defines `pub trait`: crates in `crates/adapters/` should IMPLEMENT traits from ports, not define their own. `pub trait` in adapter = Error. `pub(crate) trait` is fine. | Implemented |
| RS-HEXARCH-24 | Error | Cross-app boundary violation: no workspace path dep may cross app boundaries. Source in `apps/X/`, target in `apps/Y/` (X≠Y) = Error. Cross-app goes through `packages/` or APIs. | Implemented |
| RS-HEXARCH-25 | Error | `target.'cfg(...)'.dependencies` direction check: platform-conditional deps invisible to current checker. Iterate all TOML keys matching `target.*`, check dep sub-tables with same logic as RS-HEXARCH-13. | Implemented |
| RS-HEXARCH-26 | Error | Member manifest parse error blocks dependency analysis: if a workspace member `Cargo.toml` cannot be parsed, fail closed instead of silently skipping dependency-direction checks for that member. | Implemented |

## Relocated rules (no longer in hexarch)

| Old ID | What | New location | Why |
|--------|------|-------------|-----|
| RS-ARCH-05 (R55) | Workspace edition + rust-version metadata | RS-CARGO-05 | Workspace metadata, not hex arch |
| RS-ARCH-06 (R56) | Publish status inventory | RS-RELEASE-09 | Release concern, not hex arch |
| RS-ARCH-07 (R57) | Release profile settings inventory | RS-RELEASE-10 | Release concern, not hex arch |
| RS-ARCH-08 (R58) | Direct std::fs usage | RS-CODE-15 | Code scan, not hex arch |

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| Composition root enforcement | Too many legitimate multi-binary patterns. Users declare composition roots via guardrail3.toml. |
| Feature-gated deps as separate rule | Covered by RS-HEXARCH-21 which must not skip `optional = true` deps. |
| build.rs content analysis | ROI too low. Build-dependencies check covers the dependency graph. |
| Proc-macro crate placement | No proc-macro crates in any current project. Put in `packages/` or configure exception if needed. |
| Re-export boundary violations | Rust crate boundaries are opaque at compile time. Re-exports don't create transitive Cargo deps. |
| test/ directory skip abuse | Deliberate behavior. Moving production crate to tests/ breaks all production imports. |
| Facade-only lib.rs | Belongs in RS-CODE-27 (library profile check). Code quality, not architecture. |
| Adapter-implements-port full verification | Requires cross-crate name resolution syn can't do. RS-HEXARCH-23 (pub trait in adapter = Error) is the cheap heuristic. |

## Relationship to other families

### RS-ARCH

`RS-ARCH` owns:
- repo-global Rust root placement
- zone classification (`apps/*`, `packages/*`, `other`)
- misplaced-root reporting
- overlap/ownership legality between app/package zones

`RS-HEXARCH` does not emit repo-global misplaced-root findings.
It assumes `RS-ARCH` has already answered whether a Rust root belongs in the app zone at all.
