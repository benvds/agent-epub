# Agent EPUB v0.4.0

Install Agent EPUB without Rust on macOS Apple Silicon, Linux x86_64 and Windows
x86_64. This release adds MIT licensing, native release ZIPs and SHA-256
checksums, dependency notices, current installation instructions, a generated
sample report with its sources and a reusable agent skill.

Each archive is tested on a separate native runner using the packaged executable
with no Rust or Java on its PATH. Attached smoke reports identify the archive
hash and observed platform. They cover capability discovery, direct file input,
file requests, streamed inline requests, report content and navigation, built-in
validation, failed/unrun check reporting and existing-output preservation.

Generation and the version 1 request/result contract are unchanged. Tables,
linked footnotes and nested section contents remain supported. Ordinary tests
and the independent EPUBCheck corpus gate publication. Reader appearance and
accessibility require separate review. Claude and Ollama live evaluations remain
deferred; this release does not claim new provider evaluations.

Extract the archive for your target and follow its `docs/INSTALL.md`. The Linux
binary targets Ubuntu 24.04/glibc 2.39 or later; it is not a musl/static build.
macOS is tested on version 15 with Apple Silicon; Intel Macs are not included.
Windows is tested on Server 2025 x86_64 with the C runtime statically linked.
macOS binaries are unsigned and not notarized. Java and official EPUBCheck are
optional, separately installed dependencies. Generation needs neither.

Verify the ZIP against its `.sha256` file or `SHA256SUMS` before installing.
Checksums are transfer checks, not code signatures. The repository remains
private; downloading requires repository access.
