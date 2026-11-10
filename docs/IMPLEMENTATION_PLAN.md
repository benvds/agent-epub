# Rust CLI implementation plan

Updated 10 November 2026. Implement the [roadmap](ROADMAP.md) as a standalone CLI named `agent-epub`. Milestones 1 through 5 and the practical milestone 5a comparison are complete; their sequences below are the implementation record. Milestone 4 passes native release verification under MIT for macOS Apple Silicon, Linux x86_64 and Windows x86_64 at `v0.4.0`. Milestone 5 passes local checks, two-reader review and native release gates through `v0.5.1`. Milestone 5a records a completed practical three-pair comparison; D1 local images are delivered for v0.6.0 with two-reader verification, and milestone 6 plans further evidence review. The [PRD](PRD.md) stays independent of these programming choices.

## Milestone 1 Implementation sequence

### Checkpoint 1 Project foundation

Create a Rust binary and library with CLI argument handling, shared errors and a documented usage example. Commit the dependency lockfile. Keep generated EPUBs and build outputs out of Git.

Proposed command:

```sh
agent-epub build --input introduction.txt --input chapters.md \
  --output book.epub --title "Example book" --language en \
  --author "Example author"
```

Require title and language, accept an optional author, preserve repeated input order and infer format only from supported file extensions. Reject ambiguous extensions instead of guessing. UTF-8 is the initial text encoding. `--overwrite` explicitly permits replacing an output; it must not allow replacing an input source.

Define stable exit categories: success, invalid arguments/content, I/O failure, publication-check failure and external-validator failure or unavailability. Write diagnostics to stderr. Reserve structured machine output for milestone 2.

Deliver a compiling executable, help text, argument tests and a checkpoint commit.

### Checkpoint 2 Source normalization and chapter model

Create a shared book model containing metadata, ordered chapters, heading identifiers and unresolved links. Retain source paths and line/offset information for diagnostics. Read and validate all sources before opening the destination.

Plain-text policy:

- Each source file becomes one chapter, named from its filename.
- Blank lines separate paragraphs; preserve line breaks within paragraphs.
- Escape text literally, including angle brackets, ampersands and Markdown-looking characters.

Markdown policy:

- Use CommonMark parsing with a documented subset: headings, paragraphs, emphasis, strong emphasis, lists, quotations, code, thematic breaks and links.
- Each source starts a chapter; top-level headings split further chapters. Preserve any preamble as its own chapter. A source without a top-level heading remains one chapter.
- Reject images and raw HTML with an actionable unsupported-feature error. Detect table and note extensions explicitly, rather than quietly interpreting requested advanced formatting as ordinary text.
- Generate unique chapter filenames and heading identifiers, including repeated or non-Latin headings. Give empty chapter headings a predictable visible fallback.
- Resolve links to supplied sources and heading fragments after all chapters are known. Allow ordinary web and email links; reject unresolved local destinations and unsupported schemes.

Reject empty books, invalid XML characters and unsupported content before publishing. Preserve all supported text. Do not fetch remote resources.

Commit source handling with focused tests for preambles, mixed sources, duplicate titles, Unicode and link resolution.

### Checkpoint 3 EPUB generation

Write XML-safe XHTML from the parsed content. Generate navigation and reading order from the same chapter model, so the visible contents, reader navigation and chapter sequence agree.

Package at least:

```text
mimetype
META-INF/container.xml
EPUB/package.opf
EPUB/nav.xhtml
EPUB/style.css
EPUB/chapter-0001.xhtml
EPUB/chapter-0002.xhtml
...
```

The EPUB `mimetype` entry must be first, uncompressed and contain exactly `application/epub+zip`. Constrain ZIP settings to EPUB-compatible options. Generate correct namespaces, metadata, manifest, spine, language declarations and modification metadata for an EPUB 3.3 publication. Include a navigation entry for every chapter. Do not add legacy NCX unless reader testing demonstrates a need.

Include the visible contents page in the spine. Add previous/next chapter links at chapter boundaries and a return link to contents. Package one conservative stylesheet with relative typography and wrapping for long text and code.

Use a temporary output in the destination directory. Finish and check it before publishing. Default publication must fail if the target exists; explicit overwrite replaces it only after success. Prevent output paths that alias input paths, including symbolic-link aliases.

Commit generation once a minimal book is usable and its package tests pass.

### Checkpoint 4 Checks, independent validation and documentation

Reopen generated archives and verify:

- ZIP integrity, entry uniqueness, mimetype placement and expected package location.
- XML well-formedness, required namespaces and metadata.
- Manifest resources, spine references and chapter order.
- Contents destinations, all generated navigation links and heading fragments.
- Absence of unsupported unbundled local references.

Expose `agent-epub validate book.epub` for these structural checks. It is scoped to the generated profile and must not claim full conformance. Provide an optional EPUBCheck invocation through `--epubcheck <executable>`; the validator must be invoked without shell interpolation. Missing or unsuccessful validators must not be reported as a pass. During build, optional independent validation happens before publication of the output.

For independent release validation, run EPUBCheck against every accepted fixture. Keep synthetic fixtures only; do not copy private source publications into Git. Test CLI behavior as a subprocess, including output conflicts, unsupported features and a simulated validator failure. Run formatting, linting and the Rust test suite.

Measure generation on a synthetic 100,000-word, 40-chapter book, with process startup and built-in checks included. Keep external-validator timing separate. Record actual values and environment; do not convert a local measurement into a general guarantee.

Record manual reading results separately. If two readers cannot be exercised, mark that roadmap gate outstanding. Commit verification, examples, documentation and the measured status.

## Proposed components and dependency choices

| Component | Responsibility | Proposed dependency |
| --- | --- | --- |
| CLI | Commands, options and help | [clap](https://docs.rs/clap/latest/clap/) |
| Input and book model | Source validation, metadata and chapter ordering | Standard library and explicit domain types |
| Markdown | Parse supported source structure | [pulldown-cmark](https://docs.rs/pulldown-cmark/latest/pulldown_cmark/) |
| XML/XHTML | Escape content, write and inspect package documents | [quick-xml](https://docs.rs/quick-xml/latest/quick_xml/) or a dedicated XML parser where namespace validation is needed |
| ZIP package | Store mimetype and compress publication resources | [zip](https://docs.rs/zip/latest/zip/) with restricted compression features |
| Output publication | Temporary file and safe final replacement | [tempfile](https://docs.rs/tempfile/latest/tempfile/) |
| Identifier | Generate a publication identifier | [uuid](https://docs.rs/uuid/latest/uuid/) |

Select compatible released versions during setup and commit the lockfile. Dependency support for extra features does not make those features part of the product. Keep parsing, rendering, packaging and checking separable for meaningful tests, without introducing a service or plugin system.

## Milestone 2 Implementation sequence

1. Define a versioned book-request schema and matching structured result format. Use [serde_json](https://docs.rs/serde_json/latest/serde_json/) for request/result handling.
2. Add `build --request book.json` and streamed request input. Define precedence explicitly: structured requests and direct content flags are mutually exclusive. Resolve relative file references from the request file's directory; require an explicit base for streamed requests containing file references.
3. Add `capabilities` and machine-readable results. Separate status, output location, warnings, checks, errors and timing. Keep logs off structured stdout. Reject unknown fields where they could hide mistakes; identify schema versions and unsupported versions clearly.
4. Add JPEG/PNG covers, content-based image verification and bounded size/dimension checks. Evaluate [image](https://docs.rs/image/latest/image/) with only required formats enabled. Package the image and cover page with correct cover metadata and reading order.
5. Publish the same small examples for all target agents. Run recorded tasks against ChatGPT, Claude and an Ollama-backed agent with a tool-capable model. Preserve the generic contract rather than adding provider-specific conversation parsers.

Suggested commit boundaries: request contract; streamed inputs and paths; structured results/discovery; covers; cross-agent evaluation and documentation.

## Milestone 3 Implementation sequence

Selected release scope: simple Markdown tables, linked footnotes and nested contents. Keep the version 1 request contract and direct CLI inputs unchanged.

1. Add rectangular pipe tables with semantic column headers and alignment. Reject excess or missing cells instead of losing text. Test content, escaping and safe failures; commit a working checkpoint.
2. Collect footnote definitions per source, number by first reference, place notes at the end of the first referring chapter, and link every reference back. Reject duplicate, unused and nested definitions; keep source-relative links working. Test repeated and cross-chapter references; commit a working checkpoint.
3. Add nested contents for top-level H2–H6 headings, attaching skipped levels to the nearest preceding shallower heading. Exclude quoted/list/note headings. Verify the complete contents order and targets; commit a working checkpoint.
4. Update discovery, examples and usage. Run the regression suite, independent EPUBCheck corpus, reader checks and speed benchmark. Record limitations, finalize the release version, commit, then tag and push after CI passes.

Simple tables should preserve header relationships and all cell content. Notes should preserve references and return navigation. Inline images require supplied alternative text or a clear decorative-image choice. Style presets must remain readable with user font settings. Automated accessibility reports must identify their coverage and remaining manual checks.

Consider batch processing and deterministic output only after real repeated-generation needs are demonstrated. Do not introduce fixed layout, multimedia, DRM or full EPUB support through incidental parser options.

## Milestones 4 through 6 Implementation checkpoints

Use the work-package IDs, estimates, dependencies and acceptance gates in the [roadmap](ROADMAP.md). Keep the next implementation scope small enough to verify independently. Future release versions are not assigned yet.

### Checkpoint A Adoption and release packaging

1. A1: update the README around current capabilities and a complete streamed-request example. Include source and generated sample instructions, artifact handling and exact validation coverage.
2. A2: record the maintainer's license choice and complete the corresponding project/package metadata. Do not choose reuse terms implicitly while implementing an installer.
3. A3: add a release archive/checksum workflow for selected targets. Verify build compatibility and installation in each claimed environment. Keep runtime-independent generation separate from optional external EPUBCheck setup.
4. A4: package a short agent skill using the same examples. Verify discover, build, inspect and report behavior against the released binary. Preserve the explicit deferral of Claude and Ollama live evaluations.

Suggested commits: documentation/examples; license and metadata after the maintainer's choice; release packaging; integration instructions and clean-install verification. Release archives must not include private manuscripts or local evaluation outputs.

### Checkpoint B Input rules and structured repair

Milestone 5 implementation record (10 November 2026): preserve result `code`
and exit categories, adding `issue_code`, `source`, one-based Unicode `line` /
`column` where available, and `remedy` without changing schema version 1.
Reject leading YAML (`---`) and TOML (`+++`) front-matter blocks; import no
metadata. Math, strikethrough and alert markers retain literal CommonMark
behavior without blanket punctuation warnings. Add book-wide `split_level`
(integer 1–6, default 1), exposed as `--split-level` for direct inputs and a
request property for structured inputs. Split only at root headings of exactly
that level, preserve source boundaries and preambles, and retain heading ranks.
A leading H1 or matching split-level heading supplies the chapter title and is
excluded from nested section entries. Release verification and the later
three-pair benchmark are recorded separately in
[MILESTONE_5.md](verification/MILESTONE_5.md) and
[MILESTONE_5A.md](verification/MILESTONE_5A.md).


1. B1: retain broad error categories while adding stable detailed codes, source references, locations where available and remedies. Define result compatibility before changing serialized output. Test unsupported structures and unresolved links as distinct failures.
2. B2: use the [historical research probes](research/market-research-probes-2026-11-10.json) as input cases, then establish current expected behavior. Tables and notes are supported since milestone 3, so their old rejection results are not new acceptance expectations. Cover recognized front matter, math, strikethrough, alerts and ordinary literal punctuation. Update discovery and usage together.
3. B3: design a compatible split-level option for direct and request inputs. Preserve the current default. Test title-plus-H2 manuscripts, source boundaries, preambles, nested contents and link targets through equivalent input modes.

Suggested commits: diagnostic contract and representative errors; Markdown declarations/front-matter policy; split-level behavior; verification and current examples. Do not introduce aggregate preflight or front-matter metadata import incidentally; those require their own declared behavior.

### Checkpoint C1 Agent workflow benchmark after milestone 5

Completed as three practical paired workflows at the maintainer's request. The
[protocol](../benchmarks/workflow/PROTOCOL.md),
[report](verification/MILESTONE_5A.md) and retained events/artifacts record the
actual scope and limits. The sequence below remains the implementation plan.

1. Immediately after milestone 5, implement the milestone 5a benchmark. Assemble identical completed manuscripts and export requirements for Agent EPUB and a reasonable Pandoc wrapper. Cover supported reports and separate recovery/unsupported cases. Record the integration-recipe preparation effort for both tools.
2. Instrument the real agent client with the same model/settings, permissions and host resources. Pin versions and exact prompts. Capture setup/discovery/configuration calls, model turns, build attempts, retries, manual interventions, timing and available usage data. Retain artifact snapshots for independent scoring rather than relying on agent self-reports.
3. Run clean-environment setup-to-first-success and already-installed export scenarios separately. Both tools receive equivalent documentation, caching opportunities and validation requirements. Reuse source files; treat inline content as its own scenario rather than requiring manuscript duplication.
4. Predeclare an equal attempt/time cap, paired repeats, alternating execution order and a resource budget within the user's limits. Use isolated sessions. Independently check content, metadata, order, navigation and EPUBCheck before counting a successful result. Include failed and timed-out tasks in the report.
5. Publish sanitized raw measurements and a reproducible report comparing time to a correct EPUB, tool calls, retries and first-attempt success. Break down setup, model interaction, conversion and validation time. Report faster, slower or inconclusive outcomes without presuming a winner.

Suggested commits: shared task corpus and scoring rules; agent instrumentation and cold/warm environments; paired runs, measurements and conclusions. This plans the benchmark; it does not authorize an unbounded evaluation run or resume deferred Claude/Ollama evaluations.

### Evidence review and GitHub issues

1. After C1, review benchmark findings and feedback reported through [GitHub issues](https://github.com/benvds/agent-epub/issues). Request version, command/request, expected and actual behavior, and a minimal non-private reproduction when needed. No user recruitment or observation study is required.
2. Reuse C1 scoring and extend competitor comparisons to mdbindery only when repository-based workflows justify it.
3. Publish results against the roadmap's proposed decision targets. Record a decision for each conditional candidate and whether the evidence favors the engine, an integration improvement or a narrower audience.

Suggested commits: reproducible issue fixtures; any selected comparison extension; revised priorities. Actual provider evaluations remain subject to the existing user-approved scope.

### Selected D1 checkpoint: Local inline images

The maintainer prioritized this package after the practical benchmark. Commit
this scope checkpoint before implementation. Use a 0.6.0 development version so
capability caches cannot confuse the new image behavior with released v0.5.1.

1. Share bounded JPEG/PNG inspection with covers; add canonical-path inline asset
   loading, explicit source bases, alternative/decorative rules and localized
   errors. Keep remote fetching and other image formats outside scope.
2. Integrate Markdown images with chapter/footnote rendering, package original
   bytes and correct manifest resources, and extend generated-profile validation
   to check all image resources and alternative text. Protect asset inputs from
   overwrite and preserve existing request/result compatibility.
3. Test input-mode equivalence, image/cover coexistence, reuse, source-relative
   paths, limits, missing/corrupt resources and damaged-package rejection.
4. Add synthetic illustrated examples, update discovery and current docs, run
   regressions and EPUBCheck, and record reader/narrow-layout observations.
   Commit working implementation and verification checkpoints on main; release
   publication remains separate from this implementation request.

D1 implementation checkpoint: bounded loading and shared static-image inspection,
rendering/packaging/validation, localized errors, capability discovery, 49 passing
regression tests, three independent conformance corpora, illustrated examples and
native archive smoke are implemented. All seven required native CI jobs pass
on `243b406` across the three supported targets. Browser narrow-layout review found and
fixed intrinsic table-image overflow. The v0.6.0 showcase passes Apple Books
image, contents, popup-note and enlarged-text review after the Mac was unlocked. See the
[D1 record](verification/INLINE_IMAGES.md) for exact evidence and timing.

### Conditional work packages

Do not start every conditional candidate as one release. Select the smallest package justified by C1, GitHub issues or a documented earlier workflow. Local images need bounded resource loading and reader checks. Aggregate preflight depends on the detailed diagnostics and input rules. Reproducible archives require explicit identity/time settings and archive metadata checks. A preservation manifest should reuse the independent comparison scoring rather than imply that counts prove meaning.

Styles and preview fixtures need reader checks with font overrides and narrow layouts. Accessibility reporting should integrate an existing checker with explicit coverage. Richer metadata, batch operation, a GitHub Action and an MCP adapter each need a specific consuming workflow before implementation. Keep adapters thin and preserve the CLI as the common contract.

## Verification and completion record

| Gate | Required evidence |
| --- | --- |
| Source fidelity | Synthetic text, Unicode, Markdown, preamble, duplicate-heading and mixed-source tests. |
| Navigation | Contents, previous/next, same-file and cross-source links, and missing-fragment tests. |
| Package validity | Reopened-package checks plus independent EPUBCheck results. |
| Safe failure | Existing-output preservation, input/output alias protection and failed-validator tests. |
| Speed | Reproducible synthetic workload and measured wall-clock result. |
| Reading behavior | Two independently tested readers with recorded versions, or an explicit outstanding gate. |

Commit the documentation checkpoint first. During implementation, commit after each working checkpoint and push progress when authorized. Keep the roadmap status consistent with evidence, including any manual gates that remain open.

References: [EPUB 3.3](https://www.w3.org/TR/epub-33/), [EPUBCheck](https://www.w3.org/publishing/epubcheck/), and the primary dependency documentation linked above.
