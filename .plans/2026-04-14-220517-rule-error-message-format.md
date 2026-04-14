# Goal
Make rule error messages short, specific, and reusable so future rule work does not drift back into vague jargon.

# Required Format
Every rule message must contain exactly these 3 parts, in this order:

1. What is wrong
2. What to do
3. Why

If any of the 3 parts is missing, the message is not done.

# Message Template
Use this shape:

`<specific thing in specific file> is wrong. <specific fix in specific place>. <specific reason this improves the system>.`

Do not use vague words like:
- real proof site
- owned module
- local helper
- proper shape
- invalid pattern
- better structure

Replace them with concrete names:
- test function name
- file path
- helper file name
- assertions file path
- crate name
- dependency name
- exact module name

# Writing Rules
- Name the exact bad thing.
- Name the exact file or function.
- Name the exact destination or fix.
- Explain the reason in plain words.
- Keep it short.

# Good Pattern
`Test "{name}" in "{file}" checks results through local file "{helper_file}". Move the result assertions into the shared assertions crate file "{assertions_file}" and call that from the test instead, so internal and external tests use the same proof.`

# Bad Pattern
`Test "{name}" lacks a real proof site and must use the owned assertions module/crate.`

# Why This Exists
Agents kept writing titles and messages that were technically related to the rule but too vague to tell what was actually wrong or how to fix it. Future rule work should follow this file instead of reinventing message style each time.
