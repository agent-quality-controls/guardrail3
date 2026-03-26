# RS-TEST Self-Hosting Gotchas

## Problem Class

`RS-TEST-03` had a subject-model blind spot:

- discovery could see root-local sidecar harnesses such as `src/foo_tests/mod.rs`
- discovery could see root-local external harnesses such as `tests/foo.rs`
- but `RS-TEST-03` only iterated discovered `crates/<component>/{runtime,assertions}` pairs
- result: plain single-crate subjects were silently skipped instead of failing closed

This was a validator applicability bug, not a `ProjectTree` crawler bug.

## What Was Fixed

The family now reports unmapped root-local harnesses as `RS-TEST-03` errors instead of dropping them:

- title: `test harness outside runtime/assertions split`
- meaning: the validator saw a harness but could not map it to a discovered `runtime/assertions` component subject

This closes the false-negative class where single-crate roots looked “clean” only because `RS-TEST-03` never executed.

## Remaining Design Tension

There are currently two valid test architectures in this repo:

1. Subject-component architecture
- `x/runtime`
- `x/assertions`
- sidecars under `runtime/src/*_tests/`
- black-box harnesses under `runtime/tests/*.rs`

2. Guardrail-family implementation architecture
- one production rule file per rule
- one rule-specific sidecar test directory per rule file
- shared support in `test_support.rs`

The current validator logic is strongest on the first shape.
The repo also intentionally uses the second shape inside checker-family crates.

This creates two specific self-hosting gotchas:

- `RS-TEST-02` currently assumes cfg-test declarations must name the owned sidecar module directly; that can reject accepted family-local rule-test wiring.
- `RS-TEST-03` now correctly catches unmapped root-local harnesses, but it still needs an explicit notion of accepted guardrail-family implementation roots so it does not treat the checker’s own rule-test sidecars as ordinary application harnesses.

## Current Understanding

- The crawler is not the issue.
- The main issue is validator subject modeling and applicability.
- `RS-TEST` needs to distinguish:
  - “ordinary crate under test with harnesses”
  - “guardrail family implementation root with rule-test sidecars”

Without that distinction, the validator flips between two bad outcomes:

- false negative: silently skip root-local harnesses
- false positive: flag the checker family’s own accepted rule-test layout as if it were invalid application test architecture

## Follow-Up Direction

1. Keep the new fail-closed behavior for ordinary unmapped harnesses.
2. Add explicit family-implementation-root recognition to `RS-TEST-02` / `RS-TEST-03`.
3. Make the `rs/test` family pass its own validator by teaching the validator the accepted family-local rule-test shape rather than relying on accidental skips.
