# Agent EPUB v0.5.1

This patch fixes a panic when a local link targets a supplied Markdown source
that contains only footnote definitions and therefore produces no chapter. The
build now returns exit code 2, clean JSON with `unresolved_link`, its source
location and a remedy, while preserving existing output.

Valid-book reading resources and the version 1 request/result contract are
unchanged from v0.5.0. The practical three-pair benchmark remains pinned to
v0.5.0; its raw measurements are retained in the repository.

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
Claude and Ollama live evaluations remain deferred. The completed
[practical three-pair benchmark](https://github.com/benvds/agent-epub/blob/main/docs/verification/MILESTONE_5A.md)
compares v0.5.0 with a prepared Pandoc 3.13 wrapper. Both passed all shared tasks;
Agent EPUB was faster in the observed cold workflows, while warm export results
were mixed. These small local measurements do not establish a universal speed
or reliability advantage.

Extract the ZIP for your platform and follow `docs/INSTALL.md`. Verify its
`.sha256` file or `SHA256SUMS`. Native smoke reports identify the exact archive
hash. Platform/runtime boundaries remain those documented in installation.
The repository remains private; downloading requires repository access.
