# D1 Local inline images verification

Updated 10 November 2026. D1 was selected as the next P0 delivery at the
maintainer's request. Implementation is on main as `0.6.0-dev`; published
v0.5.1 remains cover-only. Automated and browser checks pass. A second-reader
check is pending because the Mac was locked when Apple Books was requested.
This record does not close that gate or claim a new release.

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

On macOS 27.2 ARM64 with Rust 1.95:

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
Final remote CI results are recorded below after the push.

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

Apple Books testing is **not run for this feature**: native app control reported
a locked Mac. Earlier milestone Books evidence is not reused as image evidence.
Physical-device, separate mobile-app, screen-reader and full accessibility
reviews also remain outstanding. The narrow browser check is not a substitute
for those checks.

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

## Remaining gate

Run the illustrated guide in Apple Books after unlocking the Mac, record image,
contents, note-return and enlarged-text observations, and review native CI.
A separate release decision/version/tag is still required before published
binary users receive chapter images. Current [usage](../USAGE.md#local-chapter-images),
[agent contract](../AGENT_INTERFACE.md), capability discovery, synthetic examples,
packaging and bundled agent instructions describe the development behavior.
