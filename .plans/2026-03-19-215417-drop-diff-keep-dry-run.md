# Drop diff command, keep --dry-run + --dump-dir on generate

**Date:** 2026-03-19 21:54
**Task:** Remove redundant diff subcommand, move --dump-dir to generate --dry-run

## Changes
1. cli.rs: remove Diff variant from RsCommands and TsCommands
2. cli.rs: add --dump-dir to GenerateArgs
3. main.rs: remove Diff dispatch, update --dry-run to pass dump_dir
4. diff.rs: keep the functions, just called from generate --dry-run
5. CLAUDE.md: update command table
