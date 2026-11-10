# Milestone 4 verification

Date: 10 November 2026. Target release: `v0.4.0`.

## Agreed scope

The maintainer selected MIT licensing, with `Agent EPUB contributors` as the
copyright holder. Binary targets are macOS Apple Silicon
(`aarch64-apple-darwin`), Linux x86_64 (`x86_64-unknown-linux-gnu`) and Windows
x86_64 (`x86_64-pc-windows-msvc`). Intel macOS and other architectures are not
release targets. The maintainer authorized a branch push, CI and publication
of v0.4.0 after checks pass.

A1 updates installation, current documentation, streamed input, structured
results and synthetic report examples. A2 supplies the MIT license and package
metadata. A3 builds versioned archives and checksums, then tests their contents
on separate native runners without Rust on the subprocess PATH. A4 bundles a
short reusable agent skill and exercises discovery, the three input modes,
artifact inspection and failed/unrun checks against the packaged executable.

No generation features or request/result schema changes are planned. Claude
and Ollama live evaluations remain deferred. Reader observations for unchanged
reading resources remain in the milestone 3 record.

## Acceptance evidence

Local checks pass: 34 ordinary Rust tests, formatting, Clippy with warnings
denied, and both independent EPUBCheck 5.3.0 corpora (eleven text/Markdown/mixed
fixtures and four cover fixtures). The skill validator passes. A packaged
macOS archive passes discovery, all three input modes, exact reading-resource
equivalence, table/note/contents inspection, output protection and failed/unrun
check reporting with Rust and Java absent from PATH.

Windows uses `same-file` to protect source hard-link aliases, replacing the
previous Windows fallback that never recognized them. A cross-platform regression
checks the source and output alias remain intact. The absolute-output assertion
now uses the platform's path rules. Generation and request/result schemas are
otherwise unchanged.

Native CI installation reports and publication are pending. Do not mark this
milestone complete until all three downloaded archives pass on fresh runners.
CI packages only the explicit allowlist, generates the report sample with the
release executable and includes locked dependency and standard-library notices.
Each attached smoke report identifies the exact ZIP hash. No benchmark was
rerun because parsing, rendering, compression and validation behavior did not
change. No new reader appearance claims are made; milestone 3 observations
remain historical evidence for the identical reading resources.
