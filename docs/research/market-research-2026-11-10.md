# Agent EPUB market research and product priorities

Research date: 10 November 2026. Audience: the maintainer deciding where to invest next.

Planning update later on 10 November: milestone 3 is now complete at `v0.3.0`, including tables, footnotes and nested contents. The observations and priorities below retain their `v0.2.0` research baseline; those delivered features are no longer outstanding work. The [updated roadmap](../ROADMAP.md) records the current release status and remaining work packages.

The [feature comparison matrix](epub-feature-matrix.md) provides a later `v0.4.0` capability baseline across thirteen tools. The roadmap schedules the agent workflow benchmark after milestone 5 and uses GitHub issues for feedback.

Agent EPUB has a credible niche as a small local EPUB generation component for agent applications. Its strongest assets are its explicit input contract, protected output publishing and clear reporting of completed checks. Its largest immediate problems are installation friction and a content subset that excludes common report material.

The recommended position is: **Turn supplied Markdown or text into a checked reading edition through one predictable local command.** Compete on deployment, content fidelity and successful agent completion. “Written in Rust” and “for AI agents” are supporting details that competitors can also offer.

## Scope and evidence

The product baseline is the tagged `v0.2.0` source and release verification. A copied local executable reporting `0.2.0` supplied fresh behavioral observations. Its hash, capabilities, inputs and results are recorded in [research probes](market-research-probes-2026-11-10.json). The executable was not rebuilt from the changing working tree. Tables, footnotes and nested contents are already planned for `v0.3.0`; this report treats them as development work, not released features.

Competitor capabilities come from official documentation accessed on the research date. They establish advertised functionality, not comparative reliability. No competitor was installed or benchmarked, and no customer interviews were conducted. The recommendations below are product judgments; market share, addressable spending and willingness to pay remain unmeasured.

More information: [release verification](../verification/MILESTONE_2.md) records existing conformance and reader checks; [roadmap](../ROADMAP.md) separates released and planned scope.

## Where the product fits

The best initial customer is a developer or technical user adding EPUB export to an existing assistant. Their material already exists as text or Markdown. They want a file they can read later, with chapters and links intact, without setting up a publishing project.

| Audience | Job to complete | Current fit | Main obstacle |
| --- | --- | --- | --- |
| Agent application developers | Add a dependable EPUB export tool | Strong within the declared subset | Packaging and repeated integration work |
| People generating learning guides | Read an assembled guide on an ereader | Promising | Tables, notes, diagrams and section navigation |
| People generating research reports | Preserve findings and source links in a reading edition | Partial | Tables, citations, math and charts |
| Documentation maintainers | Ship a manual as an ebook | Partial | Existing mdBook/Quarto workflows and inline assets |
| Independent novelists | Produce a polished retail book | Limited | Styling, front/back matter, metadata and previews |
| Publishers handling complex books | Produce accessible illustrated or fixed-layout publications | Poor | Scope far beyond the current product |

Prioritize report and learning-guide export after the selected milestone 3 additions. Start with prose-heavy material and explicit limits. Technical textbooks with equations and diagrams remain a harder target.

The alternatives are broader than other EPUB generators. A user can keep the Markdown, read a web page, or request a PDF. Discovery should establish why an EPUB matters: reflow, comfortable long reading, offline access or an existing ereader habit. General ebook sales do not establish demand for this CLI.

More information: [product requirements](../PRD.md) describe the intended jobs; [Quarto books](https://quarto.org/docs/books/) shows the broader publishing workflow some prospective users already have.

## How it compares with alternatives

The most consequential competition is Pandoc plus a thin wrapper, followed by tools that already combine Markdown publishing and validation.

| Alternative | Documented strengths | Implication for Agent EPUB |
| --- | --- | --- |
| [Pandoc](https://pandoc.org/MANUAL.html) | Broad document conversion, EPUB options, notes, tables, citations, filters, JSON logs and failure on warnings | Main default alternative. Agent EPUB must save integration work or demonstrate better task outcomes within its subset. |
| [mdbindery](https://github.com/sagol/mdbindery) | Markdown repositories and mdBook input; images, notes, math and table adaptations; EPUBCheck, Ace and content-count gates | Closest workflow competitor found. A small self-contained generator needs a clear deployment or fidelity advantage. |
| [calibre](https://manual.calibre-ebook.com/generated/en/ebook-convert.html) | CLI conversion, metadata, structure detection, contents and device profiles across many formats | Better fit for format conversion. Agent EPUB offers a smaller set of decisions for generating new reading editions. |
| [Quarto](https://quarto.org/docs/books/) | Book projects, bibliography, cross-references and HTML/PDF/Word/EPUB outputs | Better fit for sustained technical publishing. Agent EPUB can suit an export action within another application. |
| [Crowbook](https://github.com/lise-henry/crowbook) | Rust CLI converting Markdown books to HTML, EPUB and PDF; defaults aimed at fiction and typography | Rust and local CLI operation are already available elsewhere. Agent EPUB needs a different user job. |
| [mdbook-epub](https://github.com/michael-f-bryan/mdbook-epub) | mdBook backend and standalone mode; covers, CSS, resources and footnote backreferences | Existing mdBook users have a direct route. Its README still labels it experimental, which is a documentation claim rather than a reliability benchmark. |
| [EbookLib](https://docs.sourcefabric.org/projects/ebooklib/en/latest/tutorial.html) and [epub-builder](https://docs.rs/epub-builder/latest/epub_builder/) | Programmatic EPUB construction with metadata, resources, spine and navigation | Substitute libraries for developers willing to assemble book structure. Agent EPUB removes some application code and EPUB knowledge. |
| [Sigil](https://sigil-ebook.com/sigil/) | Direct EPUB editing, source and preview views, metadata, multi-level contents and plugins | Better for manual repair and precise editing. A complementary review tool for Agent EPUB output. |
| [Vellum](https://vellum.pub/) | Book styling, device previews, front/back matter and ebook/print generation | Competes for polished author publishing. Listed prices are US$199.99 for ebooks and US$249.99 for ebooks plus print, excluding tax; Mac only. |
| [Atticus](https://www.atticus.io/) | Writing, themes, device previews and ebook/print export across desktop platforms | Competes for author convenience. The vendor lists US$147 as a one-time payment. |

Pandoc is already usable by agents. Its manual documents format and extension discovery, reusable defaults, JSON logging and warning-based failure. Agent EPUB's distinction is its focused request/result contract and combined publishing behavior, rather than command-line or machine-readable access alone. [Pandoc manual](https://pandoc.org/MANUAL.html)

mdbindery also explicitly documents agent skills, JSON reports and phone-size previews. Its website describes an installer footprint of about 370 MB without Node tools, or 1.5 GB with them. Those are vendor figures, not local measurements. Its broader toolchain gives Agent EPUB a plausible deployment opportunity, but the workflow overlap is substantial. [mdbindery agent integration and installation](https://sagol.github.io/mdbindery/)

“Lightweight Rust EPUB tooling” also has adjacent competition. epubveri offers native and browser-based validation, separately from generation. Its documentation distinguishes its verdict from official EPUBCheck. Treat it as a possible integration to evaluate, not proof that general EPUB validation should become this project's next product. [epubveri](https://github.com/veripublica/epubveri)

More information: the linked official pages establish each comparison; the existing [EPUB tool inventory](epub-tools-analysis.md) covers additional publishing tools and quality checks.

## Current strengths

### A useful contract for applications

`capabilities` returns the supported request schema, features and limits. Builds accept ordered files, request files or streamed requests containing inline material. JSON stdout includes status, artifact location, chapter count, elapsed time and check coverage. This reduces the amount of application code needed around a successful build. [Agent interface](../AGENT_INTERFACE.md), [released implementation](../../src/agent.rs)

The contract has substantive behavior behind it: unknown request fields fail, ambiguous paths fail, chapter order is explicit, and unsupported structures receive errors. These choices help an agent recover without guessing which input the tool used.

### Protected output publishing

The generator writes to a temporary publication, checks it, and publishes only after requested checks pass. Explicit overwrite is required, output cannot replace an input, and failed generation preserves an existing result. This matters for unattended repeated use. [Usage and failure behavior](../USAGE.md), [package implementation](../../src/package.rs)

### Small local runtime

Generation needs no AI account, network service, Python or Java. External EPUBCheck adds its own runtime requirements. The copied macOS executable measured 2,887,840 bytes, about 2.75 MiB. This is one binary measurement, not an installed-footprint comparison across platforms. [Usage](../USAGE.md), [probe evidence](market-research-probes-2026-11-10.json)

A small runtime could help local assistants, desktop applications and restricted environments. That advantage needs readily installable binaries before most users can experience it.

### Verification with clear limits

The repository records independent EPUBCheck corpus checks, Apple Books and EPUB.js observations, and 20 successful synthetic tasks in a live Codex CLI evaluation. The evaluation scored actual first output artifacts rather than trusting the agent's narrative. This provides useful evidence for the tested workflow. [Release checks](../verification/MILESTONE_2.md), [agent evaluation](../verification/AGENT_EVALUATIONS.md)

The tool also distinguishes built-in structural checks from independent validation and human review. That honesty is valuable when an agent reports success to a user.

### Fast enough for interactive export

The recorded benchmark reports a 35.055 ms 95th percentile over 30 local runs for 100,000 body words, 40 chapters and a roughly 9.7 MB PNG cover. It includes startup, generation, decoding and built-in checks; it excludes external EPUBCheck and transfers. Repeated text and warm caches limit its generality. [Benchmark record](../verification/MILESTONE_2.md)

This supports an interactive export workflow. It does not establish that Agent EPUB is faster than Pandoc or another competitor. Reducing an already short conversion time is a lower priority than reducing installation and agent repair time.

More information: [milestone 2 verification](../verification/MILESTONE_2.md) states the tested conditions and remaining boundaries.

## Weaknesses and observed adoption gaps

### Common report content still fails

Released `v0.2.0` rejects tables, notes, inline images, task lists and raw HTML. Explicit rejection protects content, but users must still rewrite their input or choose another tool. Tables and sources are especially relevant to the report and learning-guide audience. The planned milestone 3 scope addresses part of this problem. [Roadmap](../ROADMAP.md), [fresh probes](market-research-probes-2026-11-10.json)

### Markdown expectations need a clearer contract

The fresh probes exposed cases where successful generation may surprise users:

| Input | Observed behavior in the copied 0.2.0 executable | Product consequence |
| --- | --- | --- |
| YAML front matter followed by a chapter | Metadata-looking text becomes visible content in an extra preamble chapter | Imported Markdown needs an explicit front-matter policy |
| `# Book title` followed by `## First chapter` and `## Second chapter` | One EPUB chapter | Users need a clear split-level rule and eventually an explicit option |
| `~~obsolete~~` | Tildes remain literal | Users expecting GitHub Markdown may misread the support promise |
| `$x^2$` and `$$y=2$$` | Delimiters remain literal | The tool should declare that it does not render math |
| GitHub `[!WARNING]` blockquote | Marker remains visible in an ordinary blockquote | Agent-authored material may need preparation |
| Broken heading fragment | Build fails with a useful message | Good failure behavior, with room for structured locations |

Literal handling of these extensions follows the documented CommonMark boundary; it is not evidence that supported content disappeared. The opportunity is to expose that boundary more precisely and detect high-confidence mismatches. Avoid blanket warnings for dollar signs or tildes that are intended as prose. [Supported subset](../USAGE.md), [probe inputs and XHTML](market-research-probes-2026-11-10.json)

### Structured errors are too coarse for reliable repair

JSON errors have four broad categories and a message string. Some messages already include file and line information, but those locations are not separate fields. External validator output remains on stderr. An integrating agent still has to interpret prose to identify unsupported tables versus unresolved links. [Agent interface](../AGENT_INTERFACE.md), [result implementation](../../src/agent.rs)

### Installation and first impressions trail the product

The reviewed README leads with milestone 1, then explains milestone 2 later. Its installation route requires building with Rust. The reviewed checkout has a check workflow, but no release-binary workflow or documented binary install path. `Cargo.toml` lacks license and repository metadata, and the tracked release tree contains no project license file. These are concrete adoption tasks before promoting reuse. They do not establish what may exist outside this checkout. [README](../../README.md), [package metadata](../../Cargo.toml), [CI](../../.github/workflows/ci.yml)

### Presentation and publication controls are minimal

There is one built-in stylesheet, basic metadata and no integrated visual preview. Identifiers are random and modification times are generated. Two identical fresh requests produced different archive hashes. Reproducibility and stable publication identity therefore need explicit controls if repeated publishing becomes a target. [Package implementation](../../src/package.rs), [repeat-build evidence](market-research-probes-2026-11-10.json)

The language tag and Unicode handling do not establish tested RTL layout, multilingual typography or screen-reader usability. The existing reader checks cover a limited set of conditions.

### The differentiation is inexpensive to imitate

A small wrapper around Pandoc can implement a JSON request and compact results. The durable advantage would come from a useful corpus, accurate diagnostics, reliable content preservation and easy distribution. The current Codex evaluation is encouraging, but 20 synthetic cases in one session do not establish broad reliability or superiority over alternatives. Claude and Ollama evaluations remain explicitly deferred.

More information: [probe evidence](market-research-probes-2026-11-10.json) records observed behavior; [evaluation methods](../verification/AGENT_EVALUATIONS.md) explain the existing success measure.

## Opportunities worth pursuing

1. **Become the EPUB export component in agent applications.** Make a streamed request the shortest supported route. Ship one small invocation recipe that reports the artifact and unrun checks correctly. Measure integration time and repeat use.
2. **Handle ordinary reports without manual simplification.** Finish tables, linked notes and nested contents. Add local inline images with supplied alternative text next if benchmark documents or GitHub issues establish that need. The win is useful output from existing writing.
3. **Make content fidelity inspectable.** Produce a build manifest listing source-to-chapter mapping, content counts, transformations and warnings. Counts can detect problems but cannot prove that meaning or order is preserved. Add explicit corpus checks for those properties.
4. **Help agents repair failures accurately.** Introduce stable issue codes, source references, locations and suggested remedies. Add a preflight command that returns multiple independent input problems without producing an EPUB.
5. **Offer dependable reading defaults.** A small selection such as report and prose styles could help. Test large text, narrow screens and reader overrides. A phone-readable table policy could matter more than decorative themes.
6. **Support distribution through existing developer workflows.** Prebuilt binaries, a reusable agent skill and a GitHub Action can each reduce work at a different adoption point. Build an MCP adapter only when target applications need that transport.

Independent validation and accessibility should remain distinct. EPUBCheck evaluates conformance; Ace checks automated accessibility issues and also supports manual review. Neither establishes every aspect of reading quality. Prefer optional integrations and exact check status to expanding the built-in checker into a universal validator. [EPUBCheck](https://www.w3.org/publishing/epubcheck/), [Ace by DAISY](https://kb.daisy.org/publishing/docs/epub/validation/ace.html)

More information: [Pandoc EPUB examples](https://pandoc.org/epub.html) show support for local images, metadata, CSS and math; [Sigil](https://sigil-ebook.com/sigil/) provides a useful manual inspection tool.

## Ranked improvements and low hanging fruit

Effort estimates are planning judgments for one maintainer familiar with the code. They include relevant verification and are not commitments. Independent tasks can overlap, so the estimates should not be added into a promised schedule.

| Priority | Improvement | Likely effort | Payoff and completion condition |
| --- | --- | --- | --- |
| P0 | Rewrite the README around the released workflow | 0.5 day | A newcomer can discover, build and inspect a streamed request from the first screen; include supported content and a downloadable sample |
| P0 | Declare the intended license and complete package metadata | 0.5 day plus owner decision | A potential adopter can evaluate reuse and find the project; the maintainer chooses the license |
| P0 | Publish prebuilt release archives and checksums | 2–4 days | Fresh macOS, Linux and Windows machines can run the CLI without Rust; smoke-check every claimed target |
| P0 | Ship a short reusable agent skill and examples | 0.5–1 day | A developer can integrate discovery, build and result handling without inventing instructions |
| P0 | Finish the selected milestone 3 conveniences | Remaining effort depends on current work | Tables, notes and nested navigation pass regression, EPUBCheck and reader checks; record actual release status |
| P1 | Add detailed diagnostic fields | 1–2 days | Unsupported tables and broken links have distinct codes and machine-readable source locations; preserve existing clients |
| P1 | Declare the Markdown dialect and front-matter policy | 0.5–1.5 days | Capabilities state extension behavior; recognized front matter receives a clear policy rather than surprising content |
| P1 | Add an explicit chapter split-level option | 1–2 days | A title plus level-two chapters works predictably; existing defaults remain compatible |
| P1 | Build a shared real-world comparison corpus | 2–4 days | Results compare Agent EPUB and Pandoc plus the same checks; add mdbindery for repository-based cases |
| P1 | Add local inline images with required alt policy | 3–5 days | Typical guides can retain diagrams, with bounded decoding, clear asset paths and reader checks |
| P2 | Add an aggregate preflight command | 2–4 days | Returns multiple actionable issues and the proposed chapter map, without publishing output |
| P2 | Add stable identifier and reproducible-build controls | 1–3 days | Identical declared inputs/settings produce matching archives; default identity behavior remains documented |
| P2 | Add a build manifest with content-preservation evidence | 2–4 days | Callers can inspect chapter mapping and transformations; reports distinguish count checks from fidelity claims |
| P2 | Add a few tested style presets and preview fixtures | 2–4 days | Samples remain readable in named readers at narrow widths and large text |

The fastest useful changes are the README, integration recipe, clearer capability declarations and diagnostic fields. Binary distribution requires more verification but likely removes a larger barrier than another formatting flag. Tables and notes are essential scope work for the proposed audience, rather than effortless additions.

For front matter, an initial detection error is cheaper than supporting a second metadata system. Later support should define precedence against request metadata and CLI flags. For diagnostics, retain the existing broad category while adding a detailed code to avoid breaking callers.

More information: [agent request schema](../../schemas/book-request.v1.json) defines compatibility constraints; [roadmap](../ROADMAP.md) defines the existing milestone 3 gates.

## Demand validation and commercial options

The current evidence supports technical feasibility and a plausible niche. It does not yet support a revenue forecast. Prices for author applications show that people pay for publishing convenience, but those buyers purchase a different workflow from a CLI component. [Vellum pricing](https://vellum.pub/), [Atticus pricing](https://www.atticus.io/)

Use the agent workflow benchmark after milestone 5 to measure setup time, tool calls, retries, manual repairs and time to a correct EPUB against Pandoc. Collect feedback through [GitHub issues](https://github.com/benvds/epub-rust/issues). Useful reports include a version, command or request, expected versus actual behavior and a minimal non-private reproduction. The plan does not require a recruited user study.

Use explicit decision criteria for the benchmark and issue-driven priorities. Suggested thresholds below are product targets, not measured market facts:

- Typical supported material reaches a first useful EPUB within ten minutes on a clean machine using a release binary.
- At least 90% of a published corpus completes without manual source rewriting, with no known supported-content loss.
- The proposed agent contract reduces failed attempts, manual intervention or integration code relative to a reasonable Pandoc wrapper.

Benchmark the complete workflow, including discovery, installation where relevant, source preparation, generation, promised validation and repairs. Use equivalent EPUBCheck conditions. Measure runtime separately from agent calls, human interventions, output fidelity and visual review. Include failures rather than comparing successful conversion times alone.

If benchmark results or reproducible issues expose adoption problems, improve the entry point before expanding scope. If a reasonable existing-converter workflow performs as well, determine whether the remaining value belongs in a wrapper or integration rather than a separate engine. GitHub issue activity alone does not measure market size or repeat use.

The most plausible commercial extensions are supported integration, managed conversion or team workflows with retained reports and policies. These require operational benefits beyond a free local converter. Keep hosted billing and infrastructure behind concrete demand; requests through GitHub issues can inform later decisions without creating a separate research milestone. Author-app prices do not establish an appropriate CLI price.

More information: [evaluation harness](../../scripts/evaluate_agents.py) offers a starting point for task scoring; [existing evaluation](../verification/AGENT_EVALUATIONS.md) explains first-artifact scoring.

## Recommended sequence

First, make the released product easy to understand and install. Complete the README, license decision, release packaging and reusable integration instructions. In parallel, finish the already selected milestone 3 scope rather than opening another major format.

Then address the observed Markdown surprises and detailed diagnostics. Run real source documents through the tool and a reasonable Pandoc alternative. Use those results to choose between inline images, preflight reporting and additional reading styles.

Keep PDF, DOCX import, fixed layout, multimedia, DRM, a full editor, a universal validator and a hosted service outside the immediate release. They add new user jobs and support obligations. The next product test is whether a developer can adopt this small EPUB component quickly and use it repeatedly.

More information: [ranked roadmap](../ROADMAP.md) provides the existing scope boundary; the priorities above add adoption and comparison work without changing that roadmap.

## Source register

All external sources were accessed on 10 November 2026. Links next to claims identify the supporting source.

- [Pandoc manual](https://pandoc.org/MANUAL.html): conversion, discovery, logging and command options.
- [Pandoc EPUB tutorial](https://pandoc.org/epub.html): EPUB workflow, metadata, images, styles and math.
- [mdbindery repository](https://github.com/sagol/mdbindery): content features, input workflows and quality gates.
- [mdbindery website](https://sagol.github.io/mdbindery/): agent integration, preview and installation information.
- [calibre CLI](https://manual.calibre-ebook.com/generated/en/ebook-convert.html): conversion options and device profiles.
- [Quarto books](https://quarto.org/docs/books/): project structure and output formats.
- [Crowbook](https://github.com/lise-henry/crowbook): Rust CLI workflow and fiction focus.
- [mdbook-epub](https://github.com/michael-f-bryan/mdbook-epub): mdBook backend and configuration.
- [EbookLib tutorial](https://docs.sourcefabric.org/projects/ebooklib/en/latest/tutorial.html): programmatic book construction.
- [epub-builder](https://docs.rs/epub-builder/latest/epub_builder/): Rust packaging library and caller responsibilities.
- [Sigil](https://sigil-ebook.com/sigil/): EPUB editing and preview capabilities.
- [Vellum](https://vellum.pub/): publishing workflow, platform and prices.
- [Atticus](https://www.atticus.io/): publishing workflow, platforms and one-time price.
- [EPUBCheck](https://www.w3.org/publishing/epubcheck/): independent EPUB conformance checking.
- [Ace by DAISY](https://kb.daisy.org/publishing/docs/epub/validation/ace.html): accessibility checking and review.
- [epubveri](https://github.com/veripublica/epubveri): native/browser validation and distinction from EPUBCheck.
- [Local probe evidence](market-research-probes-2026-11-10.json): twelve behavior probes, binary identity and two repeat builds.
- [Release verification](../verification/MILESTONE_2.md) and [agent evaluation](../verification/AGENT_EVALUATIONS.md): existing local product evidence.
