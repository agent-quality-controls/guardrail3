# g3rs-workspace-crawl TODO

- Symlink policy is not finalized yet. The first implementation skips symlink entries instead of modeling them explicitly.
- The package currently records basic readability by attempting to open files and list directories. If later ingestion packages need finer-grained recoverability diagnostics, extend the crawl types deliberately rather than smuggling ad hoc booleans into ingestion.
- The first query surface is intentionally small. Add more queries only when an ingestion package demonstrates a concrete need.
