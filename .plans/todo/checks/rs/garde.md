# RS-GARDE — Garde boundary validation checker (14 rules)

> Superseded as the primary family plan by [`.plans/by_family/rs/garde.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/garde.md).
> Keep this file as a detailed rule ledger and migration/history reference.

**Input:** Cargo.toml + clippy.toml + *.rs files
**Parser:** TOML + syn AST
**Current code:** `apps/guardrail3/crates/app/rs/families/garde/**`
**Legacy seed material:** `apps/guardrail3/crates/app/rs/validate/garde_checks.rs`

**Why separate from RS-CLIPPY:** RS-CLIPPY checks baseline bans (always required). RS-GARDE checks additional bans and source-level boundary enforcement that only make sense when the project uses garde for input validation boundaries. RS-GARDE-01 checks garde exists first — if it doesn't, the other rules are skipped.

## Implementation mapping contract

- exactly one `RS-GARDE-*` rule ID per production file
- exactly one rule-specific `*_tests/` module directory per production rule file
- `mod.rs` orchestrates only
- `discover.rs`, `facts.rs`, `inputs.rs`, `parse.rs`, and `garde_support.rs` may contain shared discovery, facts, parsing, and canonical ban helpers only

Forbidden:

- grouped family test files such as `garde_tests.rs`
- helper files that hide multiple rule predicates behind one API

## Root discovery / ownership model

`RS-GARDE` is a multi-root family.

Its owned Rust policy roots are:
- workspace roots
- standalone package roots that are not members of a workspace

It must not collapse to repo-root-only behavior.

For each owned root, the family determines:
- whether garde is actually in play for that root
- which covering `clippy.toml` applies to that root
- which Rust source files belong to that root

Verified root-policy note:
- if the effective root config is package-driven by `[rust.packages]`, root garde gating must inherit `[rust.packages.checks]`
- the root must not always fall back to the global default garde setting
- otherwise `RS-GARDE-02..09` can fail open or overfire at the root

Rules about clippy ban presence are evaluated against the covering clippy config for the owned root, not against an arbitrary repo-global file.

Important root-policy detail:
- if the root config surface is package-driven via `[rust.packages]`, the root policy must inherit that package policy
- the family must not assume the root always uses only the global default `[rust.checks]` / `[profile]` settings
- otherwise the checker can fail open by scanning or skipping the root under the wrong garde-enabled state

## Gating model

The family is intentionally conditional:
- `RS-GARDE-01` checks whether garde is present for the owned root
- if garde is absent, the ban and source-enforcement rules for that root do not fire

This is not a fail-open exception. It is the actual product contract: the garde family only governs Rust roots that are using garde as an input-boundary strategy.

## Input integrity / fail-closed expectations

The family depends on:
- owned-root `Cargo.toml`
- the covering `clippy.toml`
- relevant Rust source files under the owned root
- root/policy inputs needed to decide whether garde is enabled for that root

Unreadable or unparsable required inputs must surface explicitly through `RS-GARDE-10`.
That includes:
- Cargo root discovery failures
- clippy config parse failures for owned roots
- Rust source read/parse failures for analyzed files

Malformed inputs must not silently suppress source-level boundary findings.

## Cross-family dependency

`RS-GARDE` deliberately depends on the `RS-CLIPPY` contract:
- clippy owns the canonical ban configuration surface
- garde owns the additional boundary-specific ban requirements and source-level derive/bypass checks

So this family is valid only if:
- clippy coverage/root resolution is correct
- garde resolves the covering clippy config per owned root correctly

That dependency should stay explicit in the plan.

## Rules

Severity note:
- `Error/Info` means the violation path is `Error` and the clean / inventory path is `Info`
- `Warn/Info` means the violation path is `Warn` and the clean / inventory path is `Info`

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-GARDE-01 | R-GARDE-01 | Error/Info | garde crate in `[workspace.dependencies]` or `[dependencies]` of each enabled Rust root | Implemented |
| RS-GARDE-02 | R-GARDE-02 | Warn/Info | Core serde/toml/yaml deserialization method bans in covering clippy config `disallowed-methods` (see list below) | Implemented |
| RS-GARDE-03 | R-GARDE-03 | Warn/Info | Axum extractor type bans in covering clippy config `disallowed-types` (see list below) | Implemented |
| RS-GARDE-04 | R-GARDE-04 | Warn/Info | `reqwest::Response::json` method ban in covering clippy config | Implemented |
| RS-GARDE-05 | R-GARDE-05 | Error/Info | Struct derive inventory: structs with Deserialize/Parser/Args/FromRow + non-primitive fields must also derive Validate. Skips test files. | Implemented |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-GARDE-06 | Warn/Info | Additional deserialization method bans beyond serde_json/toml/yaml (see full list below). All deserialization entry points must go through `Validated<T>` wrapper. | Implemented |
| RS-GARDE-07 | Error/Info | Manual `impl<'de> Deserialize<'de> for T` bypasses derive check. Scan for impl blocks implementing Deserialize trait — flag target type if it has non-primitive fields and doesn't also implement Validate. | Implemented |
| RS-GARDE-08 | Error/Info | Enum derive inventory: enums with Deserialize/Parser/Args/FromRow and tuple/struct variants containing non-primitive fields must also derive Validate. Avoid false positives on C-like enums. | Implemented |
| RS-GARDE-09 | Info | `sqlx::query_as!` and `sqlx::query_as_unchecked!` bypass derive check. Scan macro invocations and flag them as inventory items requiring manual review for validation. | Implemented |
| RS-GARDE-10 | Error | Garde-family input failures: unreadable/parsing-broken Rust sources or broken garde-family policy inputs must surface explicitly instead of being skipped. | Implemented |
| RS-GARDE-11 | Error/Info | Validated boundary fields that require runtime validation must carry a meaningful field-level garde validator. Primitive-only fields, unvalidatable map/set/reference surfaces, and nested validated fields handled by `dive` are excluded. | Implemented |
| RS-GARDE-12 | Error/Info | Nested validated fields must use `#[garde(dive)]` so recursive validation actually runs. Applies to validated nested types, not arbitrary custom types. | Implemented |
| RS-GARDE-13 | Error/Info | If a field-level garde validator references `ctx`, the boundary type must declare `#[garde(context(...))]`. This makes context-driven validation explicit and prevents half-wired ctx usage. | Implemented |
| RS-GARDE-14 | Error | `GuardrailConfig` parse sites must prove a same-function garde validation call before use. First pass targets explicit/inferred `toml::from_str` and `try_into::<GuardrailConfig>()` patterns and skips `#[cfg(test)]` bodies. | Implemented |

## Legacy carry-forward from archived GARDE_GUARDRAILS.md

The older top-level garde design note is being archived, but it still contains live Rust design guidance that needs to be reconciled against the current checker contract. The remaining active interpretations are:

- wrapper-based boundary enforcement does not need a new garde-local source rule right now:
  - the enforceable checker contract in this repo is still the clippy ban surface on raw extractors and raw deserialization
  - wrapper types such as `ValidatedJson<T>`, `ValidatedQuery<T>`, `ValidatedForm<T>`, and `Validated<T>` remain application/library design guidance rather than a separate garde AST rule in guardrail3
- current enforcement chain is:
  - clippy ban pressure on raw extractors / raw deserialization
  - source-level `Validate` inventory
  - manual bypass detection
  - field-level garde validator coverage
  - nested `#[garde(dive)]` enforcement
  - explicit `#[garde(context(...))]` enforcement when `ctx` is used
- canonical clippy parity is now in better shape:
  - the full `RS-GARDE-03` extractor set is generator-backed
  - the full `RS-GARDE-06` deserialization method set is generator-backed
  - the remaining active gaps are no longer about missing canonical ban entries
- field-level garde quality is now explicitly enforced:
  - `RS-GARDE-11` checks meaningful field-level garde constraints
  - `RS-GARDE-12` checks nested validated fields use `#[garde(dive)]`
  - `RS-GARDE-13` checks explicit `#[garde(context(...))]` when field validators reference `ctx`
- runtime validation execution is now partially enforced:
  - `RS-GARDE-14` checks the live `GuardrailConfig` parse boundary, not just derive presence
  - first-pass scope is intentionally narrow to avoid pretending we already have generic validate-call dataflow
- expanded deserialization method coverage is now part of the canonical clippy baseline:
  - `RS-GARDE-06` and `RS-CLIPPY-04` parity tests now pin the expanded method set
  - service and library generated `clippy.toml` baselines now both include:
    - query-string / urlencoded constructors
    - streaming JSON deserializer constructors
    - binary / CSV / XML / CBOR / RON / postcard / flexbuffers entry points
    - config-crate extraction entry points
- expanded extractor ban coverage is now part of the canonical clippy baseline:
  - `RS-GARDE-03` and `RS-CLIPPY-05` parity tests now pin the expanded extractor set
  - service and library generated `clippy.toml` baselines now both include:
    - `axum::extract::Path`
    - `axum::extract::Multipart`
    - `axum::extract::ConnectInfo`
    - `axum_extra::extract::CookieJar`
    - `axum_extra::extract::cookie::Cookie`
    - `axum_extra::extract::TypedHeader`
    - `axum_extra::extract::JsonDeserializer`
    - `axum_extra::extract::JsonLines`
    - `axum_extra::extract::Protobuf`
    - `axum_extra::extract::Cbor`
    - `axum_extra::extract::MsgPack`

So the garde family now implements the enforceable AST-side contract from the legacy doc. The remaining wrapper material in that doc is product-pattern guidance carried by the clippy ban surface rather than a separate garde rule in this checker.

Recent hardening note:
- source-level multi-root attacks for `RS-GARDE-05/07/08/09` should use workspace roots or standalone package roots only
- workspace members are not owned garde roots and should not be used as the multi-root ownership model in tests

Latest audit-hardening checkpoint:
- routed `scoped_files` are now enforced during fact collection, not only during rule emission
  - this closes the real subtree leak where out-of-scope Rust files could still trigger `RS-GARDE-10`
    or influence source-rule state
  - dedicated regressions now live in `facts_tests/scoped_files.rs`
- alias-aware source detection now resolves renamed module imports for:
  - `serde` / `Deserialize`
  - `garde` / `Validate`
  - `sqlx::query_as!` / `query_as_unchecked!`
  - this closes bypasses for `RS-GARDE-05`, `RS-GARDE-07`, `RS-GARDE-08`, and `RS-GARDE-09`
- `RS-GARDE-10` now has direct fail-closed tests for:
  - unreadable Rust source files
  - malformed `guardrail3.toml` during garde policy resolution
- `RS-GARDE-14` is slightly stricter now:
  - same-name `.validate()` calls only suppress a finding when they occur after the parse site
  - this still is not full validate-before-use dataflow and should stay documented as a narrow first pass
- `RS-GARDE-01` applicability is now intentionally narrower:
  - roots with a real `garde` dependency are active as before
  - roots without `garde` stay silent unless parsed source shows garde adoption markers such as
    boundary derives or validate implementations
  - plain infrastructure/assertions/test-support crates without garde markers no longer produce fake
    dependency errors just because they are routed roots
  - `GuardrailConfig` parse sites alone do not count as garde-adoption markers

## Full expected ban lists

### RS-GARDE-02: Serde deserialization method bans (disallowed-methods)

Core serde (currently implemented):
- `serde_json::from_str`
- `serde_json::from_slice`
- `serde_json::from_value`
- `serde_json::from_reader`
- `toml::from_str`
- `serde_yaml::from_str`
- `serde_yaml::from_reader`

### RS-GARDE-04: reqwest response deserialization (disallowed-methods)

- `reqwest::Response::json`

### RS-GARDE-06: Additional deserialization method bans (disallowed-methods)

Query string / URL encoding:
- `serde_qs::from_str`
- `serde_qs::from_bytes`
- `serde_urlencoded::from_str`
- `serde_urlencoded::from_bytes`
- `serde_urlencoded::from_reader`

Binary formats:
- `ciborium::from_reader`
- `ciborium::de::from_reader`
- `rmp_serde::from_slice`
- `rmp_serde::from_read`
- `rmp_serde::decode::from_slice`
- `rmp_serde::decode::from_read`
- `bincode::deserialize`
- `bincode::deserialize_from`
- `bincode::serde::decode_from_slice`
- `bincode::serde::decode_from_reader`

Tabular / structured:
- `csv::Reader::deserialize`
- `csv::StringRecord::deserialize`
- `csv::ByteRecord::deserialize`

XML:
- `serde_xml_rs::from_str`
- `serde_xml_rs::from_reader`
- `quick_xml::de::from_str`
- `quick_xml::de::from_reader`

Other formats:
- `ron::from_str`
- `ron::de::from_str`
- `serde_cbor::from_slice`
- `serde_cbor::from_reader`
- `postcard::from_bytes`
- `flexbuffers::from_slice`

Streaming JSON (bypasses `serde_json::from_*` bans — from audit round 2):
- `serde_json::Deserializer::from_str`
- `serde_json::Deserializer::from_slice`
- `serde_json::Deserializer::from_reader`

Alternative TOML:
- `toml_edit::de::from_str`
- `toml_edit::de::from_slice`
- `toml_edit::de::from_document`

Config crates (from audit round 2):
- `config::Config::try_deserialize`
- `figment::Figment::extract`

### RS-GARDE-03: Axum extractor type bans (disallowed-types)

Core extractors:
- `axum::extract::Json`
- `axum::Json`
- `axum::extract::Query`
- `axum::extract::Form`

Expanded extractor coverage:
- `axum::extract::Path`
- `axum::extract::Multipart`
- `axum::extract::ConnectInfo`
- `axum_extra::extract::CookieJar`
- `axum_extra::extract::cookie::Cookie`
- `axum_extra::extract::TypedHeader`
- `axum_extra::extract::JsonDeserializer`
- `axum_extra::extract::JsonLines`
- `axum_extra::extract::Protobuf`
- `axum_extra::extract::Cbor`
- `axum_extra::extract::MsgPack`

## Code fixes for migration

| Location | Bug | Fix |
|----------|-----|-----|
| `garde_checks.rs` lines 148-171 | `content_has_garde_dependency()` uses line-by-line parsing instead of TOML parser. Confused by comments, multi-line values. | Use `toml::Value` parsing in the new family. |
| `ast_visitors.rs` SKIP_OK_TYPES | `char` missing from primitive skip list | New family must treat `char` as primitive-safe for garde derive exemption. |
| `ast_visitors.rs` DeriveVisitor | Enum handling defaults `has_non_primitive_fields = true` for all enums, causing C-like enum false positives | New family must use explicit `enum_has_non_primitive_fields()` logic. |
| `garde_checks.rs` / `ast_helpers.rs` | unreadable or unparsable source files are skipped silently | New family must surface these as `RS-GARDE-10` errors. |

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| `#[serde(deserialize_with)]` escape hatch | Too narrow. Requires field-level attribute analysis. Custom deserializer might validate internally. |
| `axum::body::Bytes` / `axum::extract::RawBody` | Doesn't deserialize. Subsequent `serde_json::from_slice` is caught by method bans. Banning Bytes would break file uploads. |
| `axum::extract::Extension` | Internal middleware state, not a trust boundary. Deserialization (if any) happens in middleware where method bans apply. |
| `axum::extract::Request` | Banning prevents implementing ValidatedJson itself. Method bans catch deserialization on the body. |
| `tonic::Request` (gRPC) | Protocol-specific. Protobuf has own type system. Most projects use axum/REST. Revisit when tonic needed. |
| Type aliases hiding Deserialize | Transparent to clippy bans. No new attack surface. |
| `#[serde(from = "OtherType")]` proxy | Proxy type flagged by RS-GARDE-05 if it has non-primitive fields. Legitimate custom validation pattern. |
| `sqlx::query_scalar!` | Returns single values. Minimal validation concern. |
| `serde::Deserialize::deserialize` trait method | Transitively covered by Deserializer constructor bans. Nobody calls this directly. |
| Per-crate clippy.toml drops garde bans | Already covered by RS-CLIPPY-13 (per-crate must contain workspace bans). |

## Notes for new implementation

- The old `garde_checks.rs` code is only a seed, not the target architecture.
- Old `R-GARDE-01..05` tests are useful as adversarial seeds, especially for:
  - garde dependency present/missing
  - ban completeness shape
  - derive inventory for `Deserialize` / `Parser` / `Args` / `FromRow`
  - primitive-only exemption
- Old implementation must NOT be copied as-is:
  - no line-based Cargo parsing
  - no silent source parse/read skips
  - no enum false positives
  - no gating later checks on heuristically found prior `CheckResult`s
