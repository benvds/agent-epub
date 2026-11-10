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

Implementation and release verification are pending. Local baseline: all 33
ordinary Rust tests pass; the two ignored independent EPUBCheck tests are run
separately. Record native installation reports and CI links here before
declaring this milestone complete.
