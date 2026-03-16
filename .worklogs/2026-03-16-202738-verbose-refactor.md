# --verbose flag + decompose 20 too_many_lines functions

**Date:** 2026-03-16 20:27
**Scope:** text/markdown reporters, 20+ source files refactored

## Summary
Added --verbose flag: without it, audit trails (R33/R35/R36) with >3 items are summarized to one line. Report went from 218 lines to 41. Decomposed 20 functions with too_many_lines into smaller sub-functions. R33 allows dropped from 153 to 122 (31 too_many_lines allows eliminated).
