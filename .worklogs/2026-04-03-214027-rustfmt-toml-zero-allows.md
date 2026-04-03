# Remove all lint allows from rustfmt-toml

**Date:** 2026-04-03 21:40

## Summary
Flipped all 9 clippy allow lints to deny. Fixed 4 issues (3 missing module
docs, 1 empty line after doc comment). Removed 9 escape hatches from
guardrail3.toml. Package now has zero allows, zero escape hatches, zero
clippy warnings.
