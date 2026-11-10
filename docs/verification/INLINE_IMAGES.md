# D1 Local inline images verification

Updated 10 November 2026. D1 was selected as the next P0 delivery at the
maintainer's request and is delivered for v0.6.0. The development implementation,
automated conformance and browser checks below were followed by an Apple Books
review of the v0.6.0 showcase after the Mac was unlocked. This closes the
second-reader gate. Publication is gated by the tag workflow's native build,
clean-install and independent-validation jobs; their smoke reports accompany
[the release](https://github.com/benvds/agent-epub/releases/tag/v0.6.0).

## Delivered behavior

Static local JPEG/PNG images work in Markdown paragraphs, headings, lists,
table cells and notes through direct files, file requests and streamed inline
requests. Original encoded bytes are preserved, actual data determines media
type, and repeated canonical paths reuse one inline resource. Cover identity
and chapter counts remain separate. Version 1 request fields are unchanged;
JSON success/validation results add `images`, excluding the cover.

Paths resolve from actual source parents; inline assets use the request file's
directory or explicit base override. Streamed inline images require
`--base-dir`. Parent paths and symlinks follow normal file resolution; this is
not a directory sandbox. Limits are 10 MiB / 8,000 pixels per side / 16 million
pixels per image, 128 unique inline files and 64 MiB total inline bytes. The
cover retains its separate per-image budget.

Nonempty alt is required unless empty brackets and the exact title
`"decorative"` request empty alt plus `role="presentation"`. Normal titles are
preserved; alt formatting becomes text. This follows the explicit decorative
choice described in [DAISY's image guidance](https://kb.daisy.org/publishing/docs/html/images-deco.html).
Description quality and complete accessibility conformance still require review.

Remote/data/file URLs, absolute paths, backslashes, queries/fragments, SVG and
animation, including APNG, are rejected. Direct image-file hyperlinks are also
rejected; use image syntax or link to a containing heading. EPUBCheck exposed
an RSC-010 error in an early example's PNG hyperlink, so that unfinished fallback
behavior was removed from the scope before acceptance. No remote fetching,
caption generation, fixed layout or image generation is added.

## Automated verification

Development verification used macOS 27.2 ARM64 with Rust 1.95 and the
`0.6.0-dev` package below. Release preparation repeats these checks for v0.6.0:

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
EPUBCHECK=/path/to/official-wrapper cargo test --locked independent_ -- --ignored
cargo build --release --locked
python3 scripts/release.py pack --binary target/release/agent-epub \
  --target aarch64-apple-darwin --out /tmp/agent-epub-image-dist
python3 scripts/release.py smoke \
  --archive /tmp/agent-epub-image-dist/agent-epub-0.6.0-dev-aarch64-apple-darwin.zip \
  --report /tmp/agent-epub-image-dist/smoke.json
```

Formatting and Clippy pass. All 49 ordinary tests pass; three external-validator
corpus tests are deliberately ignored in the ordinary run and run separately.
Official EPUBCheck 5.3.0 passes 16 text/mixed fixtures (including split levels
1–6), four cover fixtures and three illustrated fixtures. The illustrated
corpus includes both image types with a cover, repeated assets, tables, notes,
decorative markup and the actual bundled guide. The guide's explicit validator
report contains zero errors and zero warnings.

Image regression checks cover Unicode/space paths, parent paths and symlink
reuse, multiple sources with equal asset names, all three input modes, byte
preservation, heading labels/ranks, XML escaping, missing/corrupt resources,
unsupported destinations, alt policy, byte/dimension/animation/count/aggregate
limits, localized Unicode columns, JSON errors, output preservation, and output
aliases including hard links on Unix. Damaged-package tests cover missing
resources, false media types, corrupt data, invalid alt/decorative markup and
remote/non-image sources. Literal code image markers load no assets.

The native macOS development archive passes clean-install smoke with Rust and
Java absent from PATH, including illustrated input-mode resource equivalence,
image counts, covers, ordinary report/H2 behavior, diagnostics and failed/unrun
checks. CI adds the illustrated EPUBCheck corpus and exercises packaging and
clean installation on the existing three native targets. The first native CI run exposed a Windows rooted-path discrepancy: `/tmp/x`
is rooted but not fully absolute on Windows. Loading now rejects all rooted
paths, matching the declared relative-only profile on every platform. The
existing absolute-path diagnostic case exercises this correction in native CI.
Native CI on implementation commit `243b406` passes all seven required jobs:
regression/conformance, packaging on macOS Apple Silicon/Linux x86_64/Windows
x86_64, and clean installation on each target. Release publication is skipped
because this is a main-branch development build.
[CI run and job reports](https://github.com/benvds/agent-epub/actions/runs/42805678039).
The subsequent documentation-only checkpoint records this verified commit.

## Reader observations

EPUB.js 0.3.93 with JSZip 3.10.1 loaded the synthetic illustrated guide in the
in-app browser. At ordinary width and a 360-pixel reading area with 180% text:

- Cover, PNG and JPEG decode, chapter contents work, and the shared illustrated
  note and its return to the second referring chapter navigate correctly.
- Main and note PNGs shrink from 1,200 pixels to 330 pixels wide, retaining their
  4:1 aspect ratio. The 900 × 600 JPEG renders at 330 × 220.
- The table is 330 pixels wide; its repeated image is about 146 × 37 inside a
  164-pixel cell. Paginated screenshots show the image within its cell.
- The divider is decoded with empty alt and presentation role and is absent
  from the accessibility image list; meaningful alt is exposed for other images.

An initial reader check found table overflow. Image-containing tables now use
fixed column layout, and a scoped maximum-width rule counters the reader's
pixel-based image override inside cells. Ordinary text-only tables keep their
previous layout policy. Large or complex tables still need reader review.

The earlier Apple Books check was blocked by a locked Mac. During v0.6.0
release preparation the Mac was unlocked and Apple Books 9.0 opened the
regenerated showcase. PNG and JPEG illustrations, the reused table image and
decorative divider render; native contents and visible section links navigate.
Increasing font size one step reflows the text and table without image overflow;
the original size was restored. The diagram note opens as a native popup with
its reused image; Escape closes it and returns to the reading position. EPUB.js
provides the separately observed explicit cross-chapter note-return coverage.
These checks use actual v0.6.0 reading resources; earlier milestone reader
results are not substituted for image evidence.

Physical-device, separate mobile-app, screen-reader and full accessibility
reviews remain outstanding. The narrow browser check is not a substitute for
those checks.

## Practical timing

The [30-run converter measurement](inline-images-benchmark.json) uses the
existing 100,000-word/40-chapter corpus with 40 tables, 40 notes, 80 references,
80 section entries and the 18,755-byte synthetic JPEG cover. Median is
23.059 ms, p95 25.777 ms and maximum 26.415 ms on this local machine. These
observations remain well within the existing converter timing targets; they do
not measure near-limit illustrations.

The [three-run illustrated example](inline-images-example-timing.json) has three
chapters, three unique inline images and a cover. Samples are 18.603, 15.587 and
16.310 ms, median 16.310 ms. Both measurements include process startup, reading,
decoding, generation, built-in checking and publication with warm local caches.
They exclude EPUBCheck, writing, transfers and agent interaction. These small
local observations are not general speed claims or a new comparison with Pandoc.
The historical milestone 5a benchmark remains pinned to v0.5.0.

## Release preparation

The maintainer requested v0.6.0 publication after implementation. Version,
installation commands, current docs, feature matrix, bundled agent instructions
and release notes now describe the image release. The committed showcase was
regenerated with v0.6.0 and independently validated with zero errors or warnings.
The release pipeline validates that exact committed EPUB and publishes it as
`agent-epub-showcase-0.6.0.epub`, alongside three native archives, per-archive
checksums, smoke reports and `SHA256SUMS`. The aggregate checksums cover the
standalone EPUB too. Native package archives regenerate their own showcase
from the same source and request.

Release checks use formatting, Clippy, all ordinary regression tests, all three
EPUBCheck corpora, the committed showcase and native clean-install smoke.
The pre-release development timing data above and their original tool version
are retained as measured evidence, rather than relabeled as v0.6.0 timings.

## Repository feature showcase

The [showcase source](../../examples/showcase.md),
[request](../../examples/showcase-request.json) and
[committed EPUB](../../examples/showcase.epub) were added on 10 November 2026
and linked from the README. The book was initially generated with `0.6.0-dev`,
then regenerated for v0.6.0, and passes
built-in validation and official EPUBCheck 5.3.0 with zero errors or warnings.
It contains four chapters, 18 contents entries, heading levels 1–6, two tables,
five note references sharing two definitions, three unique inline image assets
and a separate cover. Inspection confirms original image bytes and literal
appendix text are retained. This is an additional example check, separate from
the three-fixture illustrated corpus above. The Apple Books release review
uses this regenerated showcase, as described above.

Release packaging regenerates the showcase from the bundled source/request;
clean-install smoke validates its chapter, image and cover results with Rust
and Java absent from PATH. The local macOS package and smoke checks pass.
The ignore rule makes an explicit exception only for `examples/showcase.epub`.
