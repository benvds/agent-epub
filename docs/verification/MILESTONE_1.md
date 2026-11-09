# Milestone 1 verification

Date: 9 November 2026. Release checkpoint: `v0.1.0`.

The milestone 1 implementation generates EPUB reading editions from ordered plain-text and Markdown sources, with visible contents, reader-native contents and adjacent-chapter navigation. The automated implementation checks are complete. Manual reading-system review remains open.

## Completed checks

| Check | Evidence |
| --- | --- |
| Rust compilation | Debug and optimized release builds succeeded using Rust 1.95.0 on macOS arm64. |
| Formatting and linting | `cargo fmt --check` and Clippy with warnings treated as errors passed. |
| Automated regression tests | Six source-handling tests and eight integration/CLI tests passed. |
| Source boundaries | Tests cover preambles, duplicate/empty headings, mixed sources, literal plain text, Unicode, quoted headings and internal links. |
| Failure behavior | Tests cover invalid metadata/encoding, unsupported content, output conflicts, source aliases and failed external validation. |
| Damaged publications | Mutated ZIP packages with missing destinations, broken spine references, invalid modification metadata, malformed XML and missing fragments are rejected. |
| Independent EPUB validation | The dedicated corpus test passed using official EPUBCheck 5.3.0 and its EPUB 3.3 rules. It generated six synthetic single-source books plus the mixed-source example. |
| Initial mixed-source example | EPUBCheck explicitly reported zero fatals, errors, warnings and infos. |
| Performance | Thirty optimized CLI runs generated a 100,000-body-word, 40-chapter synthetic book. Local median: 17.015 ms; 95th percentile: 26.318 ms; maximum: 251.725 ms. |

The benchmark includes process startup, reading, packaging, built-in checking and publication. The fixture uses repeated words and warm filesystem caches, so it is a narrow speed check rather than a production workload distribution. External validation, source creation and transfers are excluded. [Raw measurements](milestone-1-benchmark.json), [reproduction script](../../scripts/benchmark.py).

## Remaining review

The application inventory showed Apple Books, but opening it for verification failed with a `cgWindowNotFound` automation error. No successful reader UI inspection occurred. Two-reader navigation and enlarged-text review remain unverified; no accessibility or visual-quality certification is claimed.

The roadmap's manual reader gate remains open. `v0.1.0` marks the completed implementation and automated verification checkpoint, rather than completion of that manual gate. Reader review can proceed independently while milestone 2 development begins.

Cross-agent usability, general book requests and cover images belong to milestone 2. [Usage](../USAGE.md), [roadmap](../ROADMAP.md).
