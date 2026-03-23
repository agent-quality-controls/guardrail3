# Shared Rule Test Story

## Goal

Make every rule hard to bypass.

Tests should attack the rule contract, not merely confirm that the current implementation emits something.

## Universal test story per rule

Every rule should grow toward this matrix:

1. `golden`
- prove the golden fixture passes this rule

2. `attack vector`
- one test = one attack vector
- mutate the golden fixture everywhere that vector should break the rule
- do not localize the mutation to one tiny example unless the rule itself is inherently local

3. `owned hit set`
- assert all Rust-owned targets that should fire do fire

4. `owned non-hit set`
- assert out-of-scope targets do not fire
- especially:
  - non-Rust roots
  - sibling families
  - similar-but-valid structures

5. `multi-root`
- if the rule should fire in multiple Rust roots, the test must mutate all of them at once

6. `nested-root`
- if nested roots are part of the family model, mutate both top-level and nested roots in one attack test

7. `false-positive control`
- mutate nearby valid structures and prove the rule does not overfire

8. `fail-closed`
- if malformed/unreadable input should produce a failure, test that directly

9. `precedence / inheritance / shadowing`
- if the family has root resolution, override precedence, workspace inheritance, or shadowing, attack those directly

10. `severity exactness`
- assert the exact severity, not just presence of the rule ID

## Assertion standard

Do not stop at:
- “some result exists”
- “rule ID appears”

Prefer:
- exact result count
- exact target paths
- exact severity
- exact missing/present set comparisons

## Layout standard

For every rule:
- `rs_x_yy_rule_tests/`
- `mod.rs`
- files split by attack class such as:
  - `golden.rs`
  - `multi_root.rs`
  - `nested_root.rs`
  - `bypasses.rs`
  - `false_positives.rs`
  - `fail_closed.rs`
  - `severity_exactness.rs`

## Migration priority

When old tests exist, do not port them mechanically first.

First map each old test into:
- which rule it really attacks
- which attack vector it represents
- whether that vector is still valid in the new architecture

Then rebuild the coverage in the new structure.
