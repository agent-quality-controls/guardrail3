# RS-GARDE — Garde boundary validation checker (10 rules)

**Input:** Cargo.toml + clippy.toml + *.rs files
**Parser:** TOML + syn AST
**Current code:** `garde_checks.rs` (old partial baseline only; new family lives under `app/rs/checks/rs/garde`)

**Why separate from RS-CLIPPY:** RS-CLIPPY checks baseline bans (always required). RS-GARDE checks additional bans and source-level boundary enforcement that only make sense when the project uses garde for input validation boundaries. RS-GARDE-01 checks garde exists first — if it doesn't, the other rules are skipped.

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-GARDE-01 | R-GARDE-01 | Error/Info | garde crate in `[workspace.dependencies]` or `[dependencies]` of each enabled Rust root | Planned in new architecture |
| RS-GARDE-02 | R-GARDE-02 | Warn/Info | Core serde/toml/yaml deserialization method bans in covering clippy config `disallowed-methods` (see list below) | Planned in new architecture |
| RS-GARDE-03 | R-GARDE-03 | Warn/Info | Axum extractor type bans in covering clippy config `disallowed-types` (see list below) | Planned in new architecture |
| RS-GARDE-04 | R-GARDE-04 | Warn/Info | `reqwest::Response::json` method ban in covering clippy config | Planned in new architecture |
| RS-GARDE-05 | R-GARDE-05 | Error/Info | Struct derive inventory: structs with Deserialize/Parser/Args/FromRow + non-primitive fields must also derive Validate. Skips test files. | Planned in new architecture |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-GARDE-06 | Warn/Info | Additional deserialization method bans beyond serde_json/toml/yaml (see full list below). All deserialization entry points must go through `Validated<T>` wrapper. | Planned |
| RS-GARDE-07 | Error/Info | Manual `impl<'de> Deserialize<'de> for T` bypasses derive check. Scan for impl blocks implementing Deserialize trait — flag target type if it has non-primitive fields and doesn't also implement Validate. | Planned |
| RS-GARDE-08 | Error/Info | Enum derive inventory: enums with Deserialize/Parser/Args/FromRow and tuple/struct variants containing non-primitive fields must also derive Validate. Avoid false positives on C-like enums. | Planned |
| RS-GARDE-09 | Info | `sqlx::query_as!` macro bypasses derive check. Scan for `query_as!` macro invocations and flag as inventory item requiring manual review for validation. | Planned |
| RS-GARDE-10 | Error | Garde-family input failures: unreadable/parsing-broken Rust sources or broken garde-family policy inputs must surface explicitly instead of being skipped. | Planned |

## Legacy carry-forward from archived GARDE_GUARDRAILS.md

The older top-level garde design note is being archived, but it still contains live Rust requirements that are not fully implemented yet. They remain active here:

- wrapper-based boundary enforcement is still missing as a first-class library surface:
  - inbound wrappers such as `ValidatedJson<T>`, `ValidatedQuery<T>`, `ValidatedForm<T>`
  - outbound validated wrapper such as `Validated<T>`
- current enforcement is mostly:
  - clippy ban pressure on raw extractors / raw deserialization
  - source-level `Validate` inventory
  - manual bypass detection
- field-level garde quality is still not enforced:
  - no rule yet that checks boundary fields carry meaningful garde constraints
  - no rule yet that checks nested validated fields use `#[garde(dive)]`
  - no rule yet that checks context-driven validation surfaces where required
- expanded extractor ban coverage is still missing from the canonical clippy baseline:
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

So the garde family is implemented, but the full architectural enforcement chain described by the legacy doc is not complete yet.

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

Core extractors (currently implemented):
- `axum::extract::Json`
- `axum::Json`
- `axum::extract::Query`
- `axum::extract::Form`

Missing — must add:
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
