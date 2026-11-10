---
name: agent-epub
description: Create reflowable EPUB reading editions from text or Markdown using the agent-epub CLI, including tables, linked notes, nested contents and optional supplied covers and capability-gated local illustrations. Use for EPUB export, not PDF conversion or arbitrary EPUB editing.
---

Use the installed `agent-epub` executable. Run `agent-epub capabilities` to
discover its version, request schema, content rules and limits before building.
If unavailable, use the release's installation instructions; do not silently
substitute another generator.

Choose ordered files, a version 1 request file, or inline streamed JSON. See
[examples](references/examples.md) for all three. Keep publication metadata in
the request when using request mode. Output, overwrite and external-validator
choices remain CLI options. File-request paths resolve from the request's
directory; streamed file references require `--base-dir`. Inline names are
simple logical filenames for links, without directories.

Preserve the requested content. Each file starts a chapter; root H1 headings
split Markdown chapters and root H2–H6 headings enter nested contents. Tables
need equal cell counts and a header row. Notes are source-local and need used,
nonempty, unique definitions. Raw HTML, task lists and complex tables are unsupported. Use supplied PNG/JPEG covers when requested. Do not
silently remove unsupported content or fetch remote resources to make a build
pass; report the issue and make only repairs within the user's request.

Version 0.6.0 declares `inline_images: true`; earlier releases support covers
only. When supported, use static local JPEG/PNG with meaningful alt text:
`![Description](assets/image.png)`. For purely decorative images, use
`![](assets/divider.png "decorative")`. File assets resolve from the source
parent; inline assets use the request file's directory or explicit base.
Streamed inline images require `--base-dir`. URLs, SVG and animation are
unsupported. Do not invent images or descriptions for a supplied manuscript.
Respect discovery limits and use the illustrated examples when available.

Build with `--json` and inspect both the exit status and result. A usable new
artifact requires exit 0, `status: success`, a nonnull `output`, and
`checks.structural_profile: passed`. Return the reported artifact path as a
downloadable link in environments that support it. Use the output path returned
by the tool instead of inferring it from a working directory.

On failure, report `errors` and the failed operation; `output: null` does not
identify a new artifact. Existing outputs may still exist from earlier runs.
Do not retry with `--overwrite` without authorization to replace that artifact.
For independent validation, pass `--epubcheck` only with an available executable
wrapper. A requested validator that fails or cannot run is a failure, not a
pass. Report `not_run` or `not_claimed` checks as such. Structural checking alone
does not establish full conformance, accessibility or reading appearance.

The same CLI contract can be used by different agent runtimes with shell access.
Claude and Ollama live evaluations remain deferred; do not claim tested provider
support from these instructions.

For title-plus-H2 manuscripts, select `--split-level 2` with direct files or
`"split_level": 2` in a version 1 request. Default H1 splitting is unchanged.
Use the bundled `examples/title-h2-*` requests. Preserve source files; do not
rewrite all headings just to choose chapter boundaries.

On failure, retain the broad `errors[].code` category and inspect `issue_code`,
`source`, available one-based `line`/`column`, and `remedy` before retrying.
Recognized leading YAML/TOML front matter is rejected; move metadata to explicit
fields. Math, strike and alert markers remain literal CommonMark text.
