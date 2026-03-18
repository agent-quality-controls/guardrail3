# Create Adversarial Edge Case Fixtures + MANIFEST.md

**Date:** 2026-03-15 22:27
**Task:** Create 10 edge-case test fixtures and a manifest for all 50+ adversarial grep-attack fixtures.

## Goal
Create `tests/fixtures/grep-attacks/edge-cases/` with 10 files testing parser robustness (empty files, BOM, CRLF, long lines, nested attributes, syntax errors, etc.) and a MANIFEST.md listing all fixtures from all 5 agents.

## Approach

### Step-by-step plan
1. Create `tests/fixtures/grep-attacks/edge-cases/` directory
2. Write all 10 fixture files with carefully crafted content
3. Create `tests/fixtures/grep-attacks/MANIFEST.md` listing all 50+ fixtures with expected behavior

### Key decisions
- **BOM file:** Write raw bytes using printf to get actual UTF-8 BOM
- **CRLF file:** Use printf to get actual \r\n line endings
- **Very long line:** Generate 10,000 char string literal programmatically
- **Empty file:** Use touch for 0 bytes

## Files to Modify
- `tests/fixtures/grep-attacks/edge-cases/empty_file.rs` — 0 bytes
- `tests/fixtures/grep-attacks/edge-cases/only_comments.rs` — comments only
- `tests/fixtures/grep-attacks/edge-cases/unicode_bom.rs` — UTF-8 BOM prefix
- `tests/fixtures/grep-attacks/edge-cases/crlf_line_endings.rs` — Windows line endings
- `tests/fixtures/grep-attacks/edge-cases/very_long_line.rs` — 10K char line
- `tests/fixtures/grep-attacks/edge-cases/nested_cfg_attr.rs` — deeply nested attributes
- `tests/fixtures/grep-attacks/edge-cases/multiple_allows_one_line.rs` — multiple lints
- `tests/fixtures/grep-attacks/edge-cases/attribute_on_expression.rs` — allow on let in block
- `tests/fixtures/grep-attacks/edge-cases/syntax_error_midway.rs` — partial valid file
- `tests/fixtures/grep-attacks/edge-cases/no_main_lib.rs` — lib-style file
- `tests/fixtures/grep-attacks/MANIFEST.md` — full manifest
