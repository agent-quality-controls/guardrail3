# Fix `g3rs validate` errors across 15 workspaces

## Goal

`apps/guardrail3-rs/target/release/g3rs validate --path <ws>` exits 0 for all 15 workspaces in scope. Then `scripts/verify/all.sh` passes 8 layers.

## Workspaces (with rough error counts)

- packages/ts/apparch/g3ts-apparch-ingestion (~56)
- packages/ts/arch/g3ts-arch-file-tree-checks (~4)
- packages/ts/arch/g3ts-arch-ingestion (~32)
- packages/rs/fmt/g3rs-fmt-ingestion (~31)
- packages/ts/astro/content/g3ts-astro-content-file-tree-checks (~16)
- packages/ts/astro/content/g3ts-astro-content-config-checks (~110+)
- packages/rs/garde/g3rs-garde-config-checks (~38)
- packages/ts/astro/i18n/g3ts-astro-i18n-config-checks (~50)
- packages/ts/astro/g3ts-astro-check-support (~16)
- packages/rs/garde/g3rs-garde-types (~2)
- packages/ts/astro/mdx/g3ts-astro-mdx-config-checks (~115)
- packages/ts/astro/mdx/g3ts-astro-mdx-ingestion (~97)
- packages/ts/astro/media/g3ts-astro-media-types (~5)
- packages/rs/release/g3rs-release-filetree-checks (~17)
- packages/ts/astro/media/g3ts-astro-media-config-checks (~77)

## Approach

Per-workspace loop:
1. `cargo clippy --fix --allow-dirty --allow-staged --workspace --all-targets --all-features --manifest-path <ws>/Cargo.toml`
2. Manual fixes for each remaining clippy error category:
   - missing_docs_in_private_items: add `///` doc comment with one-line summary
   - missing_errors_doc: add `# Errors` section to public Result fns
   - missing_panics_doc: add `# Panics` section
   - arithmetic_side_effects: replace `+ 1` with `.checked_add(1).unwrap_or(...)` or `.saturating_add(1)`
   - indexing_slicing: use `.get(i)` or iterator
   - case_sensitive_file_extension_comparisons: use `Path::extension()` with `eq_ignore_ascii_case`
   - excessive_nesting: extract helper functions
   - must_use_candidate: add `#[must_use]`
   - missing_const_for_fn: add `const`
   - wildcard_enum_match_arm: enumerate variants
   - unnecessary_wraps: change return type
   - large_enum_variant: Box the large variant
   - too_many_lines: split file/function
   - too_many_arguments: bundle into struct
   - module_name_repetitions: rename
   - expect_used: replace with `.ok_or_else(|| ...)?`
   - disallowed_methods (std::fs): route through crate-local `fs` port module
   - derive_partial_eq_without_eq: add `Eq`
3. `cargo fmt --manifest-path <ws>/Cargo.toml --all`
4. Validate, exit 0.

## Forbidden

No `#[allow]`/`#[expect]`. No edits outside the 15 workspaces.

## Strategy notes

- Tackle small workspaces first (g3rs-garde-types, g3ts-astro-media-types, g3ts-arch-file-tree-checks) to confirm patterns.
- Save largest (mdx, media-config, content-config) for after pattern is locked in.
- For doc strings: keep concise, one short line. Match prevailing style in sibling fixed crates.
