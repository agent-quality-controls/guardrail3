# Make R35 type-aware and R-GARDE-05 smarter

**Date:** 2026-03-16 21:53
**Task:** R35 should be Error on non-primitive fields, silent on primitives. R-GARDE-05 should not require Validate on all-primitive structs.

## Goal
- R35: `#[garde(skip)]` on primitive types (bool, numeric) produces no result. On non-primitives, produces Error.
- R-GARDE-05: Only require Validate if struct has at least one non-primitive field.

## Approach

### Step 1: Add GardeSkipInfo and type-aware visitor
In `ast_helpers.rs`, add `GardeSkipInfo` struct and `find_garde_skips_with_types()` function.
In `ast_visitors.rs`, create `GardeSkipTypedVisitor` that collects field name, type, and primitive status.

### Step 2: Add primitive check helpers
Add `is_primitive_type()` helper and `PRIMITIVE_TYPES` constant. Handle `Option<T>` unwrapping.

### Step 3: Update allow_checks.rs R34/R35
Replace `find_garde_skips` with `find_garde_skips_with_types`. Primitives → silent. Non-primitives without comment → R34 Error (unchanged). Non-primitives with comment → R35 now Error with specific message about field name/type.

### Step 4: Add field type info to DeriveInfo for R-GARDE-05
Add helper `struct_has_non_primitive_fields()` that checks if a struct has any non-primitive fields. Update `count_unvalidated_input_structs` to skip all-primitive structs.

### Step 5: Tests
Add 5 new tests as specified.

## Files to Modify
- `apps/guardrail3/src/app/rs/validate/ast_visitors.rs` — new GardeSkipTypedVisitor
- `apps/guardrail3/src/app/rs/validate/ast_helpers.rs` — GardeSkipInfo, find_garde_skips_with_types, primitive helpers
- `apps/guardrail3/src/app/rs/validate/allow_checks.rs` — update check_garde_skip to be type-aware
- `apps/guardrail3/src/app/rs/validate/garde_checks.rs` — update count_unvalidated_input_structs
- `apps/guardrail3/tests/unit/allow_checks_test.rs` — new R35 tests
- `apps/guardrail3/tests/unit/test_garde_checks.rs` — new R-GARDE-05 test
