# Agent EPUB

A focused EPUB tool for AI agents. Creates reading editions from plain text and Markdown, with chapter and section contents, linked notes, simple tables and an optional JPEG/PNG cover. Accepts files or versioned JSON requests without calling an AI service.

## Project documents

- [Product requirements](docs/PRD.md)
- [Three-milestone roadmap](docs/ROADMAP.md)
- [Rust CLI implementation plan](docs/IMPLEMENTATION_PLAN.md)
- [ChatGPT EPUB capability research](docs/research/chatgpt-epub-capabilities.md)
- [EPUB tool comparison](docs/research/epub-tools-analysis.md)

## Build and use

Requires Rust 1.88 or later.

```sh
cargo build --release --locked
target/release/agent-epub build \
  --input examples/introduction.txt --input examples/guide.md \
  --output example.epub --title "Example book" --language en
target/release/agent-epub validate example.epub
```

Each input starts a chapter. Top-level Markdown headings start additional chapters; preamble text stays in its own chapter. Contents and previous/next chapter links are included automatically. Use `--overwrite` to explicitly replace an output.

Every build runs structural profile checks. Optional `--epubcheck <executable>` runs an independently installed EPUBCheck wrapper before publishing output. The built-in validator covers this tool's generated profile and does not certify arbitrary EPUBs, accessibility or reading appearance.

Inline images, complex tables, nested notes, task lists and raw HTML remain unsupported. Full EPUB support is outside the roadmap.

Milestone 2 is complete for the agreed scope: [general agent requests, streamed input, structured results and JPEG/PNG covers](docs/AGENT_INTERFACE.md). The live OpenAI agent passed all 20 evaluation tasks. Claude and Ollama evaluations are deferred at the user's request.

See `agent-epub --help` and `agent-epub build --help` for options.

[Detailed usage and supported subset](docs/USAGE.md). Try [the report example](examples/research-report.md) for tables, repeated notes and nested contents.

[Milestone 2 verification, reader checks and benchmark](docs/verification/MILESTONE_2.md).

Milestone 3 adds semantic tables, footnotes with return links and nested H2–H6 contents. [Verification and release boundary](docs/verification/MILESTONE_3.md).
