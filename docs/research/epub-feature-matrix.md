# EPUB feature comparison matrix

Updated 10 November 2026. This matrix compares 69 publication and workflow capabilities across Agent EPUB and twelve alternatives. Agent EPUB covers a focused reflowable subset; this inventory also exposes the advanced publishing features and integration work outside that subset.

The Agent EPUB baseline is released **v0.5.1**, including detailed diagnostics, explicit Markdown rules and configurable chapter splitting. Competitor columns retain their dated official-documentation evidence. A separate [three-pair benchmark](../verification/MILESTONE_5A.md) tested v0.5.0 against a prepared Pandoc 3.13 wrapper; it does not establish hands-on compatibility or speed for the other alternatives.

## How to read the matrix

| Mark | Meaning |
| --- | --- |
| Y | The documented workflow directly provides the capability. Limits still apply. |
| P | Partial, conditional or source-dependent support. Read the qualifications below. |
| M | Requires manual markup, application code or a custom script. This is not an automatic authoring feature. |
| E | A documented external tool, engine or integration is required. |
| N | Outside the documented product scope, explicitly unsupported, or absent from the inspected Agent EPUB interface. |
| ? | The reviewed documentation does not establish support. This does not mean unsupported. |

Rows cover practical EPUB content, navigation, metadata, layout, accessibility and adjacent input/automation needs. They are not an attribute-by-attribute conformance checklist. EPUB 2/3 support does not imply support for every optional feature. Packaging caller-written XHTML does not mean a library can generate that feature from Markdown. Reader features such as bookmarks, annotations, search and text-to-speech depend on the reading system and are not counted as exporter capabilities. DRM is a separate distribution capability, not a requirement for EPUB conformance. [EPUB 3.3](https://www.w3.org/TR/epub-33/), [EPUB Accessibility 1.1](https://www.w3.org/TR/epub-a11y-11/)

The same feature IDs appear in both panels so all thirteen tools can be compared without one extremely wide table. Agent EPUB is repeated for reference. Source links identify the evidence for each tool column; qualifications prevent a Y or P from implying unrestricted support.

## Converters and command-line publishing tools

Columns use [Agent EPUB usage](../USAGE.md) and [agent interface](../AGENT_INTERFACE.md), [Pandoc manual](https://pandoc.org/MANUAL.html), [Quarto EPUB options](https://quarto.org/docs/reference/formats/epub.html), [mdbindery documentation](https://sagol.github.io/mdbindery/), [calibre conversion](https://manual.calibre-ebook.com/generated/en/ebook-convert.html) and [editor](https://manual.calibre-ebook.com/edit.html), [Crowbook guide](https://crowdagger.github.io/crowbook/book/book.html), and [mdbook-epub configuration](https://github.com/michael-f-bryan/mdbook-epub).

### Formats

| ID | Capability | Agent | Pandoc | Quarto | mdbindery | calibre | Crowbook | mdbook |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F01 | EPUB 2 output | N | Y | ? | ? | Y | Y | Y |
| F02 | EPUB 3 reflowable output | Y | Y | Y | Y | Y | Y | Y |
| F03 | Fixed-layout EPUB export | N | ? | ? | ? | ? | ? | ? |
| F04 | PDF output alongside EPUB | N | E | E | E | Y | E | ? |
| F05 | Edit an existing EPUB | N | P | ? | N | Y | N | N |

### Inputs

| ID | Capability | Agent | Pandoc | Quarto | mdbindery | calibre | Crowbook | mdbook |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F06 | Literal plain-text file input | Y | P | P | ? | Y | ? | ? |
| F07 | Markdown manuscript input | Y | Y | Y | Y | P | Y | Y |
| F08 | HTML or XHTML import | N | Y | P | P | Y | ? | ? |
| F09 | DOCX manuscript import | N | Y | ? | N | Y | N | N |
| F10 | Existing mdBook project input | N | E | E | Y | ? | ? | Y |
| F11 | Explicit ordered chapter collection | Y | Y | Y | Y | P | Y | Y |
| F12 | Versioned inline book-request JSON | Y | P | ? | ? | ? | ? | ? |

### Text and assets

| ID | Capability | Agent | Pandoc | Quarto | mdbindery | calibre | Crowbook | mdbook |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F13 | Headings, lists, emphasis and quotations | Y | Y | Y | Y | Y | Y | Y |
| F14 | Inline and block code | Y | Y | Y | Y | P | Y | Y |
| F15 | Code syntax highlighting in EPUB | N | Y | Y | Y | P | Y | ? |
| F16 | Simple semantic tables | P | Y | Y | Y | P | Y | ? |
| F17 | Merged cells or complex table headers | N | P | P | ? | P | ? | ? |
| F18 | Linked footnotes or endnotes | Y | Y | Y | Y | P | Y | Y |
| F19 | Bibliography processing from citation data | N | Y | Y | P | ? | ? | ? |
| F20 | Task-list rendering | N | Y | Y | Y | ? | ? | ? |
| F21 | Raw HTML within manuscript content | N | P | P | P | Y | ? | ? |
| F22 | Supplied cover image | P | Y | Y | Y | Y | Y | Y |
| F23 | Inline raster illustrations | N | Y | Y | Y | Y | Y | ? |
| F24 | SVG resources or illustrations | N | P | P | ? | P | ? | ? |
| F25 | MathML mathematics in EPUB | N | Y | P | Y | ? | ? | ? |
| F26 | Mermaid diagram conversion | N | E | E | E | ? | ? | ? |

### Advanced EPUB

| ID | Capability | Agent | Pandoc | Quarto | mdbindery | calibre | Crowbook | mdbook |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F27 | Embedded audio | N | P | ? | ? | ? | ? | ? |
| F28 | Embedded video | N | P | ? | ? | ? | ? | ? |
| F29 | SMIL media overlays for synchronized narration | N | ? | ? | ? | ? | ? | ? |
| F30 | Scripted EPUB content | N | P | ? | ? | ? | ? | ? |
| F31 | Remote publication resources | N | P | ? | N | P | ? | ? |
| F32 | DRM protection applied by the tool | N | ? | ? | ? | N | ? | ? |
| F33 | Ruby text annotations | N | P | P | ? | ? | ? | ? |
| F34 | Resource fallback chains | N | ? | ? | ? | ? | ? | ? |
| F35 | Font obfuscation | N | ? | ? | ? | ? | ? | ? |
| F36 | Multiple renditions in one publication | N | ? | ? | ? | ? | ? | ? |

### Navigation

| ID | Capability | Agent | Pandoc | Quarto | mdbindery | calibre | Crowbook | mdbook |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F37 | Generated visible contents page | Y | Y | Y | Y | Y | Y | Y |
| F38 | Generated reader-native navigation | Y | Y | Y | Y | Y | Y | Y |
| F39 | Nested section contents | Y | Y | Y | P | Y | Y | Y |
| F40 | Configurable chapter split level | Y | Y | Y | P | Y | P | P |
| F41 | Automatic previous/next links in reading content | Y | ? | ? | ? | ? | ? | ? |
| F42 | Page-list navigation mapped to print pages | N | ? | ? | ? | ? | ? | ? |
| F43 | Landmark navigation for publication regions | N | P | P | ? | P | ? | ? |
| F44 | Index or glossary authoring support | N | P | P | ? | ? | ? | ? |

### Metadata

| ID | Capability | Agent | Pandoc | Quarto | mdbindery | calibre | Crowbook | mdbook |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F45 | Title, publication language and author | Y | Y | Y | Y | Y | Y | Y |
| F46 | Multiple authors or contributor roles | N | Y | Y | Y | Y | P | P |
| F47 | Publisher, rights and description metadata | N | Y | Y | Y | Y | P | P |
| F48 | Caller-supplied permanent publication identifier | N | Y | Y | Y | Y | ? | ? |
| F49 | Explicit RTL or vertical-writing controls | N | P | P | ? | P | ? | ? |
| F50 | Accessibility discovery metadata | N | P | P | Y | P | ? | ? |

### Presentation and accessibility

| ID | Capability | Agent | Pandoc | Quarto | mdbindery | calibre | Crowbook | mdbook |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F51 | Custom EPUB CSS | N | Y | Y | ? | Y | Y | Y |
| F52 | Embedded fonts | N | Y | Y | ? | Y | Y | Y |
| F53 | Selectable book styles or themes | N | P | P | ? | P | P | P |
| F54 | Integrated visual preview | N | N | P | E | Y | N | N |
| F55 | Supplied alternative text for images | P | Y | Y | Y | M | ? | ? |
| F56 | Semantic headings, tables or notes | P | P | P | P | P | P | P |
| F57 | Integrated Ace accessibility report | N | ? | ? | E | ? | ? | ? |

### Checks and automation

| ID | Capability | Agent | Pandoc | Quarto | mdbindery | calibre | Crowbook | mdbook |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F58 | Built-in package or link checks | Y | ? | ? | Y | Y | P | ? |
| F59 | Integrated official EPUBCheck invocation | E | N | N | E | ? | ? | ? |
| F60 | Automated source-to-output preservation evidence | P | ? | ? | P | ? | ? | ? |
| F61 | Declared reproducible EPUB build controls | N | Y | ? | Y | ? | ? | ? |
| F62 | Unattended command-line export | Y | Y | Y | Y | Y | Y | Y |
| F63 | Capability or supported-format discovery command | Y | Y | P | P | Y | Y | ? |
| F64 | Structured operation result and artifact status | Y | P | P | Y | ? | ? | ? |
| F65 | Detailed machine-readable issue codes and locations | Y | P | ? | Y | ? | ? | ? |
| F66 | Aggregate source preflight before export | N | P | P | Y | ? | ? | ? |
| F67 | Preserve previous output when requested checks fail | Y | ? | ? | P | ? | ? | ? |
| F68 | Packaged agent skill or integration recipe | Y | ? | ? | Y | ? | ? | ? |
| F69 | Generation without separately installing Python or Java | Y | Y | P | P | Y | Y | Y |

## Editors, authoring applications and libraries

Columns use the same Agent EPUB evidence, [Sigil features](https://sigil-ebook.com/sigil/), [Vellum text features](https://help.vellum.pub/text-features/), [generation](https://help.vellum.pub/generating/) and [accessibility guidance](https://faq.vellum.pub/do-ebooks-generated-by-vellum-meet-accessibility-guidelines/), [Atticus quick start](https://intercom.help/atticus-5877e36564df/en/articles/12581851-quick-start-guide-to-atticus), [table limits](https://intercom.help/atticus-5877e36564df/en/articles/12683623-can-i-use-tables-charts-in-atticus) and [notes](https://intercom.help/atticus-5877e36564df/en/articles/12623367-footnotes-and-endnotes), [InDesign export options](https://helpx.adobe.com/hk_en/indesign/desktop/save-export-and-publish/export-to-epub/epub-export-options.html) and [limitations](https://helpx.adobe.com/indesign/desktop/save-export-and-publish/export-to-epub/epub-export-support-limitations.html), [EbookLib tutorial](https://docs.sourcefabric.org/projects/ebooklib/en/latest/tutorial.html), and [epub-builder API](https://docs.rs/epub-builder/latest/epub_builder/).

EbookLib and epub-builder are libraries, so M frequently means the application supplies XHTML and behavior. Sigil is an editor: manual code editing is different from a supported unattended conversion pipeline. Vellum and Atticus marks refer to EPUB output, not print-only features. InDesign features may differ between reflowable and fixed layout.

### Formats

| ID | Capability | Agent | Sigil | Vellum | Atticus | InDesign | EbookLib | epub-builder |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F01 | EPUB 2 output | N | Y | Y | N | Y | P | Y |
| F02 | EPUB 3 reflowable output | Y | Y | Y | Y | Y | Y | Y |
| F03 | Fixed-layout EPUB export | N | ? | ? | N | Y | ? | ? |
| F04 | PDF output alongside EPUB | N | N | Y | Y | Y | N | N |
| F05 | Edit an existing EPUB | N | Y | N | N | N | P | N |

### Inputs

| ID | Capability | Agent | Sigil | Vellum | Atticus | InDesign | EbookLib | epub-builder |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F06 | Literal plain-text file input | Y | ? | ? | ? | Y | M | M |
| F07 | Markdown manuscript input | Y | ? | ? | N | ? | M | M |
| F08 | HTML or XHTML import | N | Y | ? | ? | ? | Y | Y |
| F09 | DOCX manuscript import | N | ? | Y | Y | Y | M | M |
| F10 | Existing mdBook project input | N | ? | ? | N | ? | M | M |
| F11 | Explicit ordered chapter collection | Y | Y | Y | Y | Y | Y | Y |
| F12 | Versioned inline book-request JSON | Y | ? | ? | ? | ? | M | M |

### Text and assets

| ID | Capability | Agent | Sigil | Vellum | Atticus | InDesign | EbookLib | epub-builder |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F13 | Headings, lists, emphasis and quotations | Y | Y | Y | Y | Y | M | M |
| F14 | Inline and block code | Y | M | ? | ? | M | M | M |
| F15 | Code syntax highlighting in EPUB | N | M | ? | ? | ? | M | M |
| F16 | Simple semantic tables | P | M | ? | N | P | M | M |
| F17 | Merged cells or complex table headers | N | M | ? | N | P | M | M |
| F18 | Linked footnotes or endnotes | Y | M | Y | P | Y | M | M |
| F19 | Bibliography processing from citation data | N | M | ? | ? | ? | M | M |
| F20 | Task-list rendering | N | M | ? | ? | ? | M | M |
| F21 | Raw HTML within manuscript content | N | Y | N | N | ? | Y | Y |
| F22 | Supplied cover image | P | Y | Y | Y | Y | Y | Y |
| F23 | Inline raster illustrations | N | Y | Y | Y | Y | M | M |
| F24 | SVG resources or illustrations | N | M | ? | ? | Y | Y | Y |
| F25 | MathML mathematics in EPUB | N | M | ? | ? | Y | M | M |
| F26 | Mermaid diagram conversion | N | E | ? | ? | ? | M | M |

### Advanced EPUB

| ID | Capability | Agent | Sigil | Vellum | Atticus | InDesign | EbookLib | epub-builder |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F27 | Embedded audio | N | ? | ? | ? | Y | P | M |
| F28 | Embedded video | N | ? | ? | ? | Y | P | M |
| F29 | SMIL media overlays for synchronized narration | N | ? | ? | ? | ? | P | ? |
| F30 | Scripted EPUB content | N | ? | ? | ? | Y | P | M |
| F31 | Remote publication resources | N | ? | ? | ? | ? | M | M |
| F32 | DRM protection applied by the tool | N | ? | ? | ? | ? | ? | ? |
| F33 | Ruby text annotations | N | M | ? | ? | ? | M | M |
| F34 | Resource fallback chains | N | ? | ? | ? | ? | ? | ? |
| F35 | Font obfuscation | N | ? | ? | ? | ? | ? | ? |
| F36 | Multiple renditions in one publication | N | ? | ? | ? | ? | ? | ? |

### Navigation

| ID | Capability | Agent | Sigil | Vellum | Atticus | InDesign | EbookLib | epub-builder |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F37 | Generated visible contents page | Y | Y | Y | Y | Y | Y | Y |
| F38 | Generated reader-native navigation | Y | Y | Y | Y | Y | Y | Y |
| F39 | Nested section contents | Y | Y | P | P | Y | Y | Y |
| F40 | Configurable chapter split level | Y | M | P | P | Y | M | M |
| F41 | Automatic previous/next links in reading content | Y | M | ? | ? | ? | M | M |
| F42 | Page-list navigation mapped to print pages | N | M | ? | ? | Y | M | ? |
| F43 | Landmark navigation for publication regions | N | M | P | ? | Y | Y | P |
| F44 | Index or glossary authoring support | N | M | ? | ? | P | M | M |

### Metadata

| ID | Capability | Agent | Sigil | Vellum | Atticus | InDesign | EbookLib | epub-builder |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F45 | Title, publication language and author | Y | Y | Y | Y | Y | Y | Y |
| F46 | Multiple authors or contributor roles | N | M | ? | ? | ? | Y | P |
| F47 | Publisher, rights and description metadata | N | Y | P | P | Y | Y | P |
| F48 | Caller-supplied permanent publication identifier | N | M | ? | ? | Y | Y | P |
| F49 | Explicit RTL or vertical-writing controls | N | M | ? | ? | P | P | P |
| F50 | Accessibility discovery metadata | N | M | P | ? | Y | M | M |

### Presentation and accessibility

| ID | Capability | Agent | Sigil | Vellum | Atticus | InDesign | EbookLib | epub-builder |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F51 | Custom EPUB CSS | N | Y | N | N | Y | Y | Y |
| F52 | Embedded fonts | N | Y | Y | P | P | Y | Y |
| F53 | Selectable book styles or themes | N | P | Y | Y | Y | M | M |
| F54 | Integrated visual preview | N | Y | Y | Y | Y | N | N |
| F55 | Supplied alternative text for images | P | M | Y | ? | Y | M | M |
| F56 | Semantic headings, tables or notes | P | M | P | P | P | M | M |
| F57 | Integrated Ace accessibility report | N | ? | P | ? | ? | N | N |

### Checks and automation

| ID | Capability | Agent | Sigil | Vellum | Atticus | InDesign | EbookLib | epub-builder |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| F58 | Built-in package or link checks | Y | Y | ? | ? | P | ? | ? |
| F59 | Integrated official EPUBCheck invocation | E | ? | ? | ? | ? | N | N |
| F60 | Automated source-to-output preservation evidence | P | ? | ? | ? | ? | M | M |
| F61 | Declared reproducible EPUB build controls | N | ? | ? | ? | ? | ? | ? |
| F62 | Unattended command-line export | Y | ? | ? | ? | M | M | M |
| F63 | Capability or supported-format discovery command | Y | ? | ? | ? | ? | M | M |
| F64 | Structured operation result and artifact status | Y | ? | ? | ? | ? | M | M |
| F65 | Detailed machine-readable issue codes and locations | Y | ? | ? | ? | ? | M | M |
| F66 | Aggregate source preflight before export | N | ? | ? | ? | P | M | M |
| F67 | Preserve previous output when requested checks fail | Y | ? | ? | ? | ? | M | M |
| F68 | Packaged agent skill or integration recipe | Y | ? | ? | ? | ? | M | M |
| F69 | Generation without separately installing Python or Java | Y | Y | Y | P | Y | N | Y |

## Qualifications that affect comparisons

### Agent EPUB

- F04 through F10: Agent EPUB generates new EPUBs from text and Markdown. It does not import arbitrary HTML, DOCX or existing EPUBs, and it has no mdBook-project adapter or PDF output.
- F16 and F17: tables need one header row and rectangular cells. Merged cells, multiline cells, captions and complex headers are rejected. F18 includes numbered notes, multiple backlinks and reader-dependent popup behavior.
- F22 and F55: supplied covers are JPEG/PNG with bounded decoding and alternative text. Inline images remain unsupported, so alternative-text support is currently cover-only.
- F37 through F43: visible/native nested contents and chapter links are generated. Root heading split levels 1–6 are configurable, with H1 as the default; previous/next links connect chapters. Page-list and landmark navigation are not generated.
- F45 through F50: title, language and one optional author are supported. A random identifier and modification time are generated. There are no caller controls for rich metadata, stable identity, RTL/vertical layout or accessibility discovery metadata.
- F56 is partial: semantic headings, column headers and note roles exist, but do not establish complete accessibility. F58 checks this tool's profile. F59 invokes a separately installed EPUBCheck executable when requested; this is separate from the built-in checker.
- F60 refers to the existing verification corpus and input-mode equivalence checks, not a per-build preservation manifest. F65 retains four broad categories and adds stable detailed issue codes, source references, available one-based line/column locations and remedies. Unavailable fields are null; semantic footnote errors may have no location. F66 has validation during build, but no aggregate source-only preflight command.
- F67 publishes only after requested checks succeed and protects existing outputs. F69 covers generation and built-in checking; optional EPUBCheck uses Java. Python is for developer/release helpers.

Evidence: [supported content](../USAGE.md), [request/results](../AGENT_INTERFACE.md), [native installation boundary](../INSTALL.md), [milestone 3 verification](../verification/MILESTONE_3.md), [milestone 4 verification](../verification/MILESTONE_4.md), [milestone 5 verification](../verification/MILESTONE_5.md), [workflow benchmark](../verification/MILESTONE_5A.md), [package implementation](../../src/package.rs) and [result implementation](../../src/agent.rs).

### Command-line alternatives

In the measured Pandoc 3.13 configuration, visible note-return links and supplied-source link rewriting required the prepared wrapper. The [benchmark](../verification/MILESTONE_5A.md) records that integration and its limits; ordinary footnote-link support in F18 remains distinct from visible return links.

Pandoc F06 treats ordinary text through a selected reader rather than offering Agent EPUB's literal-text contract. F12/F64 refer to its JSON document model or logs, not an equivalent versioned book request and artifact-result envelope. F17 has conversion limits. F27/F28/F30/F31 need suitable embedded markup/media and reader support. F59 is N for an integrated official-validator invocation, not an assertion that a wrapper cannot run EPUBCheck. F61 requires declared metadata/time controls. [Pandoc manual](https://pandoc.org/MANUAL.html)

Quarto's EPUB output exposes Pandoc options, but not every Pandoc feature was assumed to work through a Quarto project. Its preview is a publishing preview rather than proof of behavior in every EPUB reader. PDF needs a selected engine; Python is needed only for workflows that execute Python, not ordinary Markdown rendering. [EPUB options](https://quarto.org/docs/reference/formats/epub.html), [book outputs](https://quarto.org/docs/books/book-output.html)

mdbindery F19 supports visible numeric references; that does not establish a general bibliography engine. Diagrams require Mermaid tooling. EPUBCheck and Ace are external components; Ace can be skipped or unavailable. F60 uses count tolerances, not a proof of exact meaning/order. F61 depends on a stable identifier and fixed tool inputs. Its installers manage runtimes, so F69 is P rather than a claim that Python/Java are absent. [mdbindery](https://sagol.github.io/mdbindery/), [repository](https://github.com/sagol/mdbindery)

calibre marks combine its converter and editor, as stated in the column sources. Markdown is an input formatting choice in its text workflow. Tables and notes depend on source conversion; PDF output is not a fixed-layout EPUB. Manual HTML/CSS editing and device profiles do not establish advanced EPUB feature preservation or an agent result contract. [Conversion interface](https://manual.calibre-ebook.com/generated/en/ebook-convert.html), [editor](https://manual.calibre-ebook.com/edit.html)

Crowbook includes EPUB 2/3, notes, tables, highlighting and resource/CSS controls. Font resources still need appropriate styling. Its YAML configuration is not a versioned JSON agent request. mdbook-epub exposes cover/CSS/resource options and EPUB 3 footnote backreferences; resource packaging alone was not used to infer every content feature. [Crowbook guide](https://crowdagger.github.io/crowbook/book/book.html), [mdbook-epub](https://github.com/michael-f-bryan/mdbook-epub)

### Applications and libraries

Sigil's source editing makes manually authored content possible, but M does not establish a dedicated formatter or runtime compatibility for that content. Its general feature page does not establish every optional EPUB package feature. [Sigil](https://sigil-ebook.com/sigil/)

Vellum supports endnotes, inline images and internal links. Its documented generation flow is interactive. Accessibility guidance describes vendor testing with Ace; F57 P does not promise a user-visible per-build Ace report. Unknown table, scripting or fixed-layout cells are intentionally not marked unsupported. [Text features](https://help.vellum.pub/text-features/), [generation](https://help.vellum.pub/generating/), [accessibility](https://faq.vellum.pub/do-ebooks-generated-by-vellum-meet-accessibility-guidelines/)

Atticus recommends image substitutes for tables, which do not count as semantic table support. EPUB uses endnotes; page-bottom footnotes are print-only. Its snapshot JSON is account/book backup, not the agent operation result in F64. Browser/account setup differs from installing a standalone CLI. [Tables](https://intercom.help/atticus-5877e36564df/en/articles/12683623-can-i-use-tables-charts-in-atticus), [notes](https://intercom.help/atticus-5877e36564df/en/articles/12623367-footnotes-and-endnotes), [quick start](https://intercom.help/atticus-5877e36564df/en/articles/12581851-quick-start-guide-to-atticus)

InDesign provides fixed/reflowable export, media, scripts, MathML/SVG options and accessibility/navigation controls. Export limitations vary with layout and content. F62 M refers to custom scripting rather than a reviewed ready-made agent CLI. Audio support is not evidence of SMIL narration support. [Export options](https://helpx.adobe.com/hk_en/indesign/desktop/save-export-and-publish/export-to-epub/epub-export-options.html), [limitations](https://helpx.adobe.com/indesign/desktop/save-export-and-publish/export-to-epub/epub-export-support-limitations.html)

EbookLib exposes XHTML/resources and metadata, including media item types; P for media/SMIL does not mean it produces synchronized narration automatically. Its tutorial warns about read/modify/write preservation. epub-builder packages caller content, styles and resources and generates navigation; it is not a Markdown formatter. Its documented metadata limitations explain partial entries. [EbookLib](https://docs.sourcefabric.org/projects/ebooklib/en/latest/tutorial.html), [epub-builder](https://docs.rs/epub-builder/latest/epub_builder/)

## Delivered work and conditional candidates

| Package | Matrix rows affected | Current status |
| --- | --- | --- |
| B1 Detailed diagnostics | F65 | Delivered: detailed codes, available structured locations and remedies |
| B2 Markdown dialect and front matter | F07, F21, F63 | Delivered: recognized front matter rejected; math, strike and alert markers remain literal |
| B3 Explicit chapter splitting | F40 | Delivered: root levels 1–6 through compatible direct and structured inputs |
| C1 Agent workflow benchmark | F62 through F69 and task-specific content | Complete: three paired workflows, retained artifacts and bounded conclusions; no new format capability |
| Conditional local images | F23, F55 | Inline assets and a declared alternative-text policy if selected |
| Conditional metadata/reproducibility | F46 through F50, F61 | Only the fields and reproducibility controls actually selected and verified |
| Conditional preflight and manifest | F60, F66 | Aggregate input diagnostics and inspectable preservation evidence if selected |
| Conditional styles/accessibility | F51, F53, F54, F57 | Selected styling or report integrations if justified by benchmark findings or GitHub issues |

The [roadmap](../ROADMAP.md) is the scope authority. GitHub issues supply user feedback. A question mark is a verification task, not evidence that Agent EPUB has an advantage. Feature counts are not a useful quality score: fixed layout and a stable result contract serve different jobs.

## Source register

All external sources were accessed on 10 November 2026. Competitor marks remain documentation-based. The later Pandoc workflow comparison is linked separately; no hands-on result is inferred for the other tools.

- Agent EPUB: [usage](../USAGE.md), [agent interface](../AGENT_INTERFACE.md), [installation](../INSTALL.md), [roadmap](../ROADMAP.md).
- Standards: [EPUB 3.3](https://www.w3.org/TR/epub-33/), [EPUB Accessibility 1.1](https://www.w3.org/TR/epub-a11y-11/), [EPUBCheck](https://www.w3.org/publishing/epubcheck/).
- Pandoc: [manual](https://pandoc.org/MANUAL.html), [EPUB tutorial](https://pandoc.org/epub.html).
- Quarto: [EPUB options](https://quarto.org/docs/reference/formats/epub.html), [book output](https://quarto.org/docs/books/book-output.html).
- mdbindery: [documentation](https://sagol.github.io/mdbindery/), [repository](https://github.com/sagol/mdbindery).
- calibre: [conversion](https://manual.calibre-ebook.com/generated/en/ebook-convert.html), [editing](https://manual.calibre-ebook.com/edit.html).
- Crowbook: [guide](https://crowdagger.github.io/crowbook/book/book.html), [repository](https://github.com/lise-henry/crowbook).
- mdbook-epub: [repository and configuration](https://github.com/michael-f-bryan/mdbook-epub).
- Sigil: [official features](https://sigil-ebook.com/sigil/).
- Vellum: [text features](https://help.vellum.pub/text-features/), [generation](https://help.vellum.pub/generating/), [accessibility](https://faq.vellum.pub/do-ebooks-generated-by-vellum-meet-accessibility-guidelines/).
- Atticus: [quick start](https://intercom.help/atticus-5877e36564df/en/articles/12581851-quick-start-guide-to-atticus), [table limitations](https://intercom.help/atticus-5877e36564df/en/articles/12683623-can-i-use-tables-charts-in-atticus), [notes](https://intercom.help/atticus-5877e36564df/en/articles/12623367-footnotes-and-endnotes).
- InDesign: [export options](https://helpx.adobe.com/hk_en/indesign/desktop/save-export-and-publish/export-to-epub/epub-export-options.html), [limitations](https://helpx.adobe.com/indesign/desktop/save-export-and-publish/export-to-epub/epub-export-support-limitations.html).
- Libraries: [EbookLib tutorial](https://docs.sourcefabric.org/projects/ebooklib/en/latest/tutorial.html), [epub-builder](https://docs.rs/epub-builder/latest/epub_builder/).
