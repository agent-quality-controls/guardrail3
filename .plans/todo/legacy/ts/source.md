# TS-SOURCE — TypeScript source file checker (13 rules)

**Input:** *.ts / *.tsx files (tree-sitter parsed)
**Current code:** `ts/validate/source_scan.rs`, `ts_comment_checks.rs`

## Comment suppression rules

| New ID | Old ID | What | Description | Status |
|--------|--------|------|-------------|--------|
| TS-SOURCE-01 | T23 | eslint-disable (block, no reason) | Block `/* eslint-disable */` without reason | Implemented |
| TS-SOURCE-02 | T24 | eslint-disable (block, with reason) | Block eslint-disable with reason (inventory) | Implemented |
| TS-SOURCE-03 | T25 | eslint-disable (line, no reason) | Line `// eslint-disable-next-line` without reason | Implemented |
| TS-SOURCE-04 | T26 | eslint-disable (line, with reason) | Line eslint-disable with reason (inventory) | Implemented |
| TS-SOURCE-05 | T27 | @ts-ignore | `@ts-ignore` suppresses type checking (always error) | Implemented |
| TS-SOURCE-06 | T28 | @ts-expect-error (no reason) | `@ts-expect-error` without explanation | Implemented |
| TS-SOURCE-07 | T29 | @ts-expect-error (with reason) | `@ts-expect-error` with explanation (inventory) | Implemented |

## Code quality rules

| New ID | Old ID | What | Description | Status |
|--------|--------|------|-------------|--------|
| TS-SOURCE-08 | T30 | process.env | Direct `process.env` access (use env module) | Implemented |
| TS-SOURCE-09 | T31 | any type | `: any` or `as any` usage | Implemented |
| TS-SOURCE-10 | T32 | file length | File exceeds 400 effective lines | Implemented |
| TS-SOURCE-11 | T34 | noinspection | IDE `noinspection` suppression | Implemented |
| TS-SOURCE-12 | T35 | coverage ignore | Coverage ignore directive (`istanbul ignore`, `c8 ignore`) | Implemented |
| TS-SOURCE-13 | T59 | banned packages | Banned packages in node_modules | Implemented |
