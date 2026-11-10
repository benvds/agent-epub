# Agent EPUB tool product requirements

Status: product scope agreed; delivery progress is tracked in the roadmap. Updated 10 November 2026.

The tool helps AI agents turn supplied writing into dependable EPUBs quickly. An agent should provide content, receive a usable reading edition, and understand any problems without learning EPUB publishing internals.

The released subset covers text and Markdown, ordered chapters, nested contents, tables, linked notes, optional covers, structured requests/results and repair diagnostics. Milestones 1 through 5 are delivered through v0.5.1, and milestone 5a has a completed practical comparison. Full EPUB feature coverage remains outside the product goal; the [roadmap](ROADMAP.md) records delivery evidence and remaining decisions.

## Users and needs

- An assistant converts a report or guide into a reading edition for its user.
- A writing agent assembles ordered chapters into a book without changing the writing.
- A locally operated agent produces an EPUB from supplied material without needing another AI service.
- A person reviews the result, follows its contents and chapter links, and adjusts reading settings in their reader.

ChatGPT, Claude and an Ollama-backed agent are intended consumers. Compatibility means a tested workflow with a capable agent environment, not a promise about every model or hosting configuration.

## Problem and evidence

Existing workflows can produce useful EPUBs, but producing a file does not establish its completeness, conformance or reading quality. The local research demonstrates chapters, navigation, formatting and source links, with a limited validation scope. It does not establish a universal ChatGPT EPUB exporter. [Current capabilities](research/chatgpt-epub-capabilities.md)

The online comparison shows separate strengths in conversion, book formatting, editing and checking. This product should make a small, well-tested subset easy for agents to use and make the result's quality clear. [Tool comparison](research/epub-tools-analysis.md)

## Product principles

Preserve the supplied writing and its order. Use sensible presentation defaults. Finish ordinary requests without interactive questions once required information is present. Explain unsupported features and failures clearly. Keep the supported feature set small enough to test thoroughly.

“Robust” means that the declared subset produces structurally sound books with complete content, consistent navigation and explicit check results. “Fast” means measured completion time for complete requests, including the checks promised for that request.

## Functionality by milestone

| Requirement | Milestone 1 | Milestone 2 | Milestone 3 selected scope |
| --- | --- | --- | --- |
| Plain text and Markdown generation | Required | Retained | Retained |
| Multiple chapters in supplied order | Required | Retained | Retained |
| Table of contents | Required, one entry per chapter | Retained | Nested H2–H6 section contents |
| Chapter navigation | Required, contents links and previous/next links | Retained | Footnote references and return links |
| Basic Markdown formatting | Required, documented subset | Retained | Simple tables and linked footnotes |
| Publication title and language | Required | Retained | Existing metadata retained |
| General agent inputs | Direct source files initially | Required, reusable structured book requests and streamed input | Existing inputs retained; batch deferred |
| Agent-readable results and capability discovery | Basic help and clear outcomes | Required, consistent structured results and discovery | Discoverable table, note and section rules |
| Cover image | Deferred | Required, supplied image | Existing covers retained; inline images deferred |
| Content checks and honest validation status | Required | Retained | Section hierarchy and note-return checks; accessibility reports deferred |
| Simple tables and notes | Deferred | Deferred | Required within the documented subset |
| Inline images and style choices | Deferred | Deferred | Deferred |
| Full EPUB, multimedia and fixed layout | Outside scope | Outside scope | Outside scope |

The [roadmap](ROADMAP.md) defines the delivery gates. Milestone 3 selects tables, notes and section contents from the original shortlist; the remaining candidates stay deferred.

## Milestone 1 requirements

| ID | Requirement | Acceptance condition |
| --- | --- | --- |
| P1 | Create an EPUB from UTF-8 plain text or Markdown. | Both formats work independently and in an ordered collection of files. Empty or unreadable sources produce clear errors. |
| P2 | Preserve supported content and chapter order. | All supported text appears once and in order. Special characters remain intact. Plain text is treated literally. |
| P3 | Make chapter boundaries predictable. | Each source file begins a chapter. Top-level Markdown headings can begin additional chapters. Text before the first heading remains in the book. |
| P4 | Include a readable table of contents. | It lists every chapter in reading order, including chapters with repeated titles. Each entry reaches the intended chapter. |
| P5 | Support chapter navigation. | Reader contents navigation works; visible previous/next links connect adjacent chapters and each chapter links back to contents. |
| P6 | Apply a readable default style. | Headings, paragraphs, emphasis, lists, quotations and code remain legible when text size changes. The supported Markdown subset is documented. |
| P7 | Handle metadata predictably. | A title and language are supplied or explicitly resolved from documented defaults. The tool supplies a unique publication identifier without inventing author or rights information. |
| P8 | Check every generated publication. | Package structure, resource references, reading order and internal links are checked. The result identifies which checks ran. |
| P9 | Protect previous results. | Invalid inputs or failed generation leave an existing output unchanged. Replacing a successful output requires an explicit choice. |
| P10 | Define the support boundary. | Unsupported images, embedded markup or extensions cannot silently disappear. The agent receives a useful limitation and remedy. |

The initial conformance target is an EPUB 3.3 reflowable publication using this subset. Supporting a subset of content features does not relax the applicable publication requirements. [W3C EPUB 3.3](https://www.w3.org/TR/epub-33/)

## Milestone 2 requirements

| ID | Requirement | Acceptance condition |
| --- | --- | --- |
| P11 | Accept general agent-supplied book requests. | A request can supply metadata, ordered chapters, content or source references, and a cover. It does not depend on vendor-specific conversation formats. |
| P12 | Work without interactive prompts. | Agents can discover requirements, submit a complete request and retrieve a result through their available tools. |
| P13 | Return predictable, compact results. | Success and failure include a status, artifact location where applicable, check coverage and actionable findings. No hidden conversation state is required. |
| P14 | Include a supplied cover image. | Supported image types are declared; the cover is packaged, identified correctly and readable in tested readers. Invalid images produce actionable errors. |
| P15 | Demonstrate agent usability. | Release evaluation uses the ChatGPT-authenticated OpenAI Codex CLI. Claude and Ollama evaluations are deferred at the user's request; their live compatibility is not claimed. The generic request contract remains the intended interface for all capable agent runtimes. |

An agent supplies final writing. The product does not need to generate prose, call an AI model, or understand a provider's private conversation history.

## Milestone 3 selected requirements

| ID | Requirement | Acceptance condition |
| --- | --- | --- |
| P16 | Preserve simple report tables. | Rectangular Markdown tables retain all cells, inline formatting and alignment, with semantic column headers. Ragged rows fail with a source location. Complex tables stay outside scope. |
| P17 | Make notes easy to follow. | Source-local footnotes are numbered by first reference, shared across repeated references and linked back to every referring location. Empty, duplicate, unused and nested notes fail explicitly. Unmatched markers remain literal text under the documented Markdown rules. |
| P18 | Navigate longer chapters. | Root-level H2–H6 headings appear as nested contents entries. Skipped levels and duplicate labels remain usable. Quoted, list and note headings stay out of the contents. |

These conveniences work through the existing file and general agent inputs. Inline images, style customization, expanded metadata, batch generation and reproducible archives remain deferred.

## Milestones 4, 5 and 5a requirements

These requirements retain the original content and output-protection promises.
They describe the delivered product boundary without prescribing its implementation.

| ID | Requirement | Acceptance condition |
| --- | --- | --- |
| P19 | Install and reuse the export interface. | Named native release targets include checksums, examples and agent instructions, with recorded installation checks that do not require a source toolchain. |
| P20 | Make failures repairable without losing existing callers. | Stable detailed issue codes, source references, available locations and remedies supplement existing result categories and exit codes. |
| P21 | Declare Markdown interpretation and metadata boundaries. | Recognized front matter has an explicit outcome. Rendered and literal extensions are distinguishable; ordinary dollar signs and tildes do not cause blanket warnings. |
| P22 | Choose chapter boundaries explicitly. | Direct and structured inputs can select root heading levels 1–6. The default remains H1; sources, preambles, heading ranks, contents and links remain consistent. |
| P23 | Compare a complete real-agent export workflow. | Identical completed manuscripts, requested structures and validation requirements are measured against a prepared alternative, with cold setup and warm export reported separately. |
| P24 | Keep the practical comparison inspectable and bounded. | Three paired workflows retain failures, timing, actual tool events, exposed usage, artifact snapshots and independent checks. Claims stay within the measured corpus and user resource limits. |

See [milestone 5 verification](verification/MILESTONE_5.md) and the
[practical comparison](verification/MILESTONE_5A.md). Milestone 6 reviews evidence
and GitHub feedback before conditional features are selected.

## Quality and result handling

The release corpus must pass independent EPUBCheck validation. Runtime results must distinguish built-in structural checks from independent conformance validation and from checks that did not run. A structurally checked file must not be described as independently certified. [EPUBCheck](https://www.w3.org/publishing/epubcheck/)

Failures identify the source or chapter, the problem and a next step. Missing metadata, unresolved internal links, unsupported content and output conflicts must be distinguishable. A failed operation must never be reported as a completed book.

A successful book still needs appropriate human review before publication. Later accessibility checks must distinguish automated findings from a complete accessibility assessment. A clean automated report alone does not establish WCAG conformance. [DAISY guidance](https://kb.daisy.org/publishing/docs/epub/validation/ace.html)

Reader testing should cover at least two independent reading systems, including one desktop reader and one device or mobile reading application. Record the tested versions, contents behavior, chapter navigation and enlarged-text behavior. Current evidence covers Apple Books and EPUB.js, including a narrow browser layout; physical-device or separate mobile-app review remains outstanding coverage. Automated conformance and manual reader checks serve different purposes.

## Proposed success measures

These remain acceptance targets. Measured outcomes are recorded separately in the milestone verification records and [workflow benchmark](verification/MILESTONE_5A.md); a local observation is not a general performance guarantee.

| Measure | Target and conditions |
| --- | --- |
| Supported-content preservation | No dropped, duplicated or reordered text in the acceptance corpus. |
| Navigation correctness | Every contents and adjacent-chapter link resolves in every accepted fixture. |
| Standards conformance | Every accepted fixture passes EPUBCheck with no errors; warnings are reviewed. |
| Milestone 1 generation time | 95th percentile below 5 seconds for up to 100,000 words and 40 chapters, including built-in checks and process startup. |
| Milestone 2 generation time | 95th percentile below 10 seconds for the same text plus one supplied cover up to 10 MB. |
| Independent validation time | Report separately when run; never hide skipped validation to meet a speed target. |
| Milestone 2 agent completion | At least 95% of supported tasks succeed on the first complete request in each tested agent environment. |

Publish the benchmark corpus, reference machine and number of runs with results. Exclude source creation and file transfer from generation timing. Compare successful outputs with the same required check scope.

## Boundaries and open decisions

Fixed layout, video, audio, synchronized narration, interactive books, DRM, retailer submission and lossless editing of arbitrary existing EPUBs are outside this roadmap. PDF conversion and complex document import are also outside the initial scope.

Tables, notes and nested contents are delivered. Remaining conditional candidates need evidence from the benchmark or a concrete user workflow before selection; EPUB format support alone does not justify expanding scope.

Before declaring a milestone complete, record independent-reader evidence and measured performance. Cross-agent integration details belong to milestone 2. Packaging, implementation and language choices belong in the separate [implementation plan](IMPLEMENTATION_PLAN.md).

## References

- [Current ChatGPT capabilities and evidence](research/chatgpt-epub-capabilities.md).
- [Official EPUB tool comparison](research/epub-tools-analysis.md).
- [Current release verification](verification/MILESTONE_5.md).
- [Practical agent workflow comparison](verification/MILESTONE_5A.md).
- [W3C EPUB 3.3](https://www.w3.org/TR/epub-33/).
- [EPUBCheck](https://www.w3.org/publishing/epubcheck/).
- [DAISY accessibility-checking guidance](https://kb.daisy.org/publishing/docs/epub/validation/ace.html).
