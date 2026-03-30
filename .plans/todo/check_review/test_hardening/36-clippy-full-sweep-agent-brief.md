# Clippy Full Sweep Agent Brief

You own the `RS-CLIPPY` lane end to end.

This is not just the checker-family lane.
It is one continuous sweep for:

- `RS-CLIPPY` family behavior and tests
- all repo-root `RS-CODE` findings in clippy-owned files

No priorities.
No “later”.
No partial stopping point.
Everything in this lane is a must-fix.

## Scope You Own

You own all work under:

- `apps/guardrail3/crates/app/rs/families/clippy/**`
- `apps/guardrail3/crates/domain/modules/clippy/**`
- `.plans/todo/checks/rs/clippy.md`
- `apps/guardrail3/crates/app/rs/families/clippy/README.md`
- `apps/guardrail3/Cargo.lock` only if your changes require it

You also own repo-root `RS-CODE` cleanup **only** for findings that land in those clippy-owned files.

Do not touch:

- `arch`
- `fmt`
- `code` family
- unrelated repo-root `RS-CODE` findings outside clippy-owned files
- any other active lane

## Current Live State

Current repo-root validation state:

- `RS-CLIPPY`: `0 errors`, `0 warnings`, `115 info`
- repo-root `RS-CODE` on clippy-owned files: `138` error/warn findings

Current clippy-owned `RS-CODE` breakdown:

- `RS-CODE-24`: `135`
- `RS-CODE-09`: `2`
- `RS-CODE-10`: `1`

That means:

- the `clippy` family is already failure-clean on its own rule line
- the remaining lane debt is mostly structural code-policy debt in clippy-owned files
- but the family hardening/fix list is still authoritative and must still be fully reconciled

## Read First

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/checks/rs/clippy.md`
4. `.plans/todo/checks/rs/clippy/FIXES.md`
5. `apps/guardrail3/crates/app/rs/families/clippy/README.md`
6. `.plans/todo/check_review/test_hardening/04-clippy-and-deny.md`
7. `.plans/todo/check_review/test_hardening/14-clippy-deny-coverage-matrix.md`
8. `.plans/todo/check_review/test_hardening/14-clippy-deny-execution-plan.md`

## Main Goal

Finish the whole clippy lane so that:

1. `RS-CLIPPY` remains clean on failures
2. clippy-owned repo-root `RS-CODE` findings are reduced to zero
3. the family contract in `clippy.md`, `README.md`, and live code agree
4. known fix backlog in `FIXES.md` is actually closed, not just postponed
5. an adversarial pass comes back without a live clippy-family bug in the fix scope

## What “Done” Means

You are done only when **all** of the following are true:

- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-clippy --lib` passes
- `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family clippy --format json` has `0 errors` and `0 warnings`
- `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family code --format json` has **no** error/warn findings in:
  - `crates/app/rs/families/clippy/**`
  - `crates/domain/modules/clippy/**`
- `clippy.md` and `README.md` describe the live behavior accurately
- the fix items in `clippy/FIXES.md` are either implemented or explicitly resolved by a documented architectural decision in code+docs, not silently ignored
- you ran an attack pass against the clippy family and it did not surface a concrete remaining check bug in the fix scope

## Non-Negotiable Constraints

- Do not weaken rules.
- Do not downgrade failures just to make the lane green.
- Do not hide findings behind weaker assertions.
- Do not paper over `RS-CODE` findings with lazy justifications if actual cleanup/removal is possible.
- Prefer removal/refactor over more `#[allow(...)]`.
- Prefer exact-output assertions over set-like “good enough” helpers.
- Keep the write set inside the clippy lane.

## Concrete Work To Finish

### 1. Close the `RS-CLIPPY` fix backlog in `FIXES.md`

Treat every `FIX NOW` item as mandatory.

That includes:

- `RS-CLIPPY-24` fail-closed behavior on wrong-shape `.cargo/config*`
- malformed ban-entry handling in shared parsing
- replacing set-based assertion helpers with exact-result assertions where exactness is implied
- parity tests importing canonical exports from `domain/modules/clippy`
- wrong-type vs missing-value diagnostics for thresholds and policy booleans
- stronger exact-output proofs for `RS-CLIPPY-04/05`
- missing branch/fail-closed sidecars already called out in the fix list

Treat `DECIDE` items as mandatory too:

- you must not leave them hanging
- if the right answer is architectural, make the decision and implement it
- then update `clippy.md` and `README.md` so the chosen behavior is explicit

Treat `TEST GAP` items as mandatory:

- add the missing sidecars
- do not leave coverage holes because the family is currently green

Treat `DOC DRIFT` items as mandatory:

- the docs must match reality by the end

### 2. Eliminate clippy-owned repo-root `RS-CODE` findings

Current clippy-owned `RS-CODE` debt is:

- `RS-CODE-24`: sidecar/path-attr warnings across clippy test modules
- `RS-CODE-09`: oversize files
- `RS-CODE-10`: one import-count error

This is your lane.
Fix these inside clippy-owned files while preserving the clippy family structure.

Preferred order:

1. `RS-CODE-24`
   - remove/restructure `#[path = ...]` where possible
   - if some are structurally necessary, convert the layout to the clean family pattern rather than leaving a warning farm behind
2. `RS-CODE-10`
   - reduce the oversized import surface in clippy domain/runtime code
3. `RS-CODE-09`
   - split/refactor the two oversize files instead of tolerating them

### 3. Keep generator/runtime parity real

`domain/modules/clippy` is not auxiliary.
It is canonical policy surface.

By the end:

- checker expectations and generator outputs must match
- parity tests should derive from canonical exports, not copied tables
- clean-path inventory behavior should be explicit and consistent

### 4. Run an adversarial pass before you stop

After the lane is green:

- attack the family against `clippy.md` + `FIXES.md`
- look for fail-open paths
- look for malformed-shape silence
- look for parity drift between runtime and `domain/modules/clippy`
- look for exactness gaps where duplicated/extra findings could still slip through

If the attack surfaces a real bug, fix it before stopping.

## Verification Commands

Use these repeatedly during the sweep:

```bash
cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-clippy --lib

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family clippy --format json

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family code --format json
```

Useful ownership filter for the last command:

- `crates/app/rs/families/clippy/`
- `crates/domain/modules/clippy/`

## Commit Discipline

Commit regularly during the sweep.

Before every commit:

1. generate a worklog filename with `date`
2. write the worklog
3. stage the worklog with the code
4. commit

Do not bundle unrelated lanes into your commits.

## Suggested Execution Order

1. Read the fix backlog and current dirty clippy lane.
2. Finish shared parser/diagnostic correctness items first.
3. Tighten assertion exactness and parity tests.
4. Eliminate clippy-owned `RS-CODE-24` warning farms by restructuring sidecar/path wiring where needed.
5. Fix the remaining clippy-owned `RS-CODE-10` and `RS-CODE-09` findings.
6. Update `clippy.md` and `README.md` to match the live chosen behavior.
7. Run full family verification.
8. Run repo-root `clippy` and repo-root `code` verification for clippy-owned files.
9. Run one adversarial attack pass.
10. If anything real surfaces, fix it and rerun.

## Final Rule

This is a continuous sweep.

Do not stop because:

- the family tests are green
- the family rules are green
- only code warnings remain
- only docs remain
- only attack cleanup remains

You stop only when the entire clippy-owned lane is finished:

- `RS-CLIPPY` fixed
- clippy-owned `RS-CODE` fixed
- docs aligned
- attack pass clean
