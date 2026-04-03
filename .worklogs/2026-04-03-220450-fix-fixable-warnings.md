# Fix fixable warnings on rustfmt-toml

**Date:** 2026-04-03 22:04

## Summary
Fixed 5 escape hatches that were fixable:
- Removed unwrap_used allow: tests now use .expect() and helper fn
- Removed missing_assert_message allow: all asserts have messages
- Removed redundant_pub_crate allow: tried pub, triggers unreachable_pub — genuinely unavoidable, restored with reason
- Removed EXCEPTION comments from deny.toml: tightened regex ban (no wrappers needed for 2-dep crate), yanked = "deny"
- Implemented FromStr trait: removed should_implement_trait allow

6 remaining warnings are all genuinely unavoidable for a config parser crate.
