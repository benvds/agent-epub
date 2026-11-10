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

## Pandoc 3.13 with the small EPUB wrapper

`./tool discover` shows the wrapper's complete options. Use its arguments after
the case name, with no `build` subcommand or `--json` flag:
`./tool report --input inputs/intro.md --input inputs/report.md --cover assets/cover.png --output out/report.epub --title "Practical report" --language en --author "Benchmark Author"`.
Guide uses `--split-level 2`. For inline, stream cases.json's inline.content to
`./tool inline --stdin --output out/inline.epub --title "Inline export" --language en --author "Benchmark Author"`.
The wrapper supplies EPUB3, contents, metadata, resource paths, atomic output
replacement and a small filter for links between supplied Markdown sources.
It adds visible return links to Pandoc EPUB notes, then otherwise uses ordinary
Pandoc Markdown. `--overwrite` allows a retry to
replace an existing artifact. Inspect converter stderr and common-check issues.
Common independent scoring follows every successful conversion automatically.
