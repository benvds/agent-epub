# Milestone 3 verification

Date: 10 November 2026. Release checkpoint: `v0.3.0`.

Selected scope: simple semantic Markdown tables, linked footnotes and nested section contents. Existing file input, version 1 requests, streamed inline content and covers remain supported. Full EPUB support is outside scope.

## Implementation and checks

| Area | Evidence |
| --- | --- |
| Tables | Column headers have `scope="col"`; cell formatting, Unicode, escaping, links and alignment are preserved. Ragged rows fail with a source line before output replacement. |
| Notes | Source-local, case-insensitive labels; numbering by first reference; one definition at the end of the first referring chapter; separate backlinks for repeated and cross-chapter references. Tests include independent labels in different sources. |
| Safe failures | Duplicate, empty and unused definitions and references inside notes are rejected. Existing outputs survive failures. Missing/incorrect note targets and return links in damaged packages are detected. |
| Contents | Root H2–H6 headings become nested entries, including duplicate labels and skipped levels. Quoted, list and note headings are excluded. Damaged target, hierarchy and completeness cases fail; legacy flat contents still validate. |
| Agent contract | Direct file and streamed-inline version 1 requests produce identical report contents and chapter resources. Discovery includes detailed Markdown rules. JSON results and errors retain their existing schema. |
| Automated regressions | 33 ordinary Rust tests pass. Formatting and Clippy with warnings denied pass. Two ignored independent-validation tests are run separately. |
| Independent conformance | Official EPUBCheck 5.3.0, EPUB 3.3 rules: eleven plain/Markdown/mixed books plus four cover books. New table, note, section and combined-report fixtures are included; the combined report is checked with both PNG and JPEG covers. |

The OpenAI live evaluation recorded for milestone 2 remains historical evidence for that release. Milestone 3 adds CLI contract tests, not a new live model score. Claude and Ollama evaluations remain deferred at the user's request.

## Reader observations

The synthetic [report fixture](../../examples/research-report.md) was opened in two independent reading implementations.

| Reader | Observed result |
| --- | --- |
| Apple Books 9.0, build 6724, macOS | Rendered the table with aligned numeric values and all cells. The visible contents showed nested Findings / Observations / Method / Detail entries and the second chapter. Page navigation reached both chapters. Clicking the first note opened the native popup with both paragraphs, its chapter link and all three return links. |
| EPUB.js 0.3.93, JSZip 3.10.1, in-app browser | Parsed all seven chapter/section entries. Selecting the deep Detail entry reached its page. Note references and the third return link were activated across chapters; the report returned to the second chapter and its reference. Table headers were exposed as column headers. |
| EPUB.js narrow reading area | At 360 pixels and 180% text, prose reflowed and the sample two-column table measured approximately 280 pixels wide. Deep section navigation still reached the selected heading. |

Books automation could not reliably activate a return link inside its popup; cross-chapter returns were exercised in EPUB.js. EPUB.js displayed notes in reading content, while Books used a popup. Note presentation depends on the reader. The narrow browser test is not a physical mobile-device test. Wide tables, long labels and individual publications still need review; these observations do not establish accessibility certification or universal reader compatibility.

## Performance

The optimized release benchmark generated 100,000 base prose words in 40 chapters, plus 40 tables, 40 notes, 80 note references, 80 section entries and a 9,722,608-byte PNG cover. Thirty runs measured a median of 35.353 ms and a 95th percentile of 39.856 ms. The slowest sample was 214.490 ms.

Timing includes process startup, cover decoding, generation, built-in validation and publication. It excludes external EPUBCheck, source construction and transfers. Repeated prose and warm filesystem caches make this a bounded local measurement. [Raw measurements](milestone-3-benchmark.json).

Reproduce with:

```sh
cargo build --release --locked
cargo run --release --locked --example evaluation_covers -- /tmp/epub-covers --benchmark
python3 scripts/benchmark.py target/release/agent-epub \
  --conveniences --cover /tmp/epub-covers/benchmark-cover.png
```

## Release boundary

Tables require one header row, equal cell counts and escaped literal pipes. Merged/multiline cells, captions and complex header relationships are unsupported. Tables inside lists, quotations and notes fail. Footnote definitions must be outside containers; unknown note markers without definitions remain literal Markdown text. Source-relative links inside notes use the same rules as chapter links.

Inline images, custom styles, richer metadata, task lists, raw HTML, batch generation, deterministic archives, fixed layout, multimedia and DRM remain deferred. No additional full-EPUB claims are made. The release gate runs formatting, Clippy, regressions and the independent conformance corpus in CI before the tag is pushed.
