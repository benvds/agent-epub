# Agent EPUB v0.5.0

Milestone 5 adds predictable chapter boundaries and actionable repair diagnostics.

- Choose root heading levels 1–6 with `--split-level` for direct files or the
  optional `split_level` version 1 request property. H1 remains the default.
  Title-plus-H2 guides preserve their preamble, heading ranks, repeated titles,
  nested contents, links and footnote returns.
- JSON failures keep existing error categories and add stable `issue_code`,
  source references, available one-based Unicode locations and `remedy` fields.
  Human stderr diagnostics include repair hints. Structured stdout stays clean.
- Recognized leading YAML/TOML front matter fails explicitly; metadata is not
  imported. Math, strikethrough and alert markers retain documented literal
  CommonMark behavior without blanket punctuation warnings.
- Capability discovery, schema, help, documentation, packaged agent instructions
  and equivalent direct/file/inline H2 examples describe the same rules.

Formatting, Clippy, regression tests, independent EPUBCheck 5.3.0 corpora and
reader checks cover this release. Native CI builds and clean-install smoke
checks gate the release archives for macOS Apple Silicon, Linux x86_64 and
Windows x86_64. MIT licensed; generation remains offline and requires no Rust,
Java or AI service. Optional EPUBCheck is installed separately. See
`docs/verification/MILESTONE_5.md` in the repository for exact evidence and limits.
Accessibility and individual publication appearance need separate review.
Claude and Ollama live evaluations remain deferred. Milestone 5a's comparative
agent benchmark is separate future work.

Extract the ZIP for your platform and follow `docs/INSTALL.md`. Verify its
`.sha256` file or `SHA256SUMS`. Native smoke reports identify the exact archive
hash. Platform/runtime boundaries remain those documented in installation.
The repository remains private; downloading requires repository access.
