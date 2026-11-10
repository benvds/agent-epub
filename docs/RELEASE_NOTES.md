# Agent EPUB v0.6.0

Include local illustrations in your agent's reading editions using ordinary
Markdown: `![Description](assets/image.png)`. Version 1 requests remain unchanged;
images work through direct files, request files and streamed inline content.

- Bundle static PNG/JPEG bytes unchanged, detect the actual format and reuse
  repeated canonical paths. Images work in headings, lists, table cells and notes
  alongside chapter splitting, nested contents, tables, footnotes and covers.
- Require supplied alt text, or an explicit `![](path "decorative")` choice for
  decorative images. Images adapt to the reading width; image tables have scoped
  sizing that also works with the tested reader's image overrides.
- Resolve assets from source-file parents or the request base for inline content.
  Streamed inline images require `--base-dir`. URLs, rooted paths, SVG, animation
  and direct image-file hyperlinks remain unsupported. Use a heading link to
  navigate to an illustration.
- Enforce 10 MiB / 8,000 pixels per side / 16 million pixels per image, 128 unique
  inline files and 64 MiB aggregate inline data. Missing, corrupt, unsupported or
  undescribed images return localized repair diagnostics and preserve existing
  output. Image inputs are protected against output aliasing.
- Discover formats, path rules and limits through capabilities. Successful JSON
  results add `images`, the unique inline-resource count excluding the cover.

Try the **[showcase EPUB](https://github.com/benvds/agent-epub/releases/download/v0.6.0/agent-epub-showcase-0.6.0.epub)**
or compare its [Markdown source](https://github.com/benvds/agent-epub/blob/v0.6.0/examples/showcase.md).
The complete four-chapter example includes a cover, automatic contents, headings
1–6, lists, quotations, code, tables, shared notes, PNG/JPEG images, decorative
markup and a literal-text appendix. The source, request and generated EPUB are
also bundled in every native ZIP and committed in the repository.

All 49 ordinary macOS regression tests pass, with platform-appropriate tests in
native CI. Official EPUBCheck 5.3.0 covers the text, cover and illustrated corpora
and the committed showcase. Apple Books 9.0 and EPUB.js 0.3.93 reader observations
include illustrations and enlarged text; browser checks include a 360-pixel
reading area. See the [verification record](https://github.com/benvds/agent-epub/blob/v0.6.0/docs/verification/INLINE_IMAGES.md)
for coverage and remaining manual review. Accessibility conformance, physical
devices and separate mobile-app review are not claimed.

Download and extract the ZIP for macOS Apple Silicon, Linux x86_64 or Windows
x86_64, verify its `.sha256` file or `SHA256SUMS`, and follow `docs/INSTALL.md`.
`SHA256SUMS` also covers the standalone showcase EPUB. Native build/install gates
must pass before publication; attached smoke reports identify exact archive
hashes. Generation remains offline and needs no Rust, Python, Java or AI service.
Optional independent EPUBCheck is installed separately. MIT licensed.

The practical three-pair agent benchmark remains pinned to v0.5.0. Its cold
workflow advantage and mixed warm results do not measure v0.6.0 image workflows.
Claude and Ollama live evaluations remain deferred.
