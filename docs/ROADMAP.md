# Agent EPUB tool roadmap

Updated 9 November 2026. Deliver a useful, tested subset in three milestones. Dates are intentionally unset; each milestone ends when its acceptance gates pass.

## Milestone 1 Text and Markdown reading editions

Status: implementation and automated checks complete at `v0.1.0`; manual two-reader review remains open. See the [verification record](verification/MILESTONE_1.md).

Create reflowable EPUBs from plain text or Markdown, with a table of contents and reliable chapter navigation.

Deliver:

- Ordered UTF-8 source files, including mixed plain text and Markdown.
- Predictable chapter boundaries, preserved preambles and duplicate-title handling.
- Book title, language, optional author and generated publication identifier.
- A visible contents page and reader-native chapter contents.
- Previous/next chapter links and a return-to-contents link.
- Basic headings, paragraphs, emphasis, lists, quotations, code and links.
- A readable default style, structural checks, clear failures and safe output replacement.
- Usage examples and an explicit supported-Markdown profile.

Images, general structured agent requests, tables, notes, embedded markup and full EPUB feature coverage are deferred.

Exit gates:

1. Plain text and Markdown each generate complete books; mixed-source order is preserved.
2. Contents and adjacent-chapter navigation are correct, including duplicate headings, empty-heading labels and preamble text.
3. Supported fixtures pass independent EPUBCheck validation; built-in checks catch broken references and malformed packages.
4. Failure tests confirm that invalid input and failed generation preserve existing output.
5. Representative books are checked in two independent readers, with the versions and results recorded. If reader access is unavailable, record that gate as outstanding.
6. The generation benchmark is measured against the PRD target; results and remaining limitations are recorded.

## Milestone 2 General agent inputs and cover images

Status: in progress. Generic requests, streamed input, discovery, structured results and JPEG/PNG covers are implemented and tested. Live cross-agent evaluation and reader review remain open. See the [agent interface](AGENT_INTERFACE.md).

Let different agents create books through one predictable, unattended workflow, and add a supplied cover image.

Deliver:

- A versioned book request with metadata, explicit ordered chapters and inline content or file references.
- Request input from a file or a stream, without provider-specific conversation parsing.
- Capability discovery and compact structured success, warning and error results.
- Explicit behavior for missing information, conflicting options and repeated requests.
- Supplied JPEG or PNG covers, correct cover identification and image validation.
- Examples and task evaluations for ChatGPT, Claude and a capable Ollama-backed agent.

Exit gates:

1. All milestone 1 regression checks continue to pass.
2. Equivalent direct-file and structured requests produce equivalent reading content and navigation.
3. Invalid requests, paths and cover files produce stable, actionable outcomes.
4. Covers work in independently validated files and tested readers.
5. Each agent environment meets the PRD completion target, with versions and results recorded.

## Milestone 3 Selected conveniences

Choose a small set of additions based on actual book-generation needs. Full EPUB support remains outside scope.

Candidate order:

| Priority | Candidate | Evidence needed before inclusion |
| --- | --- | --- |
| 1 | Simple semantic tables and linked notes | Repeated demand from reports, learning material or nonfiction. |
| 2 | Inline images with supplied alternative text | A concrete guide or illustrated-book workflow. |
| 3 | Deeper contents, richer metadata and a few style presets | Agent feedback showing that current defaults are insufficient. |
| 4 | Automated accessibility reports and review guidance | Agreed coverage and a clear distinction from certification. |
| 5 | Batch generation and reproducible output | A measured repeated-generation workflow that benefits from them. |

Exit gates for each chosen addition:

- It has a documented input boundary, fallback behavior and failure examples.
- It preserves content and passes the existing navigation and conformance corpus.
- It has relevant reader checks and does not undermine the speed targets.
- Its value justifies the added agent instructions and maintenance burden.

## Progress and checkpoints

The implementation plan breaks each milestone into small commits. Commit and push the planning documents first, then commit runnable foundations, generation behavior and verification separately. Keep unfinished acceptance gates visible rather than declaring a milestone complete from code tests alone.

For current scope and implementation sequence, see the [PRD](PRD.md) and [Rust CLI implementation plan](IMPLEMENTATION_PLAN.md).
