# Agent EPUB tool product requirements

Status: product scope agreed; delivery progress is tracked in the roadmap. Updated 9 November 2026.

The tool helps AI agents turn supplied writing into dependable EPUBs quickly. An agent should provide content, receive a usable reading edition, and understand any problems without learning EPUB publishing internals.

The initial release focuses on plain text and Markdown, with a table of contents and working chapter navigation. Later releases add general agent inputs, a cover image and selected conveniences. Full EPUB feature coverage is outside the product goal.

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

| Requirement | Milestone 1 | Milestone 2 | Milestone 3 candidates |
| --- | --- | --- | --- |
| Plain text and Markdown generation | Required | Retained | Retained |
| Multiple chapters in supplied order | Required | Retained | Retained |
| Table of contents | Required, one entry per chapter | Retained | Optional deeper heading navigation |
| Chapter navigation | Required, contents links and previous/next links | Retained | Additional navigation conveniences |
| Basic Markdown formatting | Required, documented subset | Retained | Extend only with clear demand |
| Publication title and language | Required | Retained | Richer optional metadata |
| General agent inputs | Direct source files initially | Required, reusable structured book requests and streamed input | Optional batch convenience |
| Agent-readable results and capability discovery | Basic help and clear outcomes | Required, consistent structured results and discovery | Improve from real agent feedback |
| Cover image | Deferred | Required, supplied image | Optional image conveniences |
| Content checks and honest validation status | Required | Retained | Broader automated accessibility review |
| Simple tables and notes | Deferred | Deferred | Evaluate for inclusion |
| Inline images and style choices | Deferred | Deferred | Evaluate for inclusion |
| Full EPUB, multimedia and fixed layout | Outside scope | Outside scope | Outside scope |

The [roadmap](ROADMAP.md) defines the delivery gates. Milestone 3 is a prioritized shortlist, not a commitment to every candidate.

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

## Quality and result handling

The release corpus must pass independent EPUBCheck validation. Runtime results must distinguish built-in structural checks from independent conformance validation and from checks that did not run. A structurally checked file must not be described as independently certified. [EPUBCheck](https://www.w3.org/publishing/epubcheck/)

Failures identify the source or chapter, the problem and a next step. Missing metadata, unresolved internal links, unsupported content and output conflicts must be distinguishable. A failed operation must never be reported as a completed book.

A successful book still needs appropriate human review before publication. Later accessibility checks must distinguish automated findings from a complete accessibility assessment. A clean automated report alone does not establish WCAG conformance. [DAISY guidance](https://kb.daisy.org/publishing/docs/epub/validation/ace.html)

Reader testing should cover at least two independent reading systems, including one desktop reader and one device or mobile reading application. Record the tested versions, contents behavior, chapter navigation and enlarged-text behavior. Automated conformance and manual reader checks serve different purposes.

## Proposed success measures

These are acceptance targets, not measured product performance.

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

Milestone 3 candidates need evidence of user value before inclusion. Prioritize simple tables or notes if agents repeatedly need them; do not add features solely because the EPUB format allows them.

Before declaring a milestone complete, record independent-reader evidence and measured performance. Cross-agent integration details belong to milestone 2. Packaging, implementation and language choices belong in the separate [implementation plan](IMPLEMENTATION_PLAN.md).

## References

- [Current ChatGPT capabilities and evidence](research/chatgpt-epub-capabilities.md).
- [Official EPUB tool comparison](research/epub-tools-analysis.md).
- [W3C EPUB 3.3](https://www.w3.org/TR/epub-33/).
- [EPUBCheck](https://www.w3.org/publishing/epubcheck/).
- [DAISY accessibility-checking guidance](https://kb.daisy.org/publishing/docs/epub/validation/ace.html).
