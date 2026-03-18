# Migration Orchestrator — Read This First

## What This Is

A self-executing migration plan. An orchestrator agent reads this file and executes each step in sequence. Each step is a separate plan file in this directory. Steps are numbered and must be executed in order.

## How to Execute

For each step `NN_name.md`:
1. Read the step file
2. Execute the task (spawn agents as described)
3. Run the verification commands listed in the step
4. If verification passes → mark step as DONE, move to next
5. If verification fails → read the "On Failure" section, fix, re-verify
6. After ALL steps complete → run the convergence loop (step 99)

## Step List

| Step | File | What | Depends on |
|------|------|------|-----------|
| 01 | `01_adversarial_fixtures.md` | Write 50+ adversarial test fixtures | Nothing |
| 02 | `02_capture_before.md` | Run current tool against fixtures, save "before" results | 01 |
| 03 | `03_add_syn_dependency.md` | Add syn to Cargo.toml, create parsing helpers | Nothing |
| 04 | `04_add_treesitter_dependency.md` | Add tree-sitter to Cargo.toml, create parsing helpers | Nothing |
| 10 | `10_migrate_R30_R31.md` | Migrate crate-level allow checks to syn | 03 |
| 11 | `11_migrate_R32_R33.md` | Migrate item-level allow checks to syn | 03, 10 |
| 12 | `12_migrate_R34_R37.md` | Migrate garde skip + cfg_attr allow to syn | 03, 11 |
| 13 | `13_migrate_R38_R44.md` | Migrate structure + code quality checks to syn | 03, 12 |
| 14 | `14_migrate_R58.md` | Migrate direct std::fs check to syn | 03, 13 |
| 20 | `20_migrate_T23_T29.md` | Migrate eslint-disable + ts-ignore to tree-sitter | 04 |
| 21 | `21_migrate_T30_T35.md` | Migrate process.env + any + file length to tree-sitter | 04, 20 |
| 30 | `30_verify_golden.md` | Run golden tests against all 5 projects, compare | 14, 21 |
| 31 | `31_verify_adversarial.md` | Run adversarial fixtures, verify improvements over "before" | 14, 21, 02 |
| 32 | `32_adversarial_review.md` | Launch adversarial agents to find remaining issues | 31 |
| 33 | `33_fix_issues.md` | Fix whatever 32 found | 32 |
| 99 | `99_convergence_loop.md` | Repeat 30-33 until no issues found | 33 |

## Rules for Each Step

1. **Max 10 specific changes per agent task.** Don't send an agent to do 50 things.
2. **Every step has a verification command.** If it doesn't pass, the step isn't done.
3. **Commit after every successful step.** Don't accumulate uncommitted changes.
4. **Golden tests are the source of truth.** If golden tests break, the migration introduced a regression.
5. **Adversarial fixtures are the correctness proof.** If a fixture that should fail doesn't, the check is still buggy.

## Completion Criteria

The migration is DONE when:
1. All golden snapshots match (or differences are documented as intentional improvements)
2. All adversarial fixtures produce correct results
3. Adversarial review finds 0 new issues
4. All existing 218+ tests pass
5. Mutation testing kill rate >= previous baseline
