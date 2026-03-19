# Drop diff command, keep generate --dry-run

**Date:** 2026-03-19 21:58
**Scope:** cli.rs, main.rs, CLAUDE.md, 3 test files

## Summary
Removed redundant `rs diff` / `ts diff` subcommands. `generate --dry-run` does the same thing. Moved `--dump-dir` to GenerateArgs. Updated all tests from `rs diff` to `rs generate --dry-run`.
