# Agent EPUB

**v0.5.0** creates reflowable EPUB reading editions from UTF-8 text and Markdown
for agent workflows. It includes simple tables, linked footnotes, nested section
contents and optional JPEG/PNG covers. Version 0.5 adds detailed repair diagnostics,
explicit Markdown rules and configurable chapter splitting. Generation runs offline without an AI
service or Rust installation. MIT licensed.

## Install

Download a binary from the [v0.5.0 release](https://github.com/benvds/epub-rust/releases/tag/v0.5.0):
macOS Apple Silicon, Linux x86_64 or Windows x86_64. The repository is private;
downloads require access. [Installation, checksum verification, runtime boundaries
and updates](docs/INSTALL.md) include commands for each platform.

Every archive includes the executable, license and dependency notices, request
schema, examples, generated sample EPUB and a reusable agent skill. After
extraction, put the executable on PATH and work from the extracted directory.

## Stream a book request

No intermediate manuscript file is needed. In a POSIX shell:

```sh
agent-epub capabilities
agent-epub build --request - --output guide.epub --json <<'JSON'
{"schema_version":1,"metadata":{"title":"A guide","language":"en"},"inputs":[{"name":"guide.md","format":"markdown","content":"# Start\n\nA claim[^note].\n\n## Results\n\n| Item | Value |\n| --- | ---: |\n| Example | 1 |\n\n[^note]: Synthetic example."}]}
JSON
agent-epub validate guide.epub --json
```

For the complete report sample, including repeated notes and nested headings:

```sh
agent-epub build --request - --output report.epub --json < examples/report-inline.json
```

In PowerShell, use UTF-8 input:

```powershell
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)
Get-Content -Raw -Encoding utf8 examples/report-inline.json |
  agent-epub build --request - --output report.epub --json
```

Require exit code 0 and `status: success`. Successful JSON contains the absolute
`output` path, chapter count, check results and remaining review items. A typical
check result is:

```json
{"structural_profile":"passed","epubcheck":"not_run"}
```

This means built-in checks passed and EPUBCheck did not run. Failures return a
nonzero exit code, `status: failed`, an error category/message, detailed `issue_code`, source/location when available
and a `remedy` and `output: null`.
Do not present an older file as the result of a failed build. Logs and independent
validator reports stay on stderr. [Full agent contract](docs/AGENT_INTERFACE.md).

## Use existing files

```sh
agent-epub build --input examples/research-report.md \
  --output report.epub --title "Agent EPUB sample report" --language en --json
agent-epub build --request examples/report-request.json --output requested.epub --json
```

Repeat `--input` in reading order. File-request paths resolve relative to the
request file. Streamed file references require `--base-dir`; purely inline
streams do not. Each source starts a chapter; root H1 headings split Markdown
into further chapters. Preambles stay intact. Root H2–H6 headings enter nested
contents, and previous/next chapter and return-to-contents links are automatic.

For a book title followed by H2 chapters, use `--split-level 2`:

```sh
agent-epub build --input examples/title-h2.md --split-level 2 \
  --output field-guide.epub --title "Field guide" --language en --json
agent-epub build --request examples/title-h2-request.json --output requested-guide.epub --json
agent-epub build --request - --output inline-guide.epub --json < examples/title-h2-inline.json
```

The default remains H1. Sources and preambles are preserved; only root headings
at the selected level start chapters. Request mode uses `"split_level": 2`.

The [synthetic report source](examples/research-report.md) has equivalent
[file](examples/report-request.json) and [inline](examples/report-inline.json)
requests. Its generated `examples/sample-report.epub` is bundled in each release
ZIP. Open it in an EPUB reader to inspect appearance. Return the tool's reported
output file as the artifact in an agent environment; it is not automatically
uploaded anywhere.

Existing output files are preserved unless `--overwrite` is explicitly supplied.
Invalid input or failed requested validation preserves previous output even with
overwrite enabled. Sources cannot be replaced by the output.

## Supported content and checks

Ordinary CommonMark headings, paragraphs, emphasis, lists, quotations, code and
links are supported, plus rectangular pipe tables and source-local footnotes
with return links. Local links must target supplied sources. Inline images,
raw HTML, task lists, complex tables and nested notes are unsupported. Leading YAML/TOML front matter with an initial key/value field is rejected with
a repair hint. Math, strikethrough and alert markers retain literal CommonMark
behavior. No remote resources are fetched. [Exact content rules and limits](docs/USAGE.md).

Every build reopens and checks its temporary EPUB before publication: ZIP
integrity, package metadata, manifest/spine references, chapter order, contents
hierarchy, heading fragments, note returns and adjacent-chapter links. The same
profile checker is available through `validate`. It does not validate arbitrary
EPUBs or certify accessibility, CSS appearance or retailer acceptance.

Optional `--epubcheck <executable>` runs an independently installed EPUBCheck
wrapper before publishing output. Java and EPUBCheck are separate dependencies;
missing or failed requested validators are failures. Reader review and
accessibility review remain separate from both automated checks.

## Agent integration and development

Install the bundled [agent skill](skills/agent-epub/SKILL.md) as described in
[installation](docs/INSTALL.md). It uses the same CLI and examples across agent
runtimes. The milestone 2 live OpenAI evaluation passed 20 tasks; Claude and
Ollama live evaluations remain deferred. Version 0.4 adds packaged workflow
verification, not a new live model score.

Source builds require Rust 1.88 or later:

```sh
cargo build --release --locked
cargo test --locked
```

See the [EPUB feature comparison matrix](docs/research/epub-feature-matrix.md)
for current support across Agent EPUB and twelve alternatives.

See [usage](docs/USAGE.md), [requirements](https://github.com/benvds/epub-rust/blob/v0.5.0/docs/PRD.md),
[roadmap](docs/ROADMAP.md),
[implementation plan](docs/IMPLEMENTATION_PLAN.md),
[milestone 5 verification](docs/verification/MILESTONE_5.md) and
[milestone 3 reader observations](https://github.com/benvds/epub-rust/blob/v0.5.0/docs/verification/MILESTONE_3.md).
