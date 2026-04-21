## Goal

Repair the Rust package architecture so the families we are basing everything on follow the intended split:

- parser package owns parsing and reusable query semantics
- ingestion owns discovery, normalization, and fan-out
- family types expose narrow family facts, not parser documents or repo bags
- check packages run pure local rules on atomic inputs

This plan is driven by confirmed evidence from the live codebase, not from stale planning files.

## Confirmed Evidence

### 1. Hooks has a real package-boundary bug, not just isolated rule mistakes

- `hooks` types leak raw shell parser output directly through the family boundary:
  - [g3rs-hooks-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-types/src/types.rs)
  - `G3RsHooksSelectedHookConfigFact.parsed: ParsedShellScript`
  - `G3RsHooksSourceChecksInput.parsed: ParsedShellScript`
- `ParsedShellScript` does not carry enough execution context for the rule shapes we are writing:
  - [hook-shell-parser/crates/types/src/shell_script.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/parsers/hook-shell-parser/crates/types/src/shell_script.rs)
  - it has line text and a few booleans, but not branch context, environment-flow facts, or resolved command facts
- the parser package already owns a real shell command engine:
  - [hook-shell-parser/crates/runtime/src/command_query/engine.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/parsers/hook-shell-parser/crates/runtime/src/command_query/engine.rs)
- but hooks rules re-implement the same shell semantics locally:
  - [hook_rs_09_clippy_denies_warnings/rule.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_09_clippy_denies_warnings/rule.rs)
  - [hook_rs_09_clippy_denies_warnings/support.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_09_clippy_denies_warnings/support.rs)
  - [hook_rs_17_shared_target_dir_present/rule.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_17_shared_target_dir_present/rule.rs)
  - [hook_rs_17_shared_target_dir_present/support.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_17_shared_target_dir_present/support.rs)
  - [hook_rs_16_config_changes_trigger_validation/support.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support.rs)
- the concrete bugs already fixed came out of this split:
  - parser here-string bug
  - clippy deny-warnings false positive
  - unconditional `exit 0` false positive
  - shell-semantics bug in the shared target-dir rule

### 2. Several non-hook Rust families still expose parser documents directly

- `fmt`:
  - [g3rs-fmt-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/fmt/g3rs-fmt-types/src/types.rs)
  - exposes `RustfmtToml`, `CargoToml`, `RustToolchainToml`
- `cargo`:
  - [g3rs-cargo-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/cargo/g3rs-cargo-types/src/types.rs)
  - exposes full `CargoTomlDocument`
- `clippy`:
  - [g3rs-clippy-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/clippy/g3rs-clippy-types/src/types.rs)
  - exposes `ClippyTomlDocument`, `CargoTomlDocument`, `CargoConfigToml`
- `deny`:
  - [g3rs-deny-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/deny/g3rs-deny-types/src/types.rs)
  - exposes full `DenyToml`

This is not a proven correctness bug by itself, but it keeps rules coupled to parser shapes and makes the family contracts wider than they should be.

### 3. Several check packages still do ingestion-owned normalization and bag processing

- `topology` file-tree checks derive secondary facts inside the check package:
  - [g3rs-topology-file-tree-checks/crates/runtime/src/support.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/support.rs)
  - `collect_facts(input)` computes nested workspaces, membership issues, escaping paths, and illegal placements
- `test` source checks still parse and analyze every file inside the check package:
  - [g3rs-test-source-checks/crates/runtime/src/support.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-source-checks/crates/runtime/src/support.rs)
  - `analyze_root(input)` reparses source and builds proof catalogs there
- `apparch` config checks still depend on bag inputs:
  - [g3rs-apparch-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-types/src/types.rs)
  - [g3rs-apparch-config-checks/crates/runtime/src/run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run.rs)
  - rules are fed `Vec<crates>`, `Vec<dependency_edges>`, `Vec<external_dependencies>`
- `release` config checks are also still repo-bag driven:
  - [g3rs-release-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/release/g3rs-release-types/src/types.rs)
  - [g3rs-release-config-checks/crates/runtime/src/run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run.rs)
  - rules are driven from `repo`, `crates`, `edges`, `input_failures`

These are architectural defects. They increase coupling and make it easier for discovery/normalization drift to produce rule errors later.

### 4. There is at least one extra local parser drift in the Rust side

- `test` ingestion has its own local shell parser:
  - [g3rs-test-ingestion/crates/runtime/src/hook_shell.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-ingestion/crates/runtime/src/hook_shell.rs)
- that duplicates shell parsing concepts already owned by `hook-shell-parser`

This is the same class of problem as hooks, even if it has not yet produced a confirmed user-visible bug.

## Scope

This repair work covers only Rust packages.

It does not include:

- new TypeScript family work
- new rule inventory expansion
- repo policy changes outside the Rust packages themselves unless required by a bug fix

## Key Decisions

### Decision 1: Fix correctness bugs first, then shrink boundaries

Reason:

- hooks has confirmed correctness bugs already
- boundary cleanup without first stabilizing correctness is backwards

Alternative rejected:

- broad simultaneous refactor across all Rust families
- rejected because it would mix confirmed bug repair with speculative cleanup

### Decision 2: Treat hooks as the reference repair sequence

Reason:

- it has the strongest confirmed evidence
- it shows the exact failure mode:
  - parser too weak
  - rule package compensates
  - duplicated semantics drift

Alternative rejected:

- start from `cargo` just because it is earlier in the ideal family order
- rejected because the highest-confidence defects are currently in `hooks`

### Decision 3: Do not blindly narrow every parser-backed family in one pass

Reason:

- some parser-backed families are wide but not yet proven incorrect
- we should separate:
  - "confirmed broken"
  - "architecturally too wide"

Alternative rejected:

- immediate repo-wide rewrite of all family types
- rejected because it would be high-risk and low-signal without rule-by-rule evidence

## Repair Program

### Phase 1 - Hooks package seam repair

#### Target

- `packages/parsers/hook-shell-parser`
- `packages/rs/hooks/g3rs-hooks-types`
- `packages/rs/hooks/g3rs-hooks-ingestion`
- `packages/rs/hooks/g3rs-hooks-source-checks`

#### Goals

- move reusable shell command semantics entirely into `hook-shell-parser`
- stop re-implementing shell evaluation in hook rules
- expose narrower hook-family facts instead of raw `ParsedShellScript` where possible
- keep rule files pure and local

#### Concrete steps

1. Inventory every hook rule that still walks shell semantics locally.
2. Group them by semantic need:
   - command presence
   - command argument shape
   - env-flow coverage
   - fail-open detection
   - control-flow / guarded-exit behavior
3. Extend `hook-shell-parser` query API to cover those needs in one place.
4. Add tests to the parser/query layer proving the currently duplicated cases.
5. Replace per-rule support interpreters with parser-query calls.
6. Narrow `g3rs-hooks-*types` so rules receive family-owned facts, not raw parser documents, where that remains practical.

#### Done criteria

- no rule-local shell interpreter stacks remain in hooks source checks
- no duplicated `TokenCursor` / segment splitting / wrapper descent logic remains in hooks rules
- current hook bug fixes are preserved by parser-level tests

### Phase 2 - Remove duplicate shell parsing from `test` ingestion

#### Target

- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/hook_shell.rs`

#### Goals

- decide whether this local shell parser should be deleted in favor of `hook-shell-parser`
- if equivalent semantics are needed, centralize them

#### Concrete steps

1. Identify why `test` ingestion owns a local shell parser.
2. Compare its needs against `hook-shell-parser`.
3. Replace it with the shared parser if the semantics match.
4. If it truly needs different semantics, document the reason explicitly and isolate the divergence.

#### Done criteria

- no accidental duplicate shell parser remains without a justified boundary

### Phase 3 - Move normalization out of check packages for the bag-heavy families

#### Target families

- `topology`
- `test`
- `apparch`
- `release`

#### Goals

- check packages stop building second-order facts from repo bags
- ingestion owns fan-out into atomic rule inputs

#### Concrete steps by family

##### `topology`

- move `collect_facts(input)` logic out of file-tree checks support and into ingestion or family-owned normalization before rules
- replace current bag input consumption with derived atomic inputs such as:
  - one nested workspace issue
  - one membership issue
  - one escaping path issue
  - one illegal family-file placement

##### `test`

- move `analyze_root(input)` and proof catalog construction out of source checks support
- ingestion should parse source once and emit atomic test-family facts
- source rules should stop reparsing and stop owning global proof-catalog assembly

##### `apparch`

- split current bag input into atomic rule inputs where possible:
  - one crate plus the relevant dependency edge set
  - one patch bypass
  - one same-layer cycle
- avoid rules that require whole `Vec<crates>` and whole `Vec<edges>` when the rule is local

##### `release`

- split repo-level and crate-level concerns more cleanly
- avoid running crate-local rules against a monolithic `Vec<crates>` bag when each rule can own one crate or one edge

#### Done criteria

- check runners do not contain fact-collection passes like `collect_facts` or `analyze_root`
- rules consume local inputs representing one firing opportunity

### Phase 4 - Narrow family type surfaces for config families

#### Target families

- `fmt`
- `cargo`
- `clippy`
- `deny`

#### Goals

- stop exporting parser documents through family type crates where not needed
- replace parser-owned values with family-owned snapshots and facts

#### Concrete steps

1. For each family, inventory which parser fields rules actually read.
2. Define family-owned fact types with just those fields.
3. Update ingestion to produce those facts.
4. Update checks to consume the narrowed facts.

#### Done criteria

- family type crates no longer expose full parser document types unless the entire parsed document is truly the atomic fact

## Execution Order

1. Hooks seam repair
2. Test shell-parser deduplication
3. Topology normalization extraction
4. Test normalization extraction
5. Apparch bag breakup
6. Release bag breakup
7. Config-family type narrowing:
   - fmt
   - cargo
   - clippy
   - deny

## Verification Strategy

For each bug or seam repair:

1. Add failing tests first.
2. Fix at the parser / ingestion / type boundary where the defect originates.
3. Run family tests and validator passes.
4. Run adversarial review against the original evidence list above.
5. Do not mark the phase done until the attack pass finds no remaining gap in that phase.

## Files Most Likely To Change

### Hooks

- [hook-shell-parser/crates/types/src/shell_script.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/parsers/hook-shell-parser/crates/types/src/shell_script.rs)
- [hook-shell-parser/crates/runtime/src/command_query/api.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/parsers/hook-shell-parser/crates/runtime/src/command_query/api.rs)
- [hook-shell-parser/crates/runtime/src/command_query/engine.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/parsers/hook-shell-parser/crates/runtime/src/command_query/engine.rs)
- [g3rs-hooks-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-types/src/types.rs)
- [g3rs-hooks-ingestion/crates/runtime/src/run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/run.rs)
- [g3rs-hooks-source-checks/crates/runtime/src/run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs)

### Test

- [g3rs-test-ingestion/crates/runtime/src/hook_shell.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-ingestion/crates/runtime/src/hook_shell.rs)
- [g3rs-test-source-checks/crates/runtime/src/support.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-source-checks/crates/runtime/src/support.rs)
- [g3rs-test-source-checks/crates/runtime/src/run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-source-checks/crates/runtime/src/run.rs)
- [g3rs-test-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/test/g3rs-test-types/src/types.rs)

### Topology

- [g3rs-topology-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/topology/g3rs-topology-types/src/types.rs)
- [g3rs-topology-file-tree-checks/crates/runtime/src/support.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/support.rs)
- [g3rs-topology-file-tree-checks/crates/runtime/src/run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/topology/g3rs-topology-file-tree-checks/crates/runtime/src/run.rs)

### Apparch

- [g3rs-apparch-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-types/src/types.rs)
- [g3rs-apparch-ingestion/crates/runtime/src/run/config.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/config.rs)
- [g3rs-apparch-config-checks/crates/runtime/src/run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/run.rs)

### Release

- [g3rs-release-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/release/g3rs-release-types/src/types.rs)
- [g3rs-release-config-checks/crates/runtime/src/run.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run.rs)

### Config families to narrow later

- [g3rs-fmt-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/fmt/g3rs-fmt-types/src/types.rs)
- [g3rs-cargo-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/cargo/g3rs-cargo-types/src/types.rs)
- [g3rs-clippy-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/clippy/g3rs-clippy-types/src/types.rs)
- [g3rs-deny-types/src/types.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/rs/deny/g3rs-deny-types/src/types.rs)
