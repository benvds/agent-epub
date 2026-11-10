# Milestone 4 verification

Date: 10 November 2026. Release checkpoint: `v0.4.0`.

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

All native packaging and clean-install gates passed in
[CI run 42782531302](https://github.com/benvds/epub-rust/actions/runs/42782531302)
on commit `144a1c5ff4f2c4338c435d85af56acb170fa9605`. Formatting, Clippy,
regressions and both independent conformance corpora passed in the same run.

| Target | Observed fresh-runner environment | Installation result |
| --- | --- | --- |
| macOS Apple Silicon | macOS 15.8.2, arm64 | Passed |
| Linux x86_64 | Ubuntu 24.04 runner, glibc 2.39, x86_64 | Passed |
| Windows x86_64 | Windows Server 2025, build 26100, AMD64 | Passed |

The installed binary ran with an empty tool directory as PATH, with no Cargo,
Rust compiler or Java discoverable. Python drives the verification harness; it
is not invoked by the executable. Discovery, ordered-file input, file requests
and streamed-inline requests succeeded. All three modes produced exactly the
bundled sample's chapter and navigation resources. Checks inspect Unicode,
semantic table headers, three note references and seven ordered chapter/section
contents targets. Missing validators fail both build and validate; unsupported
content and output conflicts fail safely. Prior output remains byte-for-byte
intact. A corrupted ZIP fails checksum verification before installation.

The sample EPUBs extracted from all three CI archives also passed official
EPUBCheck 5.3.0 locally, and their chapter/navigation resources match across
platforms. The README and skill's self-contained inline examples were executed
successfully. Skill validation passes; these are CLI workflow checks rather than
a fresh live-agent evaluation.

The tag workflow repeats the native gates before publishing. The
[v0.4.0 release](https://github.com/benvds/epub-rust/releases/tag/v0.4.0)
attaches the final archives, individual `.sha256` files, `SHA256SUMS` and three
`<target>-smoke.json` reports identifying each published ZIP's exact hash.
Initial CI archive hashes are not substituted for the final release hashes.
CI packages only the explicit allowlist, generates the report sample with the
release executable and includes locked dependency and standard-library notices. No benchmark was
rerun because parsing, rendering, compression and validation behavior did not
change. No new reader appearance claims are made; milestone 3 observations
remain historical evidence for the identical reading resources.
