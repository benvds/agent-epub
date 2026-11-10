# Practical EPUB agent workflow benchmark

This directory contains the committed protocol, exact common prompt, two
integration recipes, synthetic source corpus, Pandoc wrapper and reproducible
measurement/scoring scripts. The [results](../../docs/verification/MILESTONE_5A.md)
use three paired real Codex sessions per tool. This is a practical local
comparison, not a statistically powered population claim.

The harness currently selects official macOS arm64 archives. It downloads each
archive into a fresh session directory, checks its SHA-256, then starts Codex.
The agent performs local extraction, discovery and exports through `./tool`.
The shared wrapper captures native converter results and artifact snapshots,
then applies generator-independent checks and official EPUBCheck. All source
content is synthetic. Existing account authentication is used without copying
credentials or retaining them in the results.

Required: Python 3.11+, GitHub CLI authenticated for the private Agent EPUB
release, curl, the recorded Codex CLI and an official EPUBCheck executable
wrapper. Pandoc and Agent EPUB are downloaded into each temporary workspace;
neither is installed system-wide. The agent's shell helpers use the recorded
host's `/opt/homebrew/bin/python3`.

Run converter/scorer smoke checks with downloaded, checksum-verified archives:

```sh
python3 benchmarks/workflow/benchmark.py smoke \
  --work /tmp/epub-workflow-smoke-new \
  --agent-archive /path/to/agent-epub-0.5.0-aarch64-apple-darwin.zip \
  --pandoc-archive /path/to/pandoc-3.13-arm64-macOS.zip \
  --validator /path/to/epubcheck-wrapper
```

Each measured session uses a new directory and refuses to replace an earlier
one. Run serially in the protocol's order, checking account limits between
sessions. Example first session:

```sh
python3 benchmarks/workflow/benchmark.py run \
  --work /tmp/epub-workflow-new \
  --out docs/verification/workflow-runs-new \
  --pair 1 --tool agent \
  --validator /path/to/epubcheck-wrapper --codex /path/to/codex
```

Continue with `(1, pandoc)`, `(2, pandoc)`, `(2, agent)`, `(3, agent)`,
`(3, pandoc)`. The process has a 240-second session limit and three converter
attempts per task. The operator must enforce the protocol's total account and
usage limits; the harness records usage but cannot read the desktop's account
limit tool itself. Do not rerun or discard a slow or failed measured session.

Reproduce the published summaries without any model calls or downloads:

```sh
python3 benchmarks/workflow/summarize.py docs/verification/workflow-runs \
  --output /tmp/workflow-summary.json
python3 benchmarks/workflow/audit_sources.py docs/verification/workflow-runs \
  --output /tmp/workflow-source-audit.json
```

Raw events, native results, per-attempt independent scores, failed recovery
snapshots and final EPUBs are retained beside the result JSON for each session.
User/workspace paths are replaced with placeholders in exported text logs.
`audit_sources.py` is an additional post-measurement audit of all visible source
lines for this small corpus; it does not alter the timed checker or retry rules.
It is not a general Markdown parser or a guarantee of preserved meaning.
