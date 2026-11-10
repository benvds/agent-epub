# Usage and supported content

Build with `cargo build --release --locked`. The executable is `target/release/agent-epub` (`.exe` on Windows). File generation needs no AI service, network access, Python or Java. Independent EPUBCheck validation needs a separately installed validator.

## Generate a book

```sh
agent-epub build --input first.txt --input chapters.md \
  --output book.epub --title "My book" --language en \
  --author "An author"
```

Repeat `--input` in reading order. Supported extensions are `.txt`, `.md` and `.markdown`, case-insensitively. Sources must be UTF-8. A leading byte-order mark and conventional line endings are normalized. Title and language are required; author is optional. Supply a valid BCP 47 language tag, such as `en`, `nl` or `en-US`.

Each text file becomes one chapter. Blank lines separate paragraphs, and line breaks within a paragraph stay visible. Text is literal, including angle brackets and Markdown markers.

Each Markdown file begins a chapter. Top-level `#` headings start additional chapters. Text preceding the first heading stays in a chapter named after the file. A heading inside a quotation does not split the surrounding chapter. Empty top-level headings get a `Chapter N` label. Duplicate titles remain distinct contents entries.

Supported Markdown: headings, paragraphs, strong/emphasized text, lists, quotations, inline/fenced code, thematic breaks, line breaks, links, simple pipe tables and defined footnotes. Inline images, raw HTML and task lists produce explicit errors. Other extensions are not advertised as supported; ordinary CommonMark interpretation applies.

## Tables

Use a header row and a Markdown separator row, with optional `:---`, `:---:` or `---:` alignment. Every data row must have exactly the header's number of cells. Use `\|` for a literal pipe, including inside code spans. Ragged rows fail with the source path and line number. For example, `| A | B |` followed by `|---|---|` and `| only A |` fails instead of padding or discarding cells.

Tables must be outside lists and quotations. Cells support ordinary inline Markdown, including links. Output uses column header cells with `scope="col"`. Merged cells, multiline cells, captions and complex row-header relationships are outside the supported subset. Wide tables still need reader review.

Limits: 20 MiB per source, 64 MiB total source text and 1,000 chapters. This is a focused reflowable EPUB 3 profile. Covers, inline illustrations, arbitrary HTML import and full EPUB feature coverage are not part of milestone 1.

## Footnotes

Write `A claim[^source].` with a separate definition such as `[^source]: Supporting explanation.` Indent continuation paragraphs by four spaces. Definitions are local to their supplied source; labels match without regard to case. Notes are numbered in first-reference order across the book and placed at the end of the first referring chapter. Later chapters link to that same note. Every reference has its own return link.

Definitions may contain ordinary Markdown paragraphs, lists, code and links to supplied chapters. Put definitions outside other containers. Empty, duplicate and unused definitions, references inside a note, and tables inside a note fail explicitly. Unknown markers such as `[^missing]` without a definition remain literal text under the Markdown parser's rules; inspect source spelling if a note appears as text. Raw HTML and inline images remain unsupported inside notes.

## Link between chapters

Use heading fragments such as `[section](#getting-started)`. Heading identifiers use lowercase letters and numbers, preserve Unicode and underscores, remove punctuation, and replace whitespace with hyphens. Repeated identifiers get numeric suffixes while avoiding collisions. For example, repeated `# Intro` headings become `intro` and `intro-2`.

Link to another supplied file with `[chapter](other.md)` or `[section](other.md#heading)`. Paths are relative to the source containing the link; percent-encoded filenames and fragments are accepted. Only supplied source files can become local destinations. HTTP, HTTPS and mailto links remain external. The tool does not fetch content or check live website availability.

The visible contents page, reader contents navigation, previous/next links and return-to-contents links are generated automatically. The contents page itself precedes the chapters in reading order.

## Check output and handle failures

```sh
agent-epub validate book.epub
```

Every build already checks its temporary output before publishing it. The built-in checker verifies ZIP integrity, basic package metadata, resource references, reading order, contents, internal fragments and generated adjacent-chapter navigation. It checks this tool's profile, rather than arbitrary EPUB conformance, CSS behavior or accessibility.

An existing output is preserved unless `--overwrite` is supplied. Even with overwrite enabled, invalid inputs or a failed requested validator preserve the previous file. An output cannot alias an input source. A temporary publication is renamed into place only after all requested checks pass.

| Exit code | Meaning |
| --- | --- |
| 0 | Requested operation succeeded. |
| 2 | Invalid arguments, metadata or unsupported content. |
| 3 | Input/output operation failed. |
| 4 | Structural publication checks failed. |
| 5 | Requested external validator failed or could not run. |

Diagnostics go to stderr. Success identifies whether EPUBCheck ran. Milestone 2 adds a structured request/result contract.

## Run independent EPUBCheck

Download an official EPUBCheck distribution separately. Set `EPUBCHECK_JAR` to its JAR and optionally `EPUBCHECK_JAVA` to the Java executable. The repository includes an executable wrapper:

```sh
export EPUBCHECK_JAR=/path/to/epubcheck.jar
agent-epub build --input chapters.md --output book.epub \
  --title "My book" --language en --epubcheck ./scripts/epubcheck.sh
agent-epub validate book.epub --epubcheck ./scripts/epubcheck.sh
```

The configured executable receives the EPUB path as one argument. It is launched directly, without interpreting a shell command. Validator failure prevents a build from publishing output. Reports remain visible on stderr. A passing validator does not establish appearance, accessibility conformance or retailer acceptance.

## Developer verification

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
EPUBCHECK=./scripts/epubcheck.sh cargo test --locked \
  --test cli independent_epubcheck_corpus -- --ignored
python3 scripts/benchmark.py target/release/agent-epub
```

The independent corpus test is explicitly ignored in ordinary tests because it requires a separately installed official validator. The CI workflow downloads a checksum-pinned EPUBCheck release and runs that test. The benchmark is a development helper; its Python dependency is separate from the generator.

See the [verification record](verification/MILESTONE_1.md) for completed checks and remaining reader review.
