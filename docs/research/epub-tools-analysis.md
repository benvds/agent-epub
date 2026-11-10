# EPUB tools and supported functionality

Historical research snapshot. Current released support is in the [feature matrix](epub-feature-matrix.md), and the later [practical agent benchmark](../verification/MILESTONE_5A.md) records the measured Pandoc comparison. The original findings below retain their dated scope.

Research date: 9 November 2026. This comparison covers seven creation or editing tools and three quality tools.

The established tools offer strong conversion, book formatting and specialist checks. The opportunity for an agent-facing product is to combine predictable creation with clear capability boundaries, preservation checks and actionable results. That is a product inference from the reviewed documentation, not a claim that no competing product exists.

## Selection and evidence

The selection covers familiar publishing workflows: document conversion, ebook management, specialist EPUB editing, automated packaging, professional page design and independent-author formatting. EPUBCheck, Ace and Thorium add standards, accessibility and reading-system perspectives.

“Popular” here means established candidates relevant to these workflows, rather than a measured market-share ranking. No comparable active-user figures were established. “Authoritative” applies most strongly to the standards and accessibility tools maintained by W3C, DAISY and EDRLab, within their respective roles.

All capability findings below come from official project or vendor documentation. No software was purchased, installed or benchmarked. A documented capability is not proof of fidelity for every input. Where documentation did not establish a capability, the entry says so rather than assuming it is absent.

## Creation and editing inventory

| Tool | Documented functionality | Access relevant to agents | Material boundary |
| --- | --- | --- | --- |
| **Pandoc** | Converts document formats including Markdown, HTML and DOCX; produces EPUB 2 or 3; supports chapter splitting, contents, metadata, covers, CSS, fonts, notes, citations and math options. | Documented command-line use and reusable options suit unattended conversion. | Its own manual warns that conversions can lose formatting or structures beyond its document model, including some complex tables. EPUB output still needs separate checking. [Manual](https://pandoc.org/MANUAL.html), [ebook tutorial](https://pandoc.org/epub.html) |
| **calibre** | Broad ebook conversion, chapter and contents detection, metadata, typography and style controls, font embedding and table linearization. Its editor adds live preview, cover and contents editing, resource management, link-aware renaming, cleanup and checkpoints. | Conversion is available through a documented command-line interface. | Conversion quality depends on source format. The manual calls out substantial PDF-input limitations. Conversion heuristics may change presentation or structure. The editor's checks should not be assumed equivalent to EPUBCheck. [Conversion](https://manual.calibre-ebook.com/conversion.html), [editor](https://manual.calibre-ebook.com/edit.html), [conversion interface](https://manual.calibre-ebook.com/generated/en/ebook-convert.html) |
| **Sigil** | EPUB 2 and 3 editing; source and preview views; multi-level contents generation; metadata editing; EPUB/HTML, image and stylesheet import; search and replace; spelling; plugins and checkpoints. | Plugins can extend the application. | The reviewed documentation presents an interactive editor; it does not establish a general unattended creation contract. Fine control requires managing publication structure. [Official feature list](https://sigil-ebook.com/sigil/) |
| **EbookLib** | Reads EPUBs, exposes metadata and resources, creates chapter items, styles, covers, metadata, contents, spine and navigation. The tutorial covers EPUB 2/3 handling. | Programmatic use is its documented workflow. | The caller must define structure and add navigation. Its tutorial explicitly warns that reading, modifying and writing an existing book may not preserve the desired result. It is not a high-level manuscript formatter or proof of conformance. [Official tutorial](https://docs.sourcefabric.org/projects/ebooklib/en/latest/tutorial.html) |
| **Adobe InDesign** | Exports reflowable and fixed-layout EPUBs. Fixed layout supports precise design, audio, video and interactive elements. Export options include publication metadata, contents, images, styling and accessibility-related controls. | The reviewed pages document interactive export and preview. | The EPUB pages reviewed do not establish a ready-made agent workflow. Layout choice and export preparation affect the result; reader support remains relevant. [Export](https://helpx.adobe.com/indesign/desktop/save-export-and-publish/export-to-epub/export-to-epub.html), [options](https://helpx.adobe.com/indesign/desktop/save-export-and-publish/export-to-epub/epub-export-options.html), [limitations](https://helpx.adobe.com/indesign/desktop/save-export-and-publish/export-to-epub/epub-export-support-limitations.html) |
| **Vellum** | Imports manuscripts, applies book styles, includes image and text features, supports web/internal links and endnotes, previews device settings, and generates platform-specific editions or Generic EPUB. | The reviewed help documents a human-operated authoring and generation workflow. | Generation requires a purchase. A documented unattended agent interface was not established. Its preview is useful evidence of appearance, not a substitute for independent validation. [Import](https://help.vellum.pub/importing/), [text features](https://help.vellum.pub/text-features/), [preview](https://help.vellum.pub/preview/), [generation](https://help.vellum.pub/generating/) |
| **Atticus** | DOCX import or writing from scratch; chapter and front/back matter organization; title, cover and author information; themes; device previews; reflowable EPUB and print PDF export. Its help specifies EPUB 3 output. | The documented workflow uses an editor, formatting controls and export buttons. | Current reviewed help says tables/charts cannot be imported and suggests images instead. EPUB uses endnotes; page-bottom footnotes are print-only. The vendor cautions about extensive or long notes. No unattended agent interface was established. [Quick start](https://intercom.help/atticus-5877e36564df/en/articles/12581851-quick-start-guide-to-atticus), [EPUB version](https://intercom.help/atticus-5877e36564df/en/articles/12683237-can-i-export-epub2-files-from-atticus), [tables](https://intercom.help/atticus-5877e36564df/en/articles/13187630-how-do-i-create-a-table-or-chart-in-atticus), [notes](https://intercom.help/atticus-5877e36564df/en/articles/12684371-does-atticus-support-footnotes-or-endnotes) |

“Programmatic” or “command-line” access above describes available interaction, not a recommendation for the new product's implementation. The absence of a documented agent interface in this review is not proof that other integrations are impossible.

## Validation and reading inventory

| Tool | What it supports | What it cannot establish alone |
| --- | --- | --- |
| **EPUBCheck** | Checks EPUB 2 and 3 conformance against EPUB specifications. Offers unattended checks and structured reports, including JSON and XML. W3C project maintained by DAISY. | Visual quality, preservation of source meaning, complete accessibility or acceptance by every retailer. [Project](https://www.w3.org/publishing/epubcheck/), [reporting options](https://www.w3.org/publishing/epubcheck/docs/cli/) |
| **Ace by DAISY** | Automated EPUB accessibility checks; metadata, outline and image/alternative-text review information; human-readable and structured reports; desktop and unattended use. | A clean report does not establish WCAG conformance. DAISY says manual inspection and separate EPUB conformance validation are also needed. [DAISY guide](https://kb.daisy.org/publishing/docs/epub/validation/ace.html) |
| **Thorium Reader** | Reads EPUB 2/3, reflowable and fixed layout; adjustable fonts and spacing; page lists, accessibility metadata, media overlays and read aloud. | It is a reading application, rather than an EPUB generator or standards validator. Successful reading in Thorium does not establish behavior in every other reader. [EDRLab feature list](https://www.edrlab.org/software/thorium-reader/) |

These tools measure different qualities. A sound workflow checks conformance, automated accessibility and reading behavior, then reviews anything automated checks cannot decide. [EPUB Accessibility 1.1](https://www.w3.org/TR/epub-a11y-11/) provides the accessibility conformance and discoverability requirements.

## Functionality lessons for an agent-facing product

| User need | Evidence from the comparison | Product implication |
| --- | --- | --- |
| Use agent-authored manuscripts directly | Pandoc accepts document sources; EbookLib accepts prepared book content. | Accept complete manuscripts and explicit chapter collections. Avoid requiring the agent to construct EPUB internals. |
| Create attractive books quickly | Vellum and Atticus emphasize book styles and previews. | Provide a few dependable presentation choices with useful defaults. |
| Preserve research and instructional material | Pandoc supports notes and citations; Atticus documents limits on tables and notes. | Treat tables, sources and notes as first-class content, and report any unsupported structure. |
| Control contents and reading order | Sigil, calibre and EbookLib expose these separately. | Generate consistent navigation and reading order, while allowing explicit ordering. |
| Avoid damage during conversion | Pandoc and calibre document conversion limits; EbookLib warns about round trips. | Detect unsupported input before success, preserve originals, and summarize transformations. |
| Support readable small-screen tables | calibre can linearize tables, while Atticus suggests images for unsupported tables. | Offer an explicit, content-preserving adaptation. Avoid silently turning accessible text into a picture. |
| Know whether a file is valid | EPUBCheck is separate from authoring and conversion. | Make completed standards checks part of the result, with exact issue locations. |
| Know whether accessibility still needs review | Ace documents limits of automated checking. | Separate automated results from human-review requirements and accessibility claims. |
| Trust the preview | Several tools preview output; Thorium exercises an actual reading system. | Identify the preview conditions and distinguish them from tested reader compatibility. |
| Repair failures without guesswork | Standards and accessibility tools expose issue reports. | Return small, actionable findings that an agent can address without interpreting a long log. |
| Work across agent providers | Several tools can operate unattended, but the reviewed sources do not demonstrate an equivalent end-to-end contract across the named agents. | Test the complete discover, create, inspect and revise workflow with ChatGPT, Claude and an Ollama-backed agent. |

The first column expresses user needs. The third column contains proposed product choices, not claims about existing tools.

## Priorities supported by the research

The first release should focus on reflowable text publications: reports, guides, learning booklets and prose books. That scope captures the local examples and much of the converter and formatter functionality above.

The strongest differentiator is a dependable result an agent can inspect: the EPUB, completed checks, precise problems, declared transformations and remaining human-review needs. A larger collection of styling options is less valuable if content disappears or validation is unclear.

Fixed layout, media overlays and interactive books deserve separate later evaluation. They introduce requirements beyond ordinary text conversion and need appropriate reading-system tests. Their presence in the EPUB standard or a design tool does not make them necessary for an initial agent-facing product.

No speed ranking is justified by this documentation review. Competitor latency, robustness and agent ease of use need a shared test corpus and actual runs. A later benchmark should compare the same sources, required checks and completeness criteria, rather than conversion time alone.

## Sources

All sources were accessed on 9 November 2026. Feature descriptions above link directly to the supporting pages.

- Pandoc: [manual](https://pandoc.org/MANUAL.html), [ebook tutorial](https://pandoc.org/epub.html).
- calibre: [conversion](https://manual.calibre-ebook.com/conversion.html), [editing](https://manual.calibre-ebook.com/edit.html), [conversion interface](https://manual.calibre-ebook.com/generated/en/ebook-convert.html).
- Sigil: [official features](https://sigil-ebook.com/sigil/).
- EbookLib: [official tutorial](https://docs.sourcefabric.org/projects/ebooklib/en/latest/tutorial.html).
- Adobe: [EPUB export](https://helpx.adobe.com/indesign/desktop/save-export-and-publish/export-to-epub/export-to-epub.html), [options](https://helpx.adobe.com/indesign/desktop/save-export-and-publish/export-to-epub/epub-export-options.html), [limitations](https://helpx.adobe.com/indesign/desktop/save-export-and-publish/export-to-epub/epub-export-support-limitations.html).
- Vellum: [import](https://help.vellum.pub/importing/), [text features](https://help.vellum.pub/text-features/), [preview](https://help.vellum.pub/preview/), [generation](https://help.vellum.pub/generating/).
- Atticus: [quick start](https://intercom.help/atticus-5877e36564df/en/articles/12581851-quick-start-guide-to-atticus), [EPUB version](https://intercom.help/atticus-5877e36564df/en/articles/12683237-can-i-export-epub2-files-from-atticus), [tables](https://intercom.help/atticus-5877e36564df/en/articles/13187630-how-do-i-create-a-table-or-chart-in-atticus), [notes](https://intercom.help/atticus-5877e36564df/en/articles/12684371-does-atticus-support-footnotes-or-endnotes).
- EPUBCheck: [project](https://www.w3.org/publishing/epubcheck/), [structured reporting](https://www.w3.org/publishing/epubcheck/docs/cli/).
- DAISY: [Ace and its limitations](https://kb.daisy.org/publishing/docs/epub/validation/ace.html).
- EDRLab: [Thorium Reader](https://www.edrlab.org/software/thorium-reader/).
- W3C: [EPUB Accessibility 1.1](https://www.w3.org/TR/epub-a11y-11/).
