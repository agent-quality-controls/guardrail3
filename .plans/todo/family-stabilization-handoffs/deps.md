# RS-DEPS Stabilization + Attack Handoff

Owner root: `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/deps`

## What This Handoff Is For

This is **not** a repo-cleanup task.

The goal is to make the `RS-DEPS` family itself:

- structurally self-hosted like `test`, `arch`, `cargo`, `hexarch`, and `code`
- clean under `RS-ARCH`
- clean under `RS-TEST`
- clean under `RS-DEPS`
- adversarially reviewed so its rules are trustworthy

Do **not** spend time cleaning repo-wide dependency policy drift outside the family unless a finding proves to be a detector bug.

The priority is:

1. stabilize the family structure
2. make the family self-enforcing
3. attack the rules for false positives / false negatives / fail-closed gaps
4. fix the rules

## Read First

Architecture:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/README.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/README.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/deps.md`

Stabilized family specimens:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/Cargo.toml`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/test_support/src/lib.rs`

Current target family:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/deps/Cargo.toml`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/deps/src`

## Current Snapshot

As of handoff time:

- no family README yet
- still a single-crate family
- current source tree includes:
  - `facts.rs`
  - `inputs.rs`
  - `lib.rs`
  - `test_support.rs`
  - rule files `RS-DEPS-01..11`
  - rule-specific sidecar dirs already exist
- size is moderate:
  - about `48` Rust files
  - about `2856` LOC

Last known family-health snapshot before the current outer-workspace break:

- `RS-TEST` on the family root: about `23` errors
- self-family was partially green but not yet stabilized

Current global blocker:

- top-level Cargo commands from `apps/guardrail3/Cargo.toml` are currently poisoned by the in-flight `deny` workspace split
- the error looks like:
  - `multiple workspace roots found in the same workspace`
  - `.../families/deny`
  - `.../apps/guardrail3`

So if top-level validation fails for unrelated reasons, do **not** “fix” deny or the outer workspace just to get `deps` green. Another agent owns that lane.

## Shared Rust Architecture You Must Respect

Do not reintroduce family-local root discovery.

The intended flow is:

```text
ProjectTree
  -> placement
  -> family_selection
  -> FamilyMapper
  -> typed family route
  -> family runtime/orchestrator
  -> typed rule inputs
  -> pure rule functions
```

Meaning:

- `placement` decides what Rust roots exist
- `family_selection` decides which families run
- `FamilyMapper` routes scope into typed family inputs
- the family runtime may do family-local parsing/discovery **inside routed inputs only**

For `deps`, this matters a lot:

- tool-install rules are validation-root scoped
- allowlist rules are crate-local
- lockfile rules are Rust-root scoped

Do **not** let the family rediscover its own universe of Cargo roots outside `RsDepsRoute`.

## Expected End State

The family root should end up shaped like:

```text
families/deps/
  Cargo.toml
  README.md
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        facts.rs
        inputs.rs
        rs_deps_01_*.rs
        ...
        rs_deps_11_*.rs
        rs_deps_01_*_tests/
          mod.rs
        ...
    assertions/
      Cargo.toml
      src/
        lib.rs
        rs_deps_01_*.rs
        ...
  test_support/
    Cargo.toml
    src/
      lib.rs
```

Notes:

- `deps` already has good rule-per-file and sidecar structure; the missing part is the self-hosted workspace split
- runtime sidecars should not keep reusable semantic proof
- assertions should own reusable result checks
- `test_support` should hold generic fixture/build helpers only

## What To Do

### Phase 1: Structural stabilization

1. Add `README.md` for the family.
2. Convert `families/deps/Cargo.toml` into a workspace if it is still a single crate.
3. Move production code into `crates/runtime/src/`.
4. Add sibling `crates/assertions`.
5. Add sibling `test_support` and move generic helpers out of runtime-local `test_support.rs`.
6. Update workspace wiring so `guardrail3-app-rs-family-deps` points at `crates/runtime`.
7. Make the family pass:
   - `RS-ARCH`
   - `RS-TEST`
   - `RS-DEPS`

### Phase 2: Attack the rules

Attack `RS-DEPS` itself after structural stabilization.

You are looking for:

- false positives
- false negatives
- fail-closed gaps
- route/scope leaks
- allowlist resolution mistakes
- lockfile ownership mistakes
- `.gitignore` precedence mistakes
- rule behavior that disagrees with `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/deps.md`

You are **not** looking to reduce repo-wide dependency findings unless the detector is wrong.

### Phase 3: Fix the rules, not the repo

For every concrete detector bug:

1. add or update a rule-specific regression
2. patch the rule / parser / fact collector
3. rerun family tests
4. rerun family self-validation

If a wider-repo finding is legitimate debt, leave it alone.

## Attack Method

Use this approach:

1. Read `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/deps.md`.
2. Compare each implemented rule against that contract.
3. Stabilize the family first, then attack semantics.
4. Focus attack energy on the mixed-scope complexity:
   - tool PATH checks
   - crate-local allowlists
   - root-level lockfile ownership
   - `workspace = true` resolution
   - renamed dependencies
   - path dependencies that are and are not workspace-owned
   - `.gitignore` ancestor precedence and nested unignore
   - malformed `guardrail3.toml`, workspace manifests, and member manifests
5. Treat `RS-DEPS-CONFIG-05` as planned only. Do not invent it unless explicitly asked.

## High-Value Attack Targets

Attack these first:

- `RS-DEPS-CONFIG-01..07`
  - unauthorized dependency detection by section
  - `workspace = true` resolution
  - renamed dependencies
  - target-specific tables not yet implemented
- `RS-DEPS-09/10`
  - workspace root vs standalone package lockfile ownership
  - nested `.gitignore` precedence
  - accidental repo-root collapse
- `RS-DEPS-11`
  - malformed required policy inputs
  - required workspace manifest lookup failures
  - allowlist/profile fail-open mistakes

## Verify With

When the outer workspace is healthy again:

```bash
cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-deps --lib

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/deps --family arch --inventory --format json

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/deps --family test --inventory --format json

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/deps --family deps --inventory --format json
```

Expected final state:

- family tests pass
- `RS-ARCH`: `0 errors, 0 warnings, 0 info`
- `RS-TEST`: `0 errors, 0 warnings, 0 info`
- `RS-DEPS`: `0 errors, 0 warnings, 0 info`

If top-level validation is still blocked by the unrelated `deny` workspace split, document that explicitly and verify as locally as possible without editing deny-owned files.

## Output Expected From The Worker

The worker should leave behind:

- the stabilized family structure
- a family README that matches reality
- green family tests
- green `RS-ARCH` / `RS-TEST` / self-family validation on the family root once the unrelated workspace blocker is gone
- any rule fixes backed by concrete regressions

If the worker finds a policy ambiguity instead of a bug, they should document it explicitly instead of guessing.
