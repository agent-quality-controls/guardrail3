# RS-CODE Policy Expansions

This document captures the policy-expansion ideas raised during the adversarial `RS-CODE` audit.

These are not the same as implementation bugs.
An implementation bug means the current rule does not match its stated contract.
A policy expansion means the current rule may be internally consistent, but the audit suggested widening or tightening the contract itself.

## How To Read This

- `Recommended` means the expansion appears directionally strong and worth considering after core bug fixes.
- `Optional` means the idea is plausible, but it depends on how broad `RS-CODE` should become.
- `Not Recommended` means the expansion would likely blur family boundaries or create poor signal.

The current priority remains fixing shared correctness problems first:

- test-context modeling
- `same_line_reason()` parsing
- `cfg_attr` normalization and recursion
- `#[expect(...)]` handling

Those are correctness issues.
The items below are policy decisions.

## Expansion Candidates

### 1. RS-CODE-12: Treat `warn`, `allow`, or missing `unsafe_code` workspace lint as failures

- `Current policy`: `RS-CODE-12` only reports `deny` as bad and `forbid` as good inventory. Other states are silent.
- `Suggested expansion`: error when `unsafe_code` is missing, set to `warn`, or set to `allow`.
- `Reasoning`: silence on absent or weak configuration makes it harder to tell whether the repo has any unsafe boundary at all.
- `Recommendation`: `Optional`
- `Why not a clear bug`: the current rule text only promises `Info` for `forbid` and `Error` for `deny`.
- `Design note`: if adopted, this should be documented as a stricter baseline rule, not slipped in as a silent reinterpretation.

### 2. RS-CODE-15 / RS-CODE-21: Expand filesystem ownership beyond `std::fs`

- `Current policy`: the family bans direct `std::fs` usage and `std::fs::*` glob imports.
- `Suggested expansion`: include `tokio::fs`, `async_std::fs`, and similar async filesystem APIs.
- `Reasoning`: the architectural boundary is about filesystem access, not just one module path.
- `Recommendation`: `Recommended`
- `Design note`: if adopted, define the owned module set explicitly instead of letting the rule drift into ad hoc path matching.

### 3. Add an inventory counterpart for documented `#[garde(skip)]`

- `Current policy`: `RS-CODE-05` and `RS-CODE-06` only report undocumented or weakly documented `garde(skip)` uses.
- `Suggested expansion`: add a companion audit-trail rule for documented `garde(skip)`, analogous to `RS-CODE-04`.
- `Reasoning`: documented validation escapes should remain visible in inventory output.
- `Recommendation`: `Recommended`
- `Design note`: this is only worth doing after the underlying `garde(skip)` contract is clarified, since the current exemption surface is already too fuzzy.

### 4. RS-CODE-29: Count associated types and constants, not just trait methods

- `Current policy`: trait-size checks count only `TraitItem::Fn`.
- `Suggested expansion`: count associated types and constants toward trait surface size.
- `Reasoning`: a trait with many associated items can still be an oversized API surface even if method count stays low.
- `Recommendation`: `Optional`
- `Why not an obvious must-have`: the existing rule is explicitly framed around method count, which is a reasonable first-order measure.

### 5. RS-CODE-32: Extend test assertion-quality ownership to `.unwrap()`

- `Current policy`: `RS-CODE-32` only checks `.expect(...)` quality in test contexts.
- `Suggested expansion`: flag `.unwrap()` in tests as an escape from the message-quality rule.
- `Reasoning`: a developer can bypass the message-quality rule simply by switching from `.expect(...)` to `.unwrap()`.
- `Recommendation`: `Not Recommended`
- `Why`: this starts to overlap with Clippy ownership and changes the rule from “message quality” into a more general panic-style policy.
- `Alternative`: keep `.unwrap()` owned by lint configuration and keep `RS-CODE-32` narrowly about the quality of explicit test messages.

### 6. RS-CODE-07: Inventory `EXCEPTION:` comments in Rust source files

- `Current policy`: `RS-CODE-07` inventories exception comments in config files only.
- `Suggested expansion`: scan `.rs` files for `EXCEPTION:` comments too.
- `Reasoning`: source-level exception comments can represent durable audit-significant escape hatches.
- `Recommendation`: `Optional`
- `Why not automatic`: this would materially widen the rule from config commentary into source commentary and may need stronger syntax rules to avoid noise.

### 7. RS-CODE-09: Use profile-based or rule-configurable file-length thresholds

- `Current policy`: the effective line cap is a hard constant at 500.
- `Suggested expansion`: vary thresholds by profile or by explicit policy configuration.
- `Reasoning`: library crates, binaries, generated facades, and orchestration modules may have different reasonable size ceilings.
- `Recommendation`: `Optional`
- `Risk`: once thresholds become configurable, the rule may lose clarity and become harder to interpret repo-wide.

### 8. RS-CODE-13: Raise `todo!()` / `unimplemented!()` from `Warn` to `Error`

- `Current policy`: these macros warn in production code.
- `Suggested expansion`: treat them as errors because they crash at runtime.
- `Reasoning`: they are operationally closer to hard failures than stylistic smells.
- `Recommendation`: `Optional`
- `Why not clearly required`: this is a severity decision, not a mismatch with the current contract.

### 9. RS-CODE-17: Count non-function impl items toward impl blast radius

- `Current policy`: the threshold is based on impl methods only.
- `Suggested expansion`: include consts and associated types in the blast-radius count.
- `Reasoning`: large impl-level lint suppressions can still hide policy drift even when some items are not methods.
- `Recommendation`: `Optional`
- `Why not clearly required`: the current rule is specifically framed as “covering >3 methods.”

### 10. RS-CODE-23: Inventory all `include_str!()` / `include_bytes!()` usage, not only traversal

- `Current policy`: only path traversal in `include_str!()` / `include_bytes!()` is reported.
- `Suggested expansion`: inventory or warn on all such includes.
- `Reasoning`: even non-traversal includes can create opaque file-boundary coupling.
- `Recommendation`: `Optional`
- `Why`: this is a real policy choice, but it widens the rule from “bypass/traversal” into broader file-coupling inventory.

### 11. RS-CODE-23: Treat `OUT_DIR` traversal as suspicious even in `include!(concat!(env!(\"OUT_DIR\"), ...))`

- `Current policy`: `OUT_DIR` concat gets inventory treatment as a build-script pattern.
- `Suggested expansion`: escalate if the appended path traverses upward.
- `Reasoning`: `OUT_DIR` should not automatically bless path-escape behavior.
- `Recommendation`: `Recommended`
- `Design note`: this is still a policy expansion because the current rule text treats the `OUT_DIR` pattern as an explicit exception.

### 12. RS-CODE-25: Apply weak-public-error checks outside library profile

- `Current policy`: `RS-CODE-33` now owns weak public error forms across routed Rust roots, so this expansion is effectively closed.
- `Suggested expansion`: also check app crates and other public-facing roots.
- `Reasoning`: app-level public APIs can also leak weak error contracts.
- `Recommendation`: `Optional`
- `Risk`: this materially broadens what counts as “public API” in app-shaped repos and may create more noise than value.

### 13. RS-CODE-25: Add `anyhow::Error`, `eyre::Report`, and similar weak public error types

- `Current policy`: this landed as `RS-CODE-33`, which now catches `Result<_, String>`, `Result<_, &str>`, `Result<_, anyhow::Error>`, and `Result<_, Box<dyn Error>>`.
- `Suggested expansion`: include erased public error forms such as `anyhow::Error` and `eyre::Report`.
- `Reasoning`: those are often just as weak as `String` for public contracts.
- `Recommendation`: `Recommended`
- `Design note`: `RS-CODE-33` is now the explicit successor surface. Preserve one finding path per weak public error case.

### 14. RS-CODE-26: Raise glob reexports from `Warn` to `Error`

- `Current policy`: `pub use foo::*` in `lib.rs` warns.
- `Suggested expansion`: make it an error.
- `Reasoning`: glob reexports create unstable and implicit public API surfaces.
- `Recommendation`: `Optional`
- `Why`: this is a severity choice. The present warning-level policy is coherent if the goal is visibility rather than outright prohibition.

### 15. RS-CODE-27: Exempt `#[cfg(test)]` inline modules in `lib.rs`

- `Current policy`: facade-only `lib.rs` rejects inline module bodies broadly.
- `Suggested expansion`: allow inline test modules in `lib.rs`.
- `Reasoning`: some teams consider test-only inline modules acceptable in otherwise facade-oriented files.
- `Recommendation`: `Not Recommended`
- `Why`: the current rule intentionally keeps `lib.rs` structurally pure. Special-casing test inline modules would weaken the simplicity of that contract.

### 16. RS-CODE-29: Apply large-trait checks outside library profile

- `Current policy`: large trait surface checks are library-profile only.
- `Suggested expansion`: also check app crates.
- `Reasoning`: large internal app traits can still be architectural sludge.
- `Recommendation`: `Optional`
- `Risk`: the more internal the code, the less clear it is that “trait surface” should be judged as public-API design rather than local implementation detail.

### 17. RS-CODE-20: Expand extern-block ownership

- `Current policy`: `RS-CODE-20` catches `#[allow(...)]` on `extern` blocks themselves.
- `Suggested expansion`: also cover items inside extern blocks or wrapper-module suppression around extern blocks.
- `Reasoning`: FFI suppression risk can be hidden one layer away from the block itself.
- `Recommendation`: `Optional`
- `Risk`: wrapper-module detection can quickly become fuzzy unless the ownership boundary is specified very clearly.

### 18. RS-CODE-12: Inspect per-crate lint overrides, not just workspace lints

- `Current policy`: `RS-CODE-12` checks workspace lint configuration.
- `Suggested expansion`: inspect per-crate `[lints.rust]` overrides and standalone crates too.
- `Reasoning`: local overrides can weaken the repo’s unsafe policy even if the workspace baseline looks correct.
- `Recommendation`: `Recommended`
- `Design note`: this should probably be a distinct rule or a documented extension of `RS-CODE-12`, because it changes the rule from workspace-baseline validation into override auditing.

## Expansions I Would Not Mix Into RS-CODE

These came up implicitly during the audit, but they should stay out unless the family is intentionally broadened:

- macro-expansion-style hunting for `include!` hidden behind arbitrary `macro_rules!` wrappers
- turning `RS-CODE-32` into a general unwrap policy instead of a message-quality rule
- broad source-comment inventory without a clear syntax contract

## Suggested Order If Expansions Are Pursued

1. Fix shared correctness bugs first.
2. Clarify `garde(skip)` policy before adding any documented-skip inventory rule.
3. Add `#[expect(...)]` support and robust `cfg_attr` handling before broadening surrounding policies.
4. Prefer small, explicit expansions over “catch more stuff” heuristics.
5. Update [`code.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/code.md) at the same time each expansion is adopted, so the inventory stays trustworthy.
