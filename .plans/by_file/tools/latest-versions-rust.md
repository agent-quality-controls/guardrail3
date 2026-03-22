# Latest Rust Toolchain Versions & Breaking Changes (March 2026)

**Date:** 2026-03-19
**Purpose:** Reference for enforcing tool versions in guardrail3

---

## 1. Rust Stable Toolchain

- **Latest stable:** 1.94.0 (released 2026-03-05)
- **Next:** 1.95.0 beta (scheduled 2026-04-16)
- **Key features in 1.94:**
  - `array_windows` stabilized (iterator yielding `&[T; N]` windows over slices)
  - Cargo config `include` key stabilized (load additional config files)
  - TOML 1.1 support in Cargo manifests (note: using TOML 1.1 features raises MSRV)
- **Edition 2024** shipped stable in 1.85.0 (2025-02-20)

**Recommendation:** Pin `rust-toolchain.toml` to `1.94.0`. Edition 2024 should be default for new crates.

---

## 2. Clippy (ships with rustc)

- **Version:** Clippy 0.1.94 (matches Rust 1.94.0)
- **Total lints:** 750+
- **Notable recent additions (1.93-1.94 cycle):**
  - `disallowed_macros` lint added (deny configured macros via `clippy.toml`)
  - Changelog PR #16653 covers 1.94 additions (full list at GitHub CHANGELOG.md)
- **Recommendation:** Continue enforcing `#![warn(clippy::all, clippy::pedantic)]`. Consider adding `disallowed_macros` in `clippy.toml` for macros like `println!` in library code (enforce `tracing` instead).

**Source:** https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md

---

## 3. cargo-deny

- **Latest version:** 0.18.9 (2025-12-08)
- **MSRV:** 1.85.0 (edition 2024)

### Breaking changes since 0.14.x (what most projects use):

| Version | Breaking Change |
|---------|----------------|
| 0.15.x | `bans.workspace-dependencies` moved to its own section (was inline in `bans`) |
| 0.16.x | UTF-8 paths exclusively; non-UTF-8 manifest/license paths no longer work |
| 0.16.x | `--all-features`/`--features`/`--no-default-features` flags added; previously always `--all-features` |
| 0.17.x | `--context` option removed (deprecated since 0.6.3) |
| 0.18.0 | Advisory DB directory naming changed to `{last-path-component}-{stable-hash}` under `$CARGO_HOME/advisory-dbs/` |
| 0.18.0 | MSRV bumped to 1.85.0, edition 2024 |
| 0.18.5 | Release binaries now built with LTO |
| 0.18.6 | `unused-license-exception` option added to configure lint level |
| 0.18.9 | `native-certs` feature removed; root certificate source now handled by `rustls-platform-verifier` (no more `webpki-roots` default) |

### New features worth enforcing:
- `unused-license-exception` lint (0.18.6) -- flag unused license exceptions in deny.toml
- CVSS v4.0 support fixed (0.18.6, via rustsec 0.31)

**Recommendation:** Pin to `0.18.9`. Migrate `deny.toml` if upgrading from 0.14.x (workspace-dependencies section, advisory-db path changes).

**Source:** https://github.com/EmbarkStudios/cargo-deny/blob/main/CHANGELOG.md

---

## 4. rustfmt

- **Latest:** Ships with Rust 1.94.0 (rustfmt 1.8.x series)
- **Key new stable features:**
  - **Style editions** (since 1.85.0/edition 2024): `style_edition` can be set independently from Rust edition in `rustfmt.toml`
  - **`hex_literal_case`** (stable): Control casing of hex literals (`Upper`/`Lower`)
  - **`imports_granularity`** (stable): Controls import merging/splitting. New `One` variant reformats all imports into a single `use` statement
  - **Edition 2024 formatting changes:** Raw identifier sorting (`r#foo`), version-based integer sorting in identifiers

### Stable options (complete list):
`max_width`, `hard_tabs`, `tab_spaces`, `newline_style`, `indent_style`, `use_small_heuristics`, `edition`, `style_edition`, `fn_call_width`, `attr_fn_like_width`, `struct_lit_width`, `struct_variant_width`, `array_width`, `chain_width`, `single_line_if_else_max_width`, `single_line_let_else_max_width`, `reorder_imports`, `reorder_modules`, `hex_literal_case`, `imports_granularity`, `use_field_init_shorthand`, `use_try_shorthand`

**Recommendation:** Set `style_edition = "2024"` in `rustfmt.toml`. Consider `hex_literal_case = "Upper"` for consistency. `imports_granularity = "Module"` is a good default (or `"Crate"` for monorepo style).

**Source:** https://rust-lang.github.io/rustfmt/

---

## 5. cargo-machete

- **Latest version:** 0.9.1 (2025-08-15)
- **Previous:** 0.9.0 (2025-08-15), 0.8.0 (2025-02-25), 0.7.0 (2024-09-25)
- **No known breaking changes** in recent versions
- **Note:** `cargo-shear` exists as an alternative (also on crates.io) but cargo-machete remains the standard

**Recommendation:** Pin to `0.9.1`.

**Source:** https://crates.io/crates/cargo-machete

---

## Summary Table

| Tool | Latest Version | Pin To | Breaking Since Common |
|------|---------------|--------|-----------------------|
| Rust stable | 1.94.0 | 1.94.0 | Edition 2024 (1.85+) |
| Clippy | 0.1.94 | (ships with rustc) | 750+ lints, new `disallowed_macros` |
| cargo-deny | 0.18.9 | 0.18.9 | Major from 0.14.x (see table above) |
| rustfmt | (ships with rustc) | (ships with rustc) | Style editions, new stable options |
| cargo-machete | 0.9.1 | 0.9.1 | None significant |

---

## Sources

- [Rust 1.94.0 Announcement](https://blog.rust-lang.org/2026/03/05/Rust-1.94.0/)
- [Rust Releases](https://releases.rs/)
- [Clippy CHANGELOG](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md)
- [Clippy Lint Index](https://rust-lang.github.io/rust-clippy/master/index.html)
- [cargo-deny CHANGELOG](https://github.com/EmbarkStudios/cargo-deny/blob/main/CHANGELOG.md)
- [cargo-deny Releases](https://github.com/EmbarkStudios/cargo-deny/releases)
- [rustfmt Configuration](https://rust-lang.github.io/rustfmt/)
- [rustfmt Style Editions (Edition Guide)](https://doc.rust-lang.org/edition-guide/rust-2024/rustfmt-style-edition.html)
- [cargo-machete on crates.io](https://crates.io/crates/cargo-machete)
