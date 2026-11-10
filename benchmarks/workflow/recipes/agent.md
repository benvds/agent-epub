# Export integration recipe

Run `./tool install` to verify/extract the downloaded official archive into this
workspace, then `./tool discover` to inspect the installed converter interface.
`./tool CASE ...` forwards the remaining arguments to the selected converter,
logs the real invocation, snapshots the result and runs independent checks.
A successful converter exit can still fail independent checking. Inspect both.
Every output is `out/CASE.epub`. `./tool check CASE` optionally rechecks it.
All builds are offline. Run cases in this order: report, guide, inline, recovery,
image. Use metadata title from cases.json, language en, author Benchmark Author.
The report has ordered inputs and cover from cases.json; guide uses split level
2. Inline uses content already in cases.json; stream it without creating a
manuscript file. Recovery must be attempted once unchanged before the allowed
fragment correction. Image support differs: preserve the supplied image and alt
or report the explicit unsupported result. Do not drop it to make a build pass.

## Agent EPUB 0.5.0

Use native `build` arguments after the case name, always with `--json`:
`./tool report build --input inputs/intro.md --input inputs/report.md --cover assets/cover.png --output out/report.epub --title "Practical report" --language en --author "Benchmark Author" --json`.
Guide uses `--split-level 2`. For inline, stream a schema_version 1 request with
metadata and one markdown input (`name`, `format`, `content`) to
`./tool inline build --request - --output out/inline.epub --json`.
Read content from cases.json with Python; no need to repeat it by hand.
An absent optional cover must be omitted. Direct metadata flags conflict with
request mode. Existing outputs need explicit `--overwrite` when retrying.
Detailed `issue_code`, source/location and `remedy` fields help repair failures.
The native generated-profile check is followed by independent common scoring.
