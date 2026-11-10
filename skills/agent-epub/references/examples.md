# CLI examples

From the extracted release directory, with its executable on PATH:

```sh
agent-epub capabilities
agent-epub build --input examples/research-report.md \
  --title "Agent EPUB sample report" --language en --output direct.epub --json
agent-epub build --request examples/report-request.json --output request.epub --json
agent-epub build --request - --output inline.epub --json < examples/report-inline.json
agent-epub validate inline.epub --json
```

In PowerShell, stream the UTF-8 request with:

```powershell
$OutputEncoding = [System.Text.UTF8Encoding]::new($false)
Get-Content -Raw -Encoding utf8 examples/report-inline.json |
  agent-epub build --request - --output inline.epub --json
```

The release includes those synthetic source/request files. For a self-contained
inline request, the contract is:

```sh
agent-epub build --request - --output guide.epub --json <<'JSON'
{"schema_version":1,"metadata":{"title":"A guide","language":"en"},"inputs":[{"name":"guide.md","format":"markdown","content":"# Start\n\nA claim[^note].\n\n## Results\n\n| Item | Value |\n| --- | ---: |\n| Example | 1 |\n\n[^note]: Synthetic example."}]}
JSON
```

A successful result reports an absolute `output` path and:

```json
{"status":"success","checks":{"structural_profile":"passed","epubcheck":"not_run"}}
```

This excerpt omits other fields. Say that structural checks passed and EPUBCheck
was not run; reader appearance and accessibility require separate review.

For example, unsupported inline images fail with exit 2 and an `invalid_input`
error. A missing requested EPUBCheck executable fails with exit 5 and
`validator_failed`. In both cases `status` is `failed`, `output` is null and
checks are `not_claimed`. Report that failure rather than presenting a prior
file as newly generated. See the packaged `docs/AGENT_INTERFACE.md` for full
request and result rules.
