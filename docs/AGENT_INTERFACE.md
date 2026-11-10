# Agent interface

Version 0.5.0 extends the version 1 CLI contract with optional chapter splitting and additive diagnostic fields. [Install a binary and the reusable agent skill](INSTALL.md). The release includes equivalent direct, [file-request](../examples/report-request.json) and [streamed-inline](../examples/report-inline.json) report examples.

Milestone 2 supports generic book requests, streamed input, capability discovery, structured results and supplied cover images. The live OpenAI-agent evaluation passed all 20 corpus tasks. Claude and Ollama evaluations are deferred at the user's request. [Evaluation details](verification/AGENT_EVALUATIONS.md).

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

Title, language, author and split-level flags are mutually exclusive with request mode. Output location, overwrite and optional external-validation choices remain explicit command options. Request files and supplied sources are protected against output aliasing.

## Inspect the result

`--json` returns one JSON object on stdout. It includes the operation, status, absolute artifact location on success, chapter count, check coverage, remaining review items and elapsed milliseconds. It also produces structured argument errors when the CLI cannot parse a command.

Failure results contain an error code and message, and do not identify a newly created artifact. Exit codes retain their [milestone 1 meanings](USAGE.md). Available error categories are `invalid_input`, `io_error`, `invalid_epub` and `validator_failed`.

Independent-validator reports may appear on stderr; they do not contaminate structured stdout. `--help` and `--version` keep their standard command-line behavior. A success result with `epubcheck: not_run` describes built-in checking only.

## Repair a failed request

Result schema version 1 and `errors[].code` retain the broad categories above.
New additive fields are `issue_code`, `source`, `line`, `column` and `remedy`.
Callers should ignore unknown fields and fall back to the broad category for an
unrecognized detailed code. `source` is the supplied file path or the logical
inline filename resolved against the request base. Locations are one-based;
columns count Unicode characters. Source locations use normalized line endings
and exclude a leading BOM. Unavailable sources or locations are JSON `null`.
JSON syntax/type errors refer to the request document itself; streamed JSON has
no file source. Parser locations identify the start of the affected structure,
including a link's opening bracket rather than its destination text.

```json
{"code":"invalid_input","issue_code":"unresolved_link","message":"...","source":"/work/guide.md","line":3,"column":1,"remedy":"Supply the linked source, correct the heading fragment, or use an http(s)/mailto URL."}
```

Detailed codes include `unresolved_link`, `unsupported_link`,
`unsupported_inline_image`, `unsupported_raw_html`, `unsupported_task_list`,
`unsupported_table`, `invalid_table_shape`, `unsupported_footnote`,
`invalid_footnote`, `unsupported_front_matter`, `invalid_split_level` and
`invalid_request`. Other errors retain the category name as their detailed code.
Footnote semantic errors identify the source but may have no line/column.
Fix the reported issue and retry; failures remain fail-fast, with no aggregate
preflight or output publication. Human diagnostics include the code and remedy
on stderr. Structured stdout remains one JSON result.

## Choose chapter boundaries

Optional book-level `"split_level": 2` selects root H2 headings. The integer
range is 1–6; omission preserves H1 splitting. Direct inputs use
`--split-level 2`; mixing that flag with `--request` is an argument error.
All input sources share the selected policy, and text inputs remain one chapter.
Only headings of exactly the selected level split: shallower and deeper
headings retain their original ranks and text. Every source begins independently,
and text before the first matching heading remains a preamble chapter.
A leading H1 or matching heading supplies the chapter title; otherwise the
filename supplies it. A heading used as the chapter title is excluded from
nested contents. Root H2–H6 section headings otherwise keep the existing nesting
rules. Headings inside quotations, lists and notes never split chapters.

The [H2 source](../examples/title-h2.md),
[file request](../examples/title-h2-request.json) and
[inline request](../examples/title-h2-inline.json) show a title/preamble, skipped
levels, repeated chapter titles and cross-chapter links and note returns.

## Supply a cover

Add `"cover": {"path": "cover.png", "alt": "Cover of A guide"}` to a book request. Relative paths follow the same base-directory rules as input files. A streamed request containing a cover needs `--base-dir`. Direct-file builds use `--cover cover.png` and optionally `--cover-alt "Cover of A guide"`; these flags cannot be mixed with request mode.

JPEG and PNG are supported. The tool checks the actual image data, completely decodes it under bounded limits, and preserves the original encoded bytes. Limits are 10 MiB, 8,000 pixels per side and 16 million pixels total. The package uses the detected format even when the supplied filename has a different extension. Empty, corrupt, oversized or unsupported images fail without replacing a previous output.

The cover is identified in publication metadata and appears before the contents page. It is excluded from chapter counts and the chapter table of contents. Chapter previous/next links continue to connect chapters. Supplied alternative text must be nonempty; when omitted, the tool uses `Cover of <book title>`. A successful JSON result reports `cover: true` when a cover is present.

## Use from an agent runtime

Version 0.3 keeps request schema version 1. Markdown file references and inline `format: "markdown"` content both support rectangular pipe tables, source-local footnotes and nested section contents. No new request fields are needed. `capabilities` lists `markdown_extensions`, detailed `markdown_profile` rules and `section_contents_h2_h6` navigation. Read those rules before constructing tables or notes; malformed rectangular tables and duplicate, empty or unused note definitions fail with exit code 2 and JSON `invalid_input`.

The [report example](../examples/research-report.md) demonstrates all three additions. See the [content subset](USAGE.md) for escaping, unmatched-note fallback and nested-heading rules. The direct and streamed-inline request workflows are tested to produce identical report reading resources. This is CLI contract verification; the milestone 2 live OpenAI score is historical and has not been relabeled as a new model evaluation.

Give the agent permission to invoke the executable and access only the sources and output directory it needs. A useful instruction is: discover capabilities, construct a versioned request, build with `--json`, inspect the result and report any failed or unrun checks. Streamed requests avoid creating intermediate manuscript files.

ChatGPT-authenticated Codex CLI was tested through actual tool calls. Claude Code can use the same shell commands when configured with tool access; its live evaluation is deferred. An Ollama-backed runtime can expose a tool that forwards request JSON to the CLI and returns structured stdout; the selected model must support that runtime's tool use. No provider-specific conversation parsing is required, and no live Ollama result is claimed.

The request schema is available in [book-request.v1.json](../schemas/book-request.v1.json). See the [release verification](verification/MILESTONE_2.md) for the tested scope and remaining limitations.
