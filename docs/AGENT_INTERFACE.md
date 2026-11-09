# Agent interface

Milestone 2 is in progress. The first checkpoint supports generic book requests, streamed input, capability discovery and structured results. Cover support and live cross-agent evaluations follow separately.

## Discover and build

```sh
agent-epub capabilities
agent-epub build --request examples/book-request.json --output book.epub --json
agent-epub build --request - --output book.epub --json < request.json
```

`capabilities` returns JSON, including the complete supported request schema, feature boundary and limits. It requires no provider account or AI model. An agent environment needs permission to invoke the executable and access the supplied files.

Requests use `schema_version: 1`, publication `metadata` and an ordered `inputs` collection. Each input supplies either a file `path`, or inline `content` and a `format` of `text` or `markdown`. Inline inputs can specify a simple filename in `name`; otherwise the tool generates `input-N.txt` or `input-N.md`.

```json
{
  "schema_version": 1,
  "metadata": {"title": "A guide", "language": "en"},
  "inputs": [
    {"name": "intro.txt", "format": "text", "content": "An introduction."},
    {"name": "guide.md", "format": "markdown", "content": "# First chapter\n\nMain text.\n\n# Second chapter\n\nMore text."}
  ]
}
```

Unknown fields, unsupported schema versions, conflicting source fields and ambiguous names produce errors. Input formats follow the same chapter rules as direct file input. A single Markdown input can contain several chapters; no private provider conversation format is accepted or required.

## Resolve file references

File-request paths are relative to the request file's directory. `--base-dir` can explicitly override that base. Streamed requests containing file references require `--base-dir`. Pure inline streamed requests need no base; supplying one makes relative inline-to-file links explicit.

Inline input names act as logical source filenames for links. Two inline inputs named `one.md` and `two.md` can link to each other as ordinary Markdown sources. Names cannot contain directories or parent traversal. Input source files retain their actual source-relative link behavior. The tool does not create inline sources on disk or fetch remote resources.

Title, language and author flags are mutually exclusive with request mode. Output location, overwrite and optional external-validation choices remain explicit command options. Request files and supplied sources are protected against output aliasing.

## Inspect the result

`--json` returns one JSON object on stdout. It includes the operation, status, absolute artifact location on success, chapter count, check coverage, remaining review items and elapsed milliseconds. It also produces structured argument errors when the CLI cannot parse a command.

Failure results contain an error code and message, and do not identify a newly created artifact. Exit codes retain their [milestone 1 meanings](USAGE.md). Available error categories are `invalid_input`, `io_error`, `invalid_epub` and `validator_failed`.

Independent-validator reports may appear on stderr; they do not contaminate structured stdout. `--help` and `--version` keep their standard command-line behavior. A success result with `epubcheck: not_run` describes built-in checking only.

The request schema is available in [book-request.v1.json](../schemas/book-request.v1.json). The [roadmap](ROADMAP.md) tracks cover support and remaining agent evaluations. No live ChatGPT, Claude or Ollama evaluation has been run yet.
