# Capture golden snapshots + write AST migration plan

**Date:** 2026-03-15 22:14
**Scope:** Golden baselines for 4 external projects + syn/tree-sitter migration plan

## Summary
Captured golden validation snapshots against 4 real projects (2,370 checks total). Wrote migration plan to replace grep-based source checks with syn (Rust) and tree-sitter (TypeScript) AST parsing. Identified 11 grep-should-fail fixtures that prove the migration improves correctness.
