# Milestone 2 agent evaluations

Evaluation date: 9 November 2026. Claude and Ollama evaluations are deferred at the user's request. The completed live evaluation uses the ChatGPT-authenticated OpenAI Codex CLI, rather than the ChatGPT web or desktop chat interface.

For later live agent-workflow evidence, see the separate [milestone 5a comparison](MILESTONE_5A.md). This record remains the historical milestone 2 usability evaluation.

## Completed live evaluation

| Measure | Result |
| --- | --- |
| Agent environment | Codex CLI 0.167.0-alpha.3.2, existing ChatGPT login |
| Effective model | Client default; its identity was not exposed in recorded client events |
| Corpus | 20 synthetic tasks in one isolated native-agent session |
| First complete build request | 20 of 20 passed |
| Completed publications | 20 of 20 passed independent structural and EPUBCheck validation |
| Capability discovery | Called before generation |
| Immutable fixture changes | None |
| Human corrections | None |

The cases cover direct files, JSON request files and streamed requests. They include literal text, Markdown, preambles, duplicate headings, Unicode, mixed sources, inline cross-source links, metadata, overwrite behavior, JPEG and PNG covers, misleading image filenames and paths containing spaces.

An instrumented wrapper records actual CLI invocations and preserves the output from every successful build attempt. The scorer checks the first output snapshot for each case. It checks metadata, chapter count, content markers, requested input mode, structured results, cover bytes and alternative text, then validates the publication independently. A later repair cannot make an incorrect first attempt count as a first-attempt success.

This is a small, fixed usability corpus. The 100% result meets the PRD's 95% threshold for this evaluated environment and corpus; it is not a reliability estimate for all models or clients. The evaluation does not establish behavior in the ChatGPT GUI. [Machine-readable results](milestone-2-openai-evaluation.json).

## Reproduce the evaluation

Build the release binary and synthetic cover generator. Prepare a fresh workspace for each provider so earlier outputs and conversations cannot affect the result.

```sh
cargo build --release --locked --bins --examples
target/release/examples/evaluation_covers /tmp/eval-covers
python3 scripts/evaluate_agents.py prepare \
  --binary target/release/agent-epub --covers /tmp/eval-covers \
  --workspace /tmp/epub-openai-evaluation
python3 scripts/evaluate_agents.py run --provider openai \
  --executable /path/to/codex --workspace /tmp/epub-openai-evaluation
python3 scripts/evaluate_agents.py score \
  --workspace /tmp/epub-openai-evaluation \
  --epubcheck ./scripts/epubcheck.sh --output evaluation.json
```

Scoring needs `EPUBCHECK_JAR` and optionally `EPUBCHECK_JAVA`, as described in [usage](../USAGE.md). Provider execution needs the user's existing login and network access. Only synthetic task data is supplied to the provider. Raw transcripts and EPUB snapshots stay in the evaluation workspace; committed results contain counts, case outcomes and hashes.

The harness can invoke a configured Claude Code client later. No Claude result is claimed. An Ollama-backed evaluation remains deferred and is not represented by deterministic tests or another provider's result.
