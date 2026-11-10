# Practical agent export benchmark protocol

Selected by the maintainer on 10 November 2026: three paired workflows, intended
as practical observations rather than a statistically powered comparison.
Execute only after milestone 5 has passed its release gates.

Compare released Agent EPUB 0.5.0 with Pandoc 3.13 and a documented small wrapper.
Use Codex CLI 0.167.0-alpha.3.2, explicitly requested `gpt-6.1-sol`, high reasoning,
default service tier, identical permissions, host and instructions. Record the
requested model separately from any provider-reported effective model. Fresh
isolated sessions use `--ephemeral --ignore-user-config --json`; no delegation.
The [official non-interactive documentation](https://learn.chatgpt.com/docs/non-interactive-mode)
describes the event stream used for instrumentation. Pandoc flags follow its
[official manual](https://pandoc.org/MANUAL.html).

Each pair runs the same completed manuscripts and export requirements:

1. Cold setup through a correct covered, multi-source report, with Unicode,
   a table, notes, links and chapter navigation.
2. Warm title-plus-H2 guide, with preamble, skipped ranks and repeated headings.
3. Warm inline-content export, measured separately from file-based exports.
4. Warm recovery from a deliberately broken fragment, with an explicit intended
   correction. Only the recovery copy may be edited.
5. Separate unsupported-content task with a supplied local image. A supported
   valid image export and an explicit unsupported response are distinct outcomes;
   neither is counted as a shared-feature speed win.

Cold assets are fetched into fresh directories; download time is recorded and
included, followed by agent-driven extraction/configuration/discovery and first
correct output. Archives are never preinstalled in the timed cold environment.
The same session then measures warm tasks with documentation/capability caching
allowed equally. These are warm segments of isolated workflows, not separate
independent samples. Existing filesystem/network caches are uncontrolled and
must be disclosed. Agent runtimes and the independent validator are preinstalled
for both; their initial installation is outside export scope.

Order: Agent EPUB then Pandoc in pair 1, Pandoc then Agent EPUB in pair 2,
Agent EPUB then Pandoc in pair 3. Execute serially, with 240 seconds per session
and at most three conversion attempts per task for both tools. Keep timed-out
and failed attempts in results. Archive downloads are scripted by the harness before the agent starts, using
the same fixed release recipes; this is recipe-assisted cold setup, not a test
of discovering download URLs. Pandoc integration includes supplied-source link
rewriting and visible note returns; its added code is reported separately.

Three pairs are the entire planned run; do not
expand the sample to improve an outcome. A separate converter/scoring smoke
check may verify the harness before measured sessions and must be identified.

Resource ceiling: stop before account-wide weekly usage reaches 12% (a
conservative margin below the user's 1/7 daily limit and 80% period ceiling),
and after 1,000,000 reported input-plus-output tokens in measured sessions.
Check account usage between sessions. If a cap prevents completion, publish
partial evidence and the reason; do not replace missing runs with synthetic data.

Instrument real JSONL client events and every converter invocation. Snapshot
produced artifacts before retry. Independently score source text/order,
metadata, requested chapter boundaries, table/note/image structures, all local
links and EPUBCheck 5.3.0. Use equivalent requirements and the same checker for
both tools; no generator self-report counts as a fidelity check. Success time
ends when the independent checks pass. Include setup, model/client interaction,
converter runtime and validation time separately. Do not infer model thinking
time by subtracting overlapping work without documenting the calculation.

Record tool calls by purpose, model turns, conversion attempts, retries, manual
interventions, exposed token usage, first-attempt correctness, failures and
timeouts. Integration recipe/wrapper preparation is separate from measured
export sessions; record actual bounded preparation intervals rather than an
invented estimate. Keep exact prompts, corpus, versions, hashes, sanitized
per-run events, scoring reports and synthetic artifact snapshots reproducible.

Report cold and warm medians/ranges and paired observations, with quality
failures next to speed. State faster, slower or inconclusive within this small
corpus; avoid population-level claims. Record costly steps and follow-up tasks.
No Claude/Ollama evaluations, user recruitment or milestone 6 product expansion
are authorized by this benchmark.
