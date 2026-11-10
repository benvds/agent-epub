# Milestone 5a: practical agent workflow comparison

10 November 2026. Three paired real-agent workflows compared released Agent
EPUB 0.5.0 with Pandoc 3.13 and a small prepared EPUB wrapper. All six sessions
completed within their caps. **Agent EPUB was faster in each observed cold
setup and complete shared workflow. Warm export results were mixed, and neither
tool reduced clean-export retries or tool calls in this sample.** This is a
practical local result, not a statistically powered general performance claim.

The current release is v0.5.1, which adds an invalid-input guard for links to
sources without chapters. Valid chapter/navigation resources are unchanged.
The benchmark data below remain the original v0.5.0 measurements.

## Time to an independently correct EPUB

Values are median seconds, with observed minimum–maximum in parentheses.
There are three observations per tool per row. Cold time includes a fresh
archive download, agent/client startup, extraction, discovery, conversion and
independent validation. Warm rows measure subsequent tasks in the same isolated
session, including the agent's intervening tool use and preparation.

| Scenario | Agent EPUB | Pandoc wrapper | Practical reading |
| --- | ---: | ---: | --- |
| Cold covered multi-file report | 17.0 (16.7–22.2) | 23.9 (21.8–26.7) | Agent EPUB faster in all three pairs. |
| Warm title-plus-H2 guide | 5.0 (5.0–5.9) | 5.6 (5.1–7.0) | Inconclusive: paired direction varied; median paired difference was only 0.08 s. |
| Warm streamed inline content | 6.7 (6.5–6.7) | 6.4 (6.3–7.1) | Inconclusive: paired direction varied within 0.5 s. |
| Warm broken-link attempt and repair | 14.6 (14.5–14.6) | 16.4 (14.8–17.4) | Agent EPUB faster in all three; both required one retry. |
| Complete shared workflow, through recovery | 43.9 (43.2–48.2) | 54.6 (49.3–54.6) | Agent EPUB faster in all three pairs. |

The complete shared workflow was 5.5–11.4 seconds faster with Agent EPUB; the
median paired saving was 6.3 seconds. The difference between two unpaired
medians is not the median paired difference. Exact per-run values and paired
differences are in [the reproducible summary](milestone-5a-summary.json).
Final agent narration and the separate image-support task are excluded from
this shared success clock; full session times remain in the raw results.

## Quality and actual agent work

Both tools completed **12/12 shared exports**, including recovery, across three
sessions each. Each completed the nine clean exports on the first attempt.
Every deliberately broken-link task failed once and then passed after its
allowed fragment correction: three expected failed attempts and three retries
per tool. No final shared publication failed, no session timed out, no protected
source changed, and no manual intervention was needed. Failed attempts remain
in the measurements and logs.

Each agent workflow used eight instrumented local runtime calls: installation,
discovery, three clean builds, two recovery attempts and one image probe.
Agent EPUB used 10–11 actual shell tool calls per session, versus 10 for Pandoc;
totals were 31 and 30. Each workflow also had one scripted archive-download
operation before the agent started. Fewer exporter flags did not demonstrate a
reduction in agent tool calls here. CLI client turns were recorded, but exact
individual model-response turns were not exposed; they are reported as
unavailable rather than inferred from shell calls.

Independent checks applied the same requested metadata, source passage/order,
chapter boundary and contents expectations to both outputs, along with table
cells, note references/returns, image bytes where requested, every local resource
and fragment, and official EPUBCheck 5.3.0. The retained snapshots include all
three invalid Pandoc recovery outputs; Agent EPUB rejected those source links
before generating a publication. That early rejection avoided an external
validation pass during each Agent EPUB repair. Success was never counted from
a generator's own exit code alone.

A separate offline [source-line audit](milestone-5a-source-audit.json), performed
after timing, found every expected visible source fragment in all 27 final
successful publications: 24 shared outputs plus three Pandoc image books. This
audit did not change the timed checker, prompts, retry rules or outcomes. Its
small syntax recognizer is specific to the synthetic corpus. Neither markers,
counts nor this audit establish semantic fidelity for arbitrary manuscripts.

The separate local-image task succeeded three times with Pandoc and received
three explicit `unsupported_inline_image` responses from Agent EPUB. Pandoc's
image export took 6.4 seconds median (5.2–6.5). Unsupported rejection is not a
faster equivalent export and is excluded from the shared speed comparison.
Pandoc currently offers the broader feature boundary for that task.

## Where the time went

Median component seconds for the complete shared workflow:

| Component | Agent EPUB | Pandoc wrapper |
| --- | ---: | ---: |
| Download | 1.14 | 5.23 |
| Local archive extraction/configuration | 0.18 | 1.74 |
| Converter discovery execution | 0.005 | 0.068 |
| All shared converter attempts | 0.067 | 0.487 |
| Independent validation | 9.05 | 11.16 |
| Model/client/shell and other elapsed time | 33.20 | 35.85 |

Component medians need not sum to the median total. The final row is a residual
of non-overlapping runtime intervals, not a direct measurement of model thinking.
It includes client startup, shell commands outside the instrumented wrapper,
reading, request construction and interaction latency. Converter CPU time is a
small fraction of the workflow. The 1,415,764-byte Agent EPUB release archive,
versus Pandoc's 43,148,194-byte archive, explains part of the cold result.

Reported usage totals, including all clean, recovery and image work:

| Exposed usage | Agent EPUB | Pandoc wrapper |
| --- | ---: | ---: |
| Input tokens | 823,321 | 846,567 |
| Cached input tokens, included above | 749,952 | 799,616 |
| Uncached input tokens | 73,369 | 46,951 |
| Output tokens | 2,494 | 2,223 |
| Separately reported reasoning output tokens | 53 | 78 |

These totals do not establish a token-cost advantage. Context caching and
client-supplied instructions influence them; account usage is shared with other
work. The account limit tool read 3% weekly usage before measurement and 4%
afterwards, below both user ceilings. Raw input plus output totaled 1,674,605,
within the disclosed 1,800,000-token ceiling. The protocol records the
resource-only amendment made after the first session and before the second;
no outcome, sample, order, time cap or checker was changed to improve results.

## Integration preparation and reproducibility

Agent EPUB reused its released CLI contract and agent instructions. The Pandoc
baseline received a prepared wrapper with explicit metadata, EPUB3/contents,
chapter level, resource paths and output replacement, plus a supplied-source
link filter and a note-return patch. Pandoc's ordinary EPUB output in the smoke
check had note IDs/references but no visible return links; the patch added them
before measured sessions. It preserves original note text and image bytes.
This preparation prevented a broken baseline from becoming a speed comparison.

The additional Pandoc integration is 89 Python lines plus 14 Lua lines. This is
a bounded recipe for this corpus, not a replacement publishing engine. The
recorded preparation interval was 512 seconds (8.5 minutes), covering both
shared-harness work and the Pandoc wrapper, with interleaved CI work. Active
per-tool effort was not separately instrumented; the interval is not presented
as Pandoc-only labor or compared to an invented Agent EPUB preparation time.
Existing Agent EPUB integration work is historical effort. See the explicit
[preparation record](../../benchmarks/workflow/PREPARATION.json).

The [protocol](../../benchmarks/workflow/PROTOCOL.md),
[exact prompt](../../benchmarks/workflow/PROMPT.txt),
[recipes and harness](../../benchmarks/workflow/README.md),
[committed corpus](../../benchmarks/workflow/corpus/cases.json),
[sanitized raw sessions](workflow-runs/) and
[hash/environment manifest](workflow-runs/MANIFEST.json) reproduce the comparison.
All 30 produced snapshots are retained, including the three invalid first
recovery outputs. Original workspace-relative `.measurement/artifacts/` paths
in logs map to the exported session's `artifacts/` directory.

Runs used Codex CLI 0.167.0-alpha.3.2 with explicitly requested `gpt-6.1-sol`,
high reasoning and default service tier, Python 3.15.2, macOS 27.2 arm64 and
OpenJDK 27.0.1.0.3 with EPUBCheck 5.3.0. The requested model/settings were fixed;
the CLI did not expose provider-confirmed effective model identity. Fresh
`--ephemeral --ignore-user-config` sessions ran serially, with tool order
alternating between pairs, identical permissions and completed manuscripts.
Exact commands, event times, native results, usage and artifact hashes are
retained. The client emitted rollout-state warnings in both tools' runs; its
stderr is retained and that overhead remains in elapsed time.

## Limits and follow-up tasks

This is recipe-assisted cold setup: the harness downloads pinned official
archives before launching the agent, and the agent performs installation and
discovery. It does not test finding release URLs or unfamiliar documentation.
The client and validator were preinstalled equally. Warm cases follow cold
installation in the same session, with caching allowed for both tools; they are
not independent cold samples. Network/CDN and filesystem caches were not
controlled. Manuscripts are short synthetic examples. Results do not establish
performance for large books, private manuscripts, other models, hosts or
unprepared integrations. No statistically significant population claim is made.

Both tools were scored on the same requested reader contents and source links.
Extra title pages, per-reference repetition of note text and note-numbering
differences were allowed. Pandoc repeats a note body for each reference, while
Agent EPUB shares one source-local note definition. The shared scorer checks
content and working reference/return links; it does not require identical
note sharing or numbering.
Agent EPUB's visible previous/next chapter links were not a shared requirement,
and universal reader behavior and accessibility were not benchmarked.

The next work should follow the measured costs:

1. Inspect client/context and shell overhead before spending effort on converter
   micro-optimization; it dominated total time for both tools.
2. Test cached, shorter integration instructions if reduced tool calls are the
   desired advantage. This sample showed equal clean-export reliability and
   nearly identical shell-call counts, so no retry reduction is claimed.
3. Keep the Pandoc wrapper/filter smoke fixtures pinned when comparing future
   releases; supplied-source links and visible note returns required integration
   work beyond a bare converter invocation here.
4. Bring the inline-image gap to milestone 6's scope decision. The synthetic guide
   is a concrete comparison case, not automatic authorization to implement D1.

Milestone 5a's requested practical comparison is complete. Milestone 6 remains
an evidence review and product decision; Claude/Ollama live evaluations remain
deferred, and no conditional feature expansion has started.
