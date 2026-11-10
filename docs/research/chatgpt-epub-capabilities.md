# ChatGPT EPUB capabilities and current workflows

Historical research snapshot. Current released support is in the [feature matrix](epub-feature-matrix.md), and the later [practical agent benchmark](../verification/MILESTONE_5A.md) records the measured Pandoc comparison. The original findings below retain their dated scope.

Research date: 9 November 2026. Scope: current capabilities, with evidence separated from possible functionality.

ChatGPT can use code execution to produce an EPUB, but the evidence does not establish one universal EPUB exporter or a guaranteed feature set. Python is a verified route in a local builder. Pandoc is another route prescribed by a locally installed skill. Neither finding proves which route a particular ChatGPT conversation uses.

## What the evidence establishes

| Evidence | Finding | Boundary |
| --- | --- | --- |
| Official OpenAI Code Interpreter documentation | Models can write and execute Python, create files, and return file references. | This documents an API tool, not ChatGPT's internal EPUB implementation. |
| Official ChatGPT file documentation | ChatGPT supports creating and reviewing generated files. | The page names document, spreadsheet, presentation, PDF and some other previews; it does not establish an EPUB preview or EPUB feature contract. |
| One existing local EPUB builder | Python uses `lxml` to produce XHTML/XML and `zipfile` to package an EPUB. | Its origin in a particular ChatGPT session, model or product surface is unverified. |
| Six existing local EPUBs | All six declare EPUB package version 3.0 and contain XHTML navigation, NCX navigation and CSS. | These are convenience samples from one local project, not representative product testing. |
| Installed active-learning instructions | The EPUB workflow calls for Pandoc, title and author metadata, and a table of contents. | Instructions establish intended behavior, not a successful conversion or availability everywhere. |

Sources: [OpenAI Code Interpreter](https://developers.openai.com/api/docs/guides/tools-code-interpreter), [ChatGPT work with files](https://learn.chatgpt.com/docs/artifacts-viewer), local builder (identified by its hash in the audit snapshot), installed active-learning EPUB instructions, [audit snapshot](epub-evidence.json).

## How generation works in the observed builder

The builder reads a prepared chapter list containing titles, filenames and HTML fragments. Preparing those fragments from original documents happens upstream and is not demonstrated by this script.

It then:

1. Normalizes HTML elements into XHTML namespaces and adds language declarations and a shared stylesheet.
2. Rewrites selected document links to packaged chapters. Some absolute local links become links to an explanatory sources section.
3. Replaces tables with labeled records for narrow screens. This transformation assumes a particular header and body structure.
4. Adds a title page and nested EPUB navigation. It also creates a separate NCX document with a flatter outline.
5. Creates publication metadata, a resource manifest and the ordered reading sequence, called the spine.
6. Packages those resources in a ZIP archive. The first entry is the uncompressed `mimetype` file.
7. Checks archive integrity, XML parsing, internal destinations, anchors and selected table-cell text. It writes one XHTML preview for visual inspection.

This is a custom document transformation and packaging workflow. It does not call EbookLib or Pandoc. The script stores publication text in memory, uses publication-specific paths and wording, and creates a random identifier. It is not evidence of a reusable ChatGPT EPUB subsystem.

The separate skill workflow delegates conversion to Pandoc. Pandoc's EPUB tutorial demonstrates assembling chapters and applying metadata, a cover and styling. These are Pandoc capabilities; the local instructions do not prove that every option is used. [Pandoc ebook tutorial](https://pandoc.org/epub.html)

The likely general pattern is that an agent chooses available tools, prepares the content, generates the package, checks it and returns a file. This is an inference from the evidence above, not an account of proprietary ChatGPT internals.

The audit snapshot records the inspected builder hash. Private source publications and their text are not copied into this repository.

## Functionality inventory

Status meanings:

- **Observed:** present in the inspected local source or artifacts. This is demonstrated local functionality, not a ChatGPT product guarantee.
- **Documented tool capability:** established by the named tool's official documentation, with no evidence that ChatGPT invokes it by default.
- **Unverified:** not established by this research. This does not mean impossible or unsupported.

### Content and appearance

| Functionality | Status | Evidence and practical limit |
| --- | --- | --- |
| Multiple chapters and explicit reading order | Observed | All six packages have multiple spine entries. |
| Combining documents and appendices | Observed | Builder iterates documents containing chapters. |
| Title page and reading guidance | Observed | Builder creates a publication-specific title page. |
| Headings, paragraphs, bold and italic | Observed | Present in sampled XHTML. |
| Ordered and unordered lists | Observed | Present in sampled XHTML. |
| Block quotations | Observed | Two blockquote elements across the sample. |
| Tables | Observed | One remaining HTML table across the sample; reader rendering was not tested. |
| Table conversion for narrow screens | Observed | Builder transforms rows into labeled records and checks selected cell text. Complex tables are not covered. |
| Definition lists | Observed | Present in the transformed content. |
| Custom CSS and typography | Observed | All samples package a stylesheet. Reader behavior can differ. |
| Reflowable reading | Observed structure | Samples contain ordinary XHTML and no fixed-layout declaration. Actual resizing was not tested. |
| Unicode and language declarations | Observed | Dutch language metadata and multilingual characters occur in the builder. Broader script coverage is untested. |
| Images, cover images and embedded fonts | Documented tool capability | Pandoc documents these options. None occurs in the six inspected packages. |
| Footnotes and scholarly citations | Documented tool capability | Pandoc documents footnotes and citation processing; no semantic footnote elements were established in this sample. |
| Code highlighting and mathematical content | Unverified for ChatGPT EPUB output | No relevant sample or generation test was examined. |

### Navigation, metadata and packaging

| Functionality | Status | Evidence and practical limit |
| --- | --- | --- |
| EPUB 3 navigation document | Observed | Every package declares one navigation resource. |
| Nested table of contents | Observed | Builder constructs nested document and chapter links. |
| NCX navigation for older readers | Observed | Every sample includes NCX. This does not make the publication an EPUB 2 publication. |
| Internal chapter links and anchors | Observed | Audit resolved the XHTML references it inspected. |
| External source links | Observed | 367 external XHTML references across six files; websites were not checked for availability. |
| Local document link rewriting | Observed | Custom rules in the builder; some replacements change the destination's meaning. |
| Title, language, identifier and dates | Observed | Builder writes these metadata fields. |
| Description metadata | Observed | Builder writes a description. |
| Author, cover and richer publication metadata | Documented tool capability | Pandoc supports metadata configuration; no universal ChatGPT defaults are established. |
| Manifest and spine generation | Observed | Builder constructs both. |
| Correct basic ZIP container arrangement | Observed | All samples have the expected first uncompressed mimetype entry and passed CRC checks. |
| File delivery | Documented general capability | OpenAI documents generated file references; EPUB download behavior in a live ChatGPT session was not tested. |
| Rebuilding after edits | Inferred | The builder can be rerun after changing prepared content. Selective editing or lossless EPUB round trips were not demonstrated. |

### Quality, accessibility and advanced publishing

| Functionality | Status | Evidence and practical limit |
| --- | --- | --- |
| XML well-formedness checks | Observed | All inspected XML-family resources parsed successfully. |
| Missing manifest-resource checks | Observed | None found in six packages. |
| XHTML destination and anchor checks | Observed | No missing destinations or fragments found in the audited references. |
| Text-preservation checks | Observed, narrow | Builder compares selected table-cell text after transformation. It does not prove whole-document fidelity. |
| Preview generation | Observed, narrow | Builder writes one chapter preview; no completed visual review was established. |
| Full EPUB conformance validation | Unverified | No EPUBCheck result was available or run. |
| Accessibility audit and conformance | Unverified | No accessibility metadata was found. Semantic headings alone do not establish accessibility compliance. |
| Reading-system interoperability | Unverified | No Apple Books, Kobo, Thorium or other reader was tested. |
| Fixed layout, right-to-left layout and vertical writing | Unverified | No relevant samples or live tests. |
| Audio, video and synchronized narration | Unverified | No media resources or overlays in the sample. |
| JavaScript and interactive content | Unverified | No script resources or elements in the sample. |
| DRM, retailer submission and storefront acceptance | Unverified | No relevant workflow evidence. |
| Guaranteed latency, size limits and batch throughput | Unverified | No controlled benchmark or ChatGPT EPUB service limit was found. |

References for the tool-level entries: [Pandoc manual](https://pandoc.org/MANUAL.html), [Pandoc ebook tutorial](https://pandoc.org/epub.html). The [EPUB specification](https://www.w3.org/TR/epub-33/) defines publication requirements; format features are not proof of ChatGPT support.

## What the artifact checks prove

| Check | Result |
| --- | --- |
| Files inspected | 6 |
| Declared package version | 3.0 in all files; this alone does not identify a particular EPUB 3 specification edition or prove conformance |
| Compressed file sizes | 16,784 to 56,741 bytes |
| XHTML resources | 109 total |
| Spine entries | 10 to 29 per publication |
| Navigation and NCX | Present in all six |
| External XHTML references | 367 total |
| ZIP CRC and first-entry mimetype | Passed in all six |
| XML parsing, manifest-resource and XHTML-link checks | No failures found within the audit's scope |
| Image, embedded font, audio and video resources | None found |
| Accessibility metadata and fixed-layout declarations | None found |
| Generator-identifying metadata | None found |

The audit reads existing files without changing them. Its snapshot omits book text, titles, authors and source filenames, and identifies samples by file hashes. It does not inspect every CSS URL, validate NCX destinations, run EPUBCheck, compare source documents, or exercise reader rendering. [Audit snapshot](epub-evidence.json)

EPUBCheck checks EPUB conformance. Passing the narrower checks above cannot substitute for it or guarantee visual quality and accessibility. [EPUBCheck](https://www.w3.org/publishing/epubcheck/)

## Implications for an agent-facing EPUB tool

The observed workflow can produce useful reading editions with chapters, source links and customized presentation. Its dependable capability boundary is much smaller than the complete EPUB format.

A stronger tool should make three things explicit: which input features it preserves, which output features it guarantees, and which checks it completed. It should return actionable problems rather than making an agent infer whether a generated archive is publishable. Speed claims need measured workloads; producing a file successfully is not enough.

The main unresolved question is product-wide behavior. This research did not run new conversations in ChatGPT or inspect private production systems. A representative live study would need recorded prompts, tools, model and surface identifiers, generated files, validation reports and reader checks. Until then, describe demonstrated workflows rather than universal ChatGPT EPUB support.

## Sources

- [OpenAI Code Interpreter](https://developers.openai.com/api/docs/guides/tools-code-interpreter): Python execution and generated-file handling.
- [ChatGPT work with files](https://learn.chatgpt.com/docs/artifacts-viewer): user-facing file creation and preview workflows.
- [Pandoc ebook tutorial](https://pandoc.org/epub.html): chapter assembly, cover, metadata and styling examples.
- [Pandoc manual](https://pandoc.org/MANUAL.html): converter features and EPUB options.
- [EPUB 3.3 specification](https://www.w3.org/TR/epub-33/): reference for publication structure, not evidence of ChatGPT implementation.
- [EPUBCheck](https://www.w3.org/publishing/epubcheck/): full conformance-checking tool.
- Local builder (identified by its hash in the audit snapshot): inspected generation and checking logic.
- Installed active-learning EPUB instructions: prescribed Pandoc workflow.
- [Local audit snapshot](epub-evidence.json): counts, checks and file hashes.
