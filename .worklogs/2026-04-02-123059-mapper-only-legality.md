# Mapper receives only LegalityFacts — no tree, no structure

**Date:** 2026-04-02 12:30

## Summary

Complete pipeline leak closure: walker → structure → legality → mapper.

- Structure consumes ProjectTree by value
- Legality consumes StructureFacts by value, carries it in LegalityFacts
- Mapper receives &LegalityFacts only — no tree, no structure references
- Mapper Cargo.toml no longer depends on domain-project-tree
- scoped_files.rs migrated from &ProjectTree to &RustStructureFacts

## Pipeline verification

```
cargo check -p guardrail3-app-rs-legality  ✓ (no tree dep)
cargo check -p guardrail3-app-rs-family-mapper  ✓ (no tree dep)
```

Zero ProjectTree references in mapper code or Cargo.toml.

## Changes
- `legality/src/lib.rs`: collect takes RustStructureFacts by value, stores in output
- `family_mapper/src/rs.rs`: FamilyMapper stores only &LegalityFacts, self.structure
  replaced with self.legality.structure(), self.tree replaced similarly
- `family_mapper/src/scoped_files.rs`: takes &RustStructureFacts instead of &ProjectTree
- `family_mapper/Cargo.toml`: domain-project-tree dependency removed
- `runtime/src/lib.rs`: structure consumed by value, legality consumed by value

## Remaining breaks
Runtime context still has tree field. Runners still reference ctx.tree.
All families still import RsProjectSurface (removed). These are downstream.
