# Agent EPUB tool roadmap

Updated 10 November 2026. The first three milestones delivered a useful, tested EPUB subset. Milestone 4 adds installation and agent adoption at `v0.4.0`. The remaining milestones improve input handling and diagnostics and test its value against alternatives. Dates and future release versions are intentionally unset; each milestone ends when its acceptance gates pass.

The initial audience is developers adding EPUB export for prose-heavy reports and learning guides to an agent workflow. Priorities follow the [market research](research/market-research-2026-11-10.md). Broader publishing features require evidence from actual source documents and repeated use.

## Milestone 1 Text and Markdown reading editions

Status: implementation and automated checks complete at `v0.1.0`. Subsequent reader checks exercise the same reading features in the milestone 2 build. See the [milestone 1 record](verification/MILESTONE_1.md) and [milestone 2 reader record](verification/MILESTONE_2.md).

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

Status: complete for the user-approved release scope at `v0.2.0`. Generic requests, streamed input, discovery, structured results, JPEG/PNG covers, reader checks and the live OpenAI-agent evaluation are complete. Claude and Ollama evaluations are deferred at the user's request. See the [verification record](verification/MILESTONE_2.md).

Let different agents create books through one predictable, unattended workflow, and add a supplied cover image.

Deliver:

- A versioned book request with metadata, explicit ordered chapters and inline content or file references.
- Request input from a file or a stream, without provider-specific conversation parsing.
- Capability discovery and compact structured success, warning and error results.
- Explicit behavior for missing information, conflicting options and repeated requests.
- Supplied JPEG or PNG covers, correct cover identification and image validation.
- Provider-neutral examples and a live evaluation using the ChatGPT-authenticated OpenAI Codex CLI. Claude and Ollama evaluations are explicitly deferred for this release.

Exit gates:

1. All milestone 1 regression checks continue to pass.
2. Equivalent direct-file and structured requests produce equivalent reading content and navigation.
3. Invalid requests, paths and cover files produce stable, actionable outcomes.
4. Covers work in independently validated files and tested readers.
5. The evaluated OpenAI environment meets the PRD completion target, with versions and results recorded. Claude and Ollama are excluded from the release gate at the user's request; no live compatibility result is claimed for them.

## Milestone 3 Selected conveniences

Status: complete for the selected scope at `v0.3.0`. Simple semantic Markdown tables, linked footnotes with return links, and nested section contents work through the existing file and agent request inputs. Full EPUB support remains outside scope. See the [verification record](verification/MILESTONE_3.md).

Delivered from the original candidates: simple semantic tables, linked notes and deeper contents. Inline images, richer metadata, styles, accessibility reports, batch generation and reproducible archives remain candidates in the conditional backlog below.

Exit gates for each chosen addition:

- It has a documented input boundary, fallback behavior and failure examples.
- It preserves content and passes the existing navigation and conformance corpus.
- It has relevant reader checks and does not undermine the speed targets.
- Its value justifies the added agent instructions and maintenance burden.

## Milestone 4 Installation and agent adoption

Status: implementation and native installation gates complete at `v0.4.0`. Priority: P0. Make the released workflow usable without a Rust toolchain or custom integration instructions. See the [milestone 4 record](verification/MILESTONE_4.md).

| Work package | Deliverable | Estimated effort | Dependencies |
| --- | --- | --- | --- |
| A1 Current documentation and samples | Lead the README with the current release, a streamed request, result handling, supported content and a sample EPUB with its source. Keep validation coverage explicit. | 0.5 day | Released milestone 3 behavior |
| A2 Licensing and package metadata | Record the maintainer's license choice; add its file and package license, repository and documentation metadata as applicable. | 0.5 day plus maintainer decision | Maintainer selects intended reuse terms |
| A3 Release binaries | Build release archives and checksums for explicitly supported macOS, Linux and Windows targets; document installation and updates. | 2 to 4 days | A2 before distributing under the selected terms |
| A4 Reusable agent instructions | Ship a short agent skill and direct, file-request and streamed examples covering discovery, build, result inspection and failed or unrun checks. | 0.5 to 1 day | A1 contract and examples; A3 installation instructions before final verification |

Exit gates:

1. Every claimed binary target has a recorded clean-install smoke check that builds and validates a sample without Rust. The independent EPUBCheck dependency is documented separately.
2. A newcomer can follow the README from installation to a usable artifact and understand which checks ran.
3. The sample and instructions match the current release, including tables, notes and nested contents.
4. License files, package metadata, release archives and checksums agree with the maintainer's decisions.
5. Agent instructions do not claim untested provider support. Claude and Ollama live evaluations remain deferred at the user's request.

## Milestone 5 Predictable inputs and actionable diagnostics

Status: planned. Priority: P1. Reduce source rewriting and agent retries while preserving existing defaults and version 1 callers.

| Work package | Deliverable | Estimated effort | Dependencies |
| --- | --- | --- | --- |
| B1 Detailed diagnostics | Add stable issue codes, source references, available line/column locations and actionable remedies. Retain the existing broad error categories and clean JSON stdout. | 1 to 2 days | Existing request/result contract |
| B2 Markdown dialect and front matter | Audit current capability declarations against the research cases. Declare literal versus rendered extensions and define a front-matter policy. Initially, an actionable rejection of recognized front matter is acceptable. | 0.5 to 1.5 days | Current milestone 3 capabilities; reuse B1 fields when available |
| B3 Explicit chapter splitting | Add a documented split-level choice to direct and structured inputs, including title-plus-H2 manuscripts, while retaining the existing default. | 1 to 2 days | Backward-compatible schema design and documented chapter rules |

Exit gates:

1. Unsupported content and unresolved links have distinct detailed issue codes and machine-readable source locations where available. Existing broad categories remain usable.
2. Capability declarations, help, schema and examples describe the same behavior. Recognized front matter has an explicit outcome; ordinary prose containing dollar signs or tildes does not trigger blanket extension warnings.
3. Any future front-matter import defines precedence against request metadata and CLI flags before implementation.
4. Split-level fixtures cover preambles, skipped levels, duplicate headings, internal links and equivalent direct/structured requests. Existing default output behavior remains covered.
5. Relevant regression and independent EPUBCheck checks pass; changed reading structure has recorded reader checks.

## Milestone 6 Comparative evidence and user pilots

Status: planned. Priority: P1. Establish whether the focused contract saves enough work to justify adopting another EPUB tool.

| Work package | Deliverable | Estimated effort | Dependencies |
| --- | --- | --- | --- |
| C1 Shared comparison corpus | Compare current Agent EPUB with a reasonable Pandoc wrapper using the same promised checks. Add mdbindery for repository-based cases. Publish versions, configuration, outcomes and reproducible instructions. | 2 to 4 days | Current baseline can start now; A3 and milestone 5 for final adoption measurements |
| C2 Observed user pilot | Recruit five to eight prospective report/guide users, observe their current export work and follow repeated use for two weeks. Record actual documents, blockers and manual repairs with permission; commit only synthetic or approved material. | Two-week observation window; recruitment effort unknown | A1, A3 and A4 for unassisted adoption; C1 scoring method |

Evaluate installation and first export, agent attempts, manual interventions, content fidelity, reading behavior and repeat use. Time generation and independent validation separately. Include failures and compare equivalent validation conditions; successful conversion time alone is insufficient.

Proposed decision targets:

- At least three pilot users generate a second EPUB within two weeks without maintainer assistance.
- A typical supported document reaches a first useful EPUB within ten minutes on a clean machine using a release binary.
- At least 90% of the published task corpus completes without manual source rewriting, with no known loss of supported content.
- The contract reduces failed attempts, manual intervention or integration code relative to the Pandoc baseline.

Exit gates:

1. Publish observed results against each target, including misses, unsupported cases and remaining uncertainty. These targets are hypotheses, not current performance claims.
2. Compare text, links, order and requested structures independently of each generator's success report. Counts alone do not prove fidelity.
3. Select, defer or reject each conditional candidate using recorded blockers and repeated-use evidence.
4. Record the resulting product decision: continue the focused engine, improve its integration, narrow its audience or reconsider scope. Missing a target prompts a decision rather than an automatic feature expansion.

## Conditional backlog after the initial adoption work

These candidates are not committed release scope. Select a small package after reviewing milestone 6 evidence; an earlier selection needs a concrete workflow and recorded rationale.

| ID and priority | Candidate | Estimated effort | Evidence and acceptance boundary |
| --- | --- | --- | --- |
| D1 P1 | Local inline images | 3 to 5 days | Pilot guides repeatedly need diagrams. Define source-relative assets, supplied alternative text or an explicit decorative choice, bounded decoding, missing-resource failures and reader checks. Keep remote fetching outside scope. |
| D2 P2 | Aggregate preflight | 2 to 4 days | Agents repeatedly repair one issue per build. Depends on B1 and B2; return multiple independent issues and a proposed chapter map without publishing output. |
| D3 P2 | Stable identity and reproducible archives | 1 to 3 days | A repeated publishing or CI workflow needs stable comparisons. Define identifiers, modification time, ZIP metadata and relevant tool versions; verify byte-identical archives under declared settings. |
| D4 P2 | Content-preservation manifest | 2 to 4 days | Callers need inspectable source-to-chapter mapping, counts and transformations. Reuse C1 fidelity checks and B1 issue conventions; distinguish evidence from a guarantee of preserved meaning. |
| D5 P2 | A few reading styles and preview fixtures | 2 to 4 days | Users identify a presentation problem. Test report/prose defaults in named readers with narrow widths, large text and user font overrides, including wide-table behavior. |
| D6 P2 | Optional accessibility reporting | Estimate after coverage is agreed | A user needs automated review evidence. Integrate existing tools where appropriate and report unrun checks and remaining manual review; do not claim certification. |
| D7 P2 | Richer publication metadata | Estimate after fields are selected | Distribution needs establish specific fields such as rights, description or multiple contributors. Define schema compatibility and metadata verification; coordinate stable identity with D3. |
| D8 P2 | GitHub Action or MCP adapter | Estimate for the selected integration | A target workflow needs CI installation or an MCP transport. Prefer a thin adapter over the existing CLI contract and verify end-to-end outcomes. |
| D9 Deferred | Batch generation | Estimate after workflow measurement | A repeated-generation task benefits beyond a caller loop. Define per-book results, failures and output protection before adding batch semantics. |

PDF output, DOCX import, fixed layout, multimedia, DRM, a full editor, universal EPUB validation and a hosted service remain outside the immediate plan. Supported integration or managed/team workflows are commercial hypotheses to revisit after repeat use and buyer interviews establish demand; author-app prices do not establish a CLI price.

## Planning and release rules

Effort ranges are initial estimates for one maintainer familiar with the code, including relevant verification. They exclude recruitment and do not imply calendar commitments. Re-estimate before selecting a work package. A1 and A2 can start independently; A2 precedes release distribution, B1 supports later preflight work, and comparative corpus design can begin before the installation work finishes.

Keep the existing streamed request/result contract, offline generation, explicit overwrite behavior and protected output publishing throughout. Schema additions must preserve existing callers or use an explicitly versioned migration. Keep structural checks, EPUBCheck, accessibility automation and reader review distinct in every result.

Each selected package needs a documented input boundary, failure cases, focused verification and an updated completion record. Run the regression and independent conformance corpus when generation changes; record reader checks when content or presentation changes. Avoid treating documentation checks as proof of implementation behavior.

## Progress and checkpoints

The implementation plan breaks milestones into small commits using the work-package IDs above. Commit planning documents before implementation, then commit runnable foundations, generation behavior and verification separately. Keep unfinished acceptance gates visible rather than declaring a milestone complete from code tests alone. Push and publish only within the user's authorized scope.

For current scope and implementation sequence, see the [PRD](PRD.md) and [Rust CLI implementation plan](IMPLEMENTATION_PLAN.md).
