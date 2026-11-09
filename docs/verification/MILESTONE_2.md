# Milestone 2 verification

Date: 9 November 2026. Release checkpoint: `v0.2.0`.

Milestone 2 adds general agent requests and supplied JPEG/PNG covers. The release scope includes the live OpenAI-agent evaluation. Claude and Ollama evaluations are deferred at the user's request and are not release gates for this version.

## Completed implementation checks

| Area | Evidence |
| --- | --- |
| Agent inputs | Request files, streamed requests, inline content, file references, relative bases, capability discovery and structured results are tested. |
| Equivalent input modes | Direct and structured file requests produce identical cover, contents and chapter resources. Publication identifiers and modification metadata are allowed to differ. |
| Covers | PNG/JPEG data is detected and fully decoded; original bytes are preserved. Cover identification, alternative text and first-in-spine order are checked. |
| Failures | Corrupt, empty, oversized and unsupported images, invalid alternative text, ambiguous requests and output conflicts produce errors. Previous outputs remain intact. |
| Regressions | Milestone 1 source, navigation, package and safe-failure tests continue to pass. |
| Independent conformance | The original seven-book corpus and the additional PNG/JPEG cover corpus pass official EPUBCheck 5.3.0 using EPUB 3.3 rules. |
| Live agent use | ChatGPT-authenticated Codex CLI 0.167.0-alpha.3.2 completed all 20 synthetic cases on the first build requests. Independent scoring validated the first artifacts, not the agent's self-report. |

The effective OpenAI model identity was not exposed in recorded client events. The measured result applies to the recorded CLI environment and corpus, rather than a named model or the ChatGPT GUI. [Evaluation method and results](AGENT_EVALUATIONS.md).

## Reader checks

| Reader | Observed behavior |
| --- | --- |
| Apple Books 9.0, build 6724, macOS | Opened the generated publication; displayed the PNG cover with its alternative text; showed all five native contents entries, including repeated titles and Unicode; rendered literal source text and later chapter content. Cover and page navigation were exercised. |
| EPUB.js 0.3.93 with JSZip 3.10.1 in the in-app browser | Loaded both PNG and JPEG books. Both cover images decoded at their original 900 by 1,350 dimensions and fitted the reader. Repeated-title contents entries reached distinct chapters. Generated next/previous links and a cross-chapter heading link were activated successfully. |
| EPUB.js phone-width check | A 360-pixel reading area with 180% text reflowed the chapter heading, paragraph and lists. PNG and JPEG covers retained their aspect ratio at approximately 330 and 321 pixels wide respectively. |

The browser fixture used the same five-chapter content as the repository's mixed-source example. Its images were synthetic test fixtures. The native reader's accessibility automation could not reliably activate every individual HTML link; those link actions were verified in EPUB.js. A phone-sized browser area is not a physical mobile-device test. No accessibility certification or universal reader compatibility is claimed.

EPUB.js was used as an independent reading implementation, separately from the generator and EPUBCheck. [Official reader project](https://github.com/futurepress/epub.js).

## Performance

The local optimized benchmark generated a 100,000-body-word, 40-chapter book with a 9,722,608-byte PNG cover for 30 runs. The recorded 95th percentile is 35.055 ms, including process startup, cover decoding, generation, built-in checks and final output publication. External EPUBCheck and transfers are excluded.

The text fixture uses repeated words and warm filesystem caches. This is a bounded local measurement, not a general latency promise. [Raw measurements](milestone-2-benchmark.json), [benchmark script](../../scripts/benchmark.py).

## Release boundary

The tool remains focused on a reflowable text/Markdown subset with an optional cover. Inline images, tables, notes, raw HTML, fixed layout, multimedia and DRM remain outside this release. Claude and Ollama live evaluations remain deferred; their results must be recorded before claiming tested support in those environments.

The tagged release is validated through formatting, Clippy, regression tests and the independent EPUBCheck corpus in CI. [Agent interface](../AGENT_INTERFACE.md), [roadmap](../ROADMAP.md).
