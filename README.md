# Agent EPUB

Turn your agent's finished report into a checked EPUB with one local command.

Chapters, contents, source links and note returns add work to an export
integration. Agent EPUB packages that behavior behind a versioned request/result
contract. Supply the writing and read the JSON result to return a book or
repair an input problem.

Built for repeated report and guide exports from agents that can run local commands.

## A smaller integration to maintain

- **Keep your writing where it is.** Pass existing files in reading order, or
  stream content your application already holds. Inline requests need no
  intermediate manuscript file.
- **Use one request/result contract.** Discover capabilities, submit a version 1
  request and inspect JSON stdout. Success gives you an absolute artifact path;
  failures give you issue codes and remedies.
- **Include the reading features.** Chapter splitting, nested contents,
  source links, footnotes with return links, simple tables and optional covers
  are part of the exporter. [Supported content](docs/USAGE.md).
- **Run locally.** Generation works offline on macOS Apple Silicon, Linux x86_64
  and Windows x86_64. The executable needs no Rust, Python, Java or AI service.
  Optional independent EPUBCheck is installed separately.
  [Runtime and installation details](docs/INSTALL.md).

Our prepared Pandoc baseline needed 89 Python lines and 14 Lua lines for its
export recipe, source links and visible note returns. Agent EPUB supplies those
book behaviors through its native interface.
[Integration preparation and benchmark](https://github.com/benvds/agent-epub/blob/main/docs/verification/MILESTONE_5A.md).

## Install and try a report

Download **[v0.5.1](https://github.com/benvds/agent-epub/releases/tag/v0.5.1)** for
your platform, verify its checksum, extract it and put the executable on PATH.
The repository is private, so downloads require access.
[Installation commands and supported platforms](docs/INSTALL.md).

Every archive includes examples, a sample EPUB, schema, agent skill and license
notices. From the extracted directory with the executable on PATH:

```sh
agent-epub capabilities
agent-epub build --input examples/research-report.md \
  --output report.epub --title "Agent EPUB sample report" --language en --json
```

Open `report.epub` to inspect its tables, repeated notes and nested headings.
The same book is bundled as `examples/sample-report.epub`.

Install the bundled [agent skill](skills/agent-epub/SKILL.md) with these
[instructions](docs/INSTALL.md#agent-skill). Discover capabilities once per tool
version and reuse the build/result steps.

## Stream writing from your application

Send a JSON request through stdin when your application already holds the text.
For example, in a POSIX shell:

```sh
agent-epub build --request - --output guide.epub --json <<'JSON'
{
  "schema_version": 1,
  "metadata": {"title": "A guide", "language": "en"},
  "inputs": [{
    "name": "guide.md",
    "format": "markdown",
    "content": "# Start\n\nA short guide.\n\n# Next\n\n[Back to Start](#start)."
  }]
}
JSON
```

The bundled `examples/report-inline.json` contains a complete report request.

H1 splitting is the default. For H2 chapters, use `--split-level 2` with files
or `"split_level": 2` in JSON. Preambles and heading ranks stay intact.
[H2 request example](examples/title-h2-request.json).

Request-file paths are relative to that file. Streamed file references require
`--base-dir`; inline-only requests do not. [Contract and path rules](docs/AGENT_INTERFACE.md);
[PowerShell examples](docs/INSTALL.md).

## Handle the result

Require exit code 0 and `status: success`, then return the reported `output`
through your application. Every build checks its temporary EPUB before publishing
the file. A typical check result is:

```json
{"structural_profile":"passed","epubcheck":"not_run"}
```

Built-in checks passed; EPUBCheck did not run. Add `--epubcheck <executable>`
for your installed independent validator; missing or failed validators fail the
build. Appearance and accessibility need separate review.
[Check coverage](docs/USAGE.md#check-output-and-handle-failures).

Failures return nonzero, `status: failed`, `output: null`, an `issue_code` and
`remedy`, plus available source locations. Diagnostics stay off JSON stdout.
Existing files need explicit `--overwrite`; failed builds preserve previous
output even with overwrite enabled.
[Result and repair contract](docs/AGENT_INTERFACE.md#repair-a-failed-request).

## What the small benchmark showed

Three paired Codex workflows compared Agent EPUB **v0.5.0** with a prepared
Pandoc **3.13** wrapper using the same requested model/settings and independent
checks. Median times included agent interaction and validation:

| Measured outcome | Agent EPUB | Pandoc wrapper |
| --- | ---: | ---: |
| Cold download, install and first checked EPUB | 17.0 s | 23.9 s |
| Complete four-task shared workflow | 43.9 s | 54.6 s |
| Clean exports correct on the first attempt | 9/9 | 9/9 |

Agent EPUB was faster in each cold pair. Warm results were mixed; tool-call
and retry counts were similar. Improvised exporters were not measured. Current
v0.5.1 adds an invalid-input guard with unchanged valid-book reading resources.
[Protocol, raw events, artifacts and limits](https://github.com/benvds/agent-epub/blob/main/docs/verification/MILESTONE_5A.md).

## Fit and limits

Agent EPUB supports UTF-8 text and a declared Markdown subset for reflowable
reports and guides. Inline illustrations,
raw HTML, task lists, complex tables and nested notes are outside that subset.
Recognized YAML/TOML front matter is rejected with a repair hint. Math, strike
and alert markers retain literal CommonMark behavior.
[Full rules and limits](docs/USAGE.md), [feature comparison](https://github.com/benvds/agent-epub/blob/main/docs/research/epub-feature-matrix.md).

Live agent evidence is recorded for Codex. Claude and Ollama evaluations remain
deferred; the interface is available to runtimes with executable/file access.

## Documentation and development

- [Install and update](docs/INSTALL.md); [request/result contract](docs/AGENT_INTERFACE.md)
  and [JSON schema](schemas/book-request.v1.json).
- [Content rules](docs/USAGE.md), [release verification](https://github.com/benvds/agent-epub/blob/main/docs/verification/MILESTONE_5.md)
  and [practical workflow comparison](https://github.com/benvds/agent-epub/blob/main/docs/verification/MILESTONE_5A.md).
- [Product requirements](https://github.com/benvds/agent-epub/blob/main/docs/PRD.md), [roadmap](https://github.com/benvds/agent-epub/blob/main/docs/ROADMAP.md) and
  [implementation record](https://github.com/benvds/agent-epub/blob/main/docs/IMPLEMENTATION_PLAN.md).

Source builds require Rust 1.88 or later. Release CI uses Rust 1.95.0.

```sh
cargo build --release --locked
cargo test --locked
```

[MIT licensed](LICENSE).
