#!/usr/bin/env python3
"""Prepare, run and summarize three paired real-agent EPUB workflows."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import selectors
import shutil
import signal
import struct
import subprocess
import sys
import time
import zlib

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
ORDER = [(1, 'agent'), (1, 'pandoc'), (2, 'pandoc'), (2, 'agent'), (3, 'agent'), (3, 'pandoc')]
PANDOC_SHA = 'f436d18bebbfb3ddd326597686dacab9fac5dc23bd060495020f35a28a43a7c3'
AGENT_ARCHIVE = 'agent-epub-0.5.0-aarch64-apple-darwin.zip'
PANDOC_URL = 'https://github.com/jgm/pandoc/releases/download/3.13/pandoc-3.13-arm64-macOS.zip'
MODEL = 'gpt-6.1-sol'
CASES = ['report', 'guide', 'inline', 'recovery', 'image']


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def png():
    def chunk(kind, data):
        return struct.pack('>I', len(data)) + kind + data + struct.pack('>I', zlib.crc32(kind + data) & 0xffffffff)
    pixels = (b'\0' + bytes([47, 92, 122]) * 128) * 192
    return b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>2I5B', 128, 192, 8, 2, 0, 0, 0)) + chunk(b'IDAT', zlib.compress(pixels)) + chunk(b'IEND', b'')


def corpus(workspace):
    for name in ['inputs', 'assets', 'work', 'out', 'requests', 'download', '.measurement']:
        (workspace / name).mkdir(parents=True, exist_ok=True)
    (workspace / 'assets/cover.png').write_bytes(png())
    sources = {
        'inputs/intro.md': '# Introduction\n\nINTRO-MARKER: The complete introduction stays first.\n\n[Findings](report.md#findings).\n',
        'inputs/report.md': '# Findings\n\nFINDINGS-MARKER: Café, 日本語 and all table values stay intact.\n\n| Item | Count |\n| --- | ---: |\n| Café & tea | 7 |\n| 日本語 | 11 |\n\nA claim[^n], repeated[^n].\n\n# Summary\n\nSUMMARY-MARKER: The conclusion stays last.\n\n[Introduction](intro.md#introduction). Another reference[^n].\n\n[^n]: Evidence stays complete.\n',
        'inputs/guide.md': '# Field guide\n\nGUIDE-PREAMBLE: Keep the introduction. [Results](#results).\n\n## Methods\n\nGUIDE-METHODS: Keep the method.\n\n#### Detail\n\nGUIDE-DETAIL: A skipped rank stays nested.\n\n## Results\n\nGUIDE-RESULTS: Keep the first result. [Methods](#methods).\n\n## Results\n\nGUIDE-REPEAT: Keep the repeated result.\n',
        'work/recovery.md': '# Recovery\n\nRECOVERY-BEGIN: Keep this text. [Results](#missing).\n\n# Results\n\nRECOVERY-END: This is the intended destination.\n',
        'inputs/image.md': '# Image booklet\n\nIMAGE-MARKER: Keep the supplied illustration or report unsupported.\n\n![Supplied diagram](../assets/cover.png)\n',
    }
    for name, text in sources.items():
        (workspace / name).write_text(text, encoding='utf-8')
    image_sha = digest(workspace / 'assets/cover.png')
    cases = {
        'report': {'title': 'Practical report', 'inputs': ['inputs/intro.md', 'inputs/report.md'],
                   'cover': 'assets/cover.png', 'image_sha256': image_sha, 'table': True, 'notes': 3,
                   'contents': ['Introduction', 'Findings', 'Summary'],
                   'markers': ['INTRO-MARKER: The complete introduction stays first.', 'FINDINGS-MARKER: Café, 日本語 and all table values stay intact.', 'SUMMARY-MARKER: The conclusion stays last.'],
                   'chapter_markers': ['INTRO-MARKER:', 'FINDINGS-MARKER:', 'SUMMARY-MARKER:']},
        'guide': {'title': 'Field guide', 'inputs': ['inputs/guide.md'], 'split_level': 2,
                  'contents': ['Field guide', 'Methods', 'Detail', 'Results', 'Results'],
                  'markers': ['GUIDE-PREAMBLE: Keep the introduction.', 'GUIDE-METHODS: Keep the method.', 'GUIDE-DETAIL: A skipped rank stays nested.', 'GUIDE-RESULTS: Keep the first result.', 'GUIDE-REPEAT: Keep the repeated result.'],
                  'chapter_markers': ['GUIDE-PREAMBLE:', 'GUIDE-METHODS:', 'GUIDE-RESULTS:', 'GUIDE-REPEAT:']},
        'inline': {'title': 'Inline export', 'content': '# Inline\n\nINLINE-FIRST: Keep the text already held in memory.\n\n# Next\n\nINLINE-NEXT: Keep the second chapter. [Back](#inline).\n',
                   'contents': ['Inline', 'Next'], 'markers': ['INLINE-FIRST: Keep the text already held in memory.', 'INLINE-NEXT: Keep the second chapter.'],
                   'chapter_markers': ['INLINE-FIRST:', 'INLINE-NEXT:']},
        'recovery': {'title': 'Recovery export', 'inputs': ['work/recovery.md'],
                     'repair': 'Attempt once unchanged, then change only (#missing) to (#results) in work/recovery.md and retry.',
                     'contents': ['Recovery', 'Results'], 'markers': ['RECOVERY-BEGIN: Keep this text.', 'RECOVERY-END: This is the intended destination.'],
                     'chapter_markers': ['RECOVERY-BEGIN:', 'RECOVERY-END:']},
        'image': {'title': 'Image booklet', 'inputs': ['inputs/image.md'], 'inline_image': True, 'image_sha256': image_sha,
                  'contents': ['Image booklet'], 'markers': ['IMAGE-MARKER: Keep the supplied illustration or report unsupported.'],
                  'chapter_markers': ['IMAGE-MARKER:']},
    }
    (workspace / 'cases.json').write_text(json.dumps(cases, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    return cases


def recipe(tool):
    common = '''# Export integration recipe

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

'''
    if tool == 'agent':
        return common + '''## Agent EPUB 0.5.0

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
'''
    return common + '''## Pandoc 3.13 with the small EPUB wrapper

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
'''


PROMPT = '''Complete the supplied EPUB export workflow using the local tool as its user.
Work only in this directory. Read RECIPE.md and cases.json, then install,
discover and perform report, guide, inline, recovery and image in that exact
order. Use actual tool calls. Inspect the independent check result and repair
failures within three build attempts per case. Image is a separate support
probe: retain a valid image or report unsupported; do not rewrite it away.
Do not read or modify .measurement, ./tool, installed/, download/, inputs/,
assets/, RECIPE.md or cases.json. You may write requests/ and out/, and make
only the explicitly allowed fragment edit to work/recovery.md after its first
failed attempt. Do not inspect the product repository, install other software,
access external services, alter the harness, or delegate. The official archives
are already downloaded; use ./tool install to extract/configure this workspace.
Use /opt/homebrew/bin/python3 for JSON helpers to avoid host shell shims.
Use existing files without duplicating manuscripts. Stream the inline content
from cases.json. Do not launch parallel builds. Each ./tool invocation snapshots
and independently validates the resulting book before reporting its outcome;
no extra validation call is required after a passed common check. Stop retrying
at three attempts per case and continue to the next case. End with a short
accurate outcome report. All source data is synthetic.'''


def prepare(workspace, tool, validator, started):
    workspace.mkdir(parents=True, exist_ok=False)
    cases = corpus(workspace)
    control = workspace / '.measurement'
    for name in ['runtime.py', 'score.py', 'pandoc_epub.py', 'source_links.lua']:
        shutil.copyfile(HERE / name, control / name)
    (workspace / 'RECIPE.md').write_text(recipe(tool), encoding='utf-8')
    (workspace / 'tool').write_text(f'#!{sys.executable}\nimport runpy\nrunpy.run_path(".measurement/runtime.py", run_name="__main__")\n')
    # runpy does not automatically add the target script directory to sys.path.
    (workspace / 'tool').write_text(f'#!{sys.executable}\nimport runpy,sys\nsys.path.insert(0,".measurement")\nrunpy.run_path(".measurement/runtime.py", run_name="__main__")\n')
    (workspace / 'tool').chmod(0o755)
    config = {'tool': tool, 'validator': str(validator.resolve()), 'started': started, 'cases': cases}
    (control / 'config.json').write_text(json.dumps(config))
    fixed = [p for folder in ['inputs', 'assets'] for p in (workspace / folder).rglob('*') if p.is_file()]
    fixed += [workspace / 'cases.json', workspace / 'RECIPE.md', workspace / 'tool']
    (control / 'fixed_hashes.json').write_text(json.dumps({str(p.relative_to(workspace)): digest(p) for p in fixed}, indent=2))
    return config


def fetch(workspace, tool):
    started = time.monotonic()
    download = workspace / 'download'
    if tool == 'agent':
        subprocess.run(['gh', 'release', 'download', 'v0.5.0', '--repo', 'benvds/epub-rust', '--pattern', AGENT_ARCHIVE, '--pattern', AGENT_ARCHIVE + '.sha256', '--dir', str(download)], check=True, capture_output=True, timeout=90)
        original = download / AGENT_ARCHIVE
        expected = (download / (AGENT_ARCHIVE + '.sha256')).read_text().split()[0]
        url = 'https://github.com/benvds/epub-rust/releases/download/v0.5.0/' + AGENT_ARCHIVE
        original.rename(download / 'tool.zip')
    else:
        subprocess.run(['curl', '--fail', '--location', '--silent', '--show-error', '--max-time', '90', '-o', str(download / 'tool.zip'), PANDOC_URL], check=True, capture_output=True, timeout=95)
        expected, url = PANDOC_SHA, PANDOC_URL
    actual = digest(download / 'tool.zip')
    if actual != expected:
        raise RuntimeError('Official archive checksum mismatch.')
    return {'url': url, 'sha256': actual, 'bytes': (download / 'tool.zip').stat().st_size,
            'seconds': round(time.monotonic() - started, 6)}


def run_agent(workspace, config, executable):
    command = [str(executable), 'exec', '--ephemeral', '--ignore-user-config', '--skip-git-repo-check',
               '--sandbox', 'workspace-write', '--json', '--model', MODEL,
               '-c', 'model_reasoning_effort="high"', '-c', 'service_tier="default"',
               '-c', 'shell="/bin/bash"', '-C', str(workspace), '-o', str(workspace / '.measurement/final.txt'), '-']
    control = workspace / '.measurement'
    (control / 'client-command.json').write_text(json.dumps(command, indent=2))
    (control / 'prompt.txt').write_text(PROMPT)
    env = dict(os.environ)
    # Keep installed toolchains/shims out of benchmark shell startup. Codex itself
    # remains authenticated through its existing account; credentials are not copied.
    env['PATH'] = '/opt/homebrew/bin:/usr/bin:/bin:/usr/sbin:/sbin'
    process = subprocess.Popen(command, cwd=workspace, env=env, stdin=subprocess.PIPE,
                               stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, start_new_session=True)
    process.stdin.write(PROMPT)
    process.stdin.close()
    streams = selectors.DefaultSelector()
    streams.register(process.stdout, selectors.EVENT_READ, 'stdout')
    streams.register(process.stderr, selectors.EVENT_READ, 'stderr')
    timed_out = False
    events = []
    errors = []
    while streams.get_map():
        if time.monotonic() - config['started'] > 240 and process.poll() is None:
            os.killpg(process.pid, signal.SIGTERM)
            timed_out = True
        for key, _ in streams.select(timeout=1):
            line = key.fileobj.readline()
            if not line:
                streams.unregister(key.fileobj)
            elif key.data == 'stderr':
                errors.append(line)
            else:
                try:
                    event = json.loads(line)
                except ValueError:
                    event = {'type': 'unparsed', 'line': line}
                events.append({'elapsed_seconds': round(time.monotonic() - config['started'], 6), 'event': event})
    process.wait(timeout=10)
    (control / 'client.events.jsonl').write_text(''.join(json.dumps(event, ensure_ascii=False) + '\n' for event in events), encoding='utf-8')
    (control / 'client.stderr.txt').write_text(''.join(errors), encoding='utf-8')
    return {'return_code': process.returncode, 'timed_out': timed_out, 'elapsed_seconds': round(time.monotonic() - config['started'], 6)}


def summarize_run(workspace, run):
    control = workspace / '.measurement'
    calls = [json.loads(line) for line in (control / 'calls.jsonl').read_text().splitlines()] if (control / 'calls.jsonl').exists() else []
    events = [json.loads(line)['event'] for line in (control / 'client.events.jsonl').read_text().splitlines()]
    usage = {key: sum(e.get('usage', {}).get(key, 0) for e in events if e.get('type') == 'turn.completed') for key in ['input_tokens', 'cached_input_tokens', 'output_tokens', 'reasoning_output_tokens']}
    fixed = json.loads((control / 'fixed_hashes.json').read_text())
    changed = [name for name, expected in fixed.items() if digest(workspace / name) != expected]
    result = dict(run, usage=usage, changed_protected_sources=changed, manual_interventions=0,
                  client_turns=sum(e.get('type') == 'turn.started' for e in events), model_turns=None,
                  model_turns_note='CLI does not expose individual model response boundaries; client turns and real command events are retained.',
                  shell_tool_calls=sum(e.get('type') == 'item.completed' and e.get('item', {}).get('type') == 'command_execution' for e in events),
                  runtime_calls=len(calls), cases={})
    by_purpose = {}
    for e in events:
        if e.get('type') == 'item.completed' and e.get('item', {}).get('type') == 'command_execution':
            command = e['item'].get('command', '')
            purpose = 'setup' if './tool install' in command else 'discovery' if './tool discover' in command else 'conversion_or_repair' if './tool ' in command else 'read_or_prepare'
            by_purpose[purpose] = by_purpose.get(purpose, 0) + 1
    result['shell_calls_by_purpose'] = by_purpose
    previous = 0
    for case in CASES:
        attempts = [call for call in calls if call.get('case') == case and call['action'] == case]
        correct = next((call for call in attempts if call.get('outcome') in ['correct', 'unsupported']), None)
        completion = correct['end_seconds'] if correct else None
        result['cases'][case] = {
            'outcome': correct.get('outcome') if correct and not changed else 'failed',
            'attempts': len(attempts), 'retries': max(0, len(attempts) - 1),
            'first_attempt_correct': bool(attempts and attempts[0].get('outcome') == 'correct' and not changed),
            'completion_seconds': completion, 'segment_seconds': round(completion - previous, 6) if completion is not None and previous is not None else None,
            'converter_seconds': round(sum(c.get('converter_seconds', 0) for c in attempts), 6),
            'validation_seconds': round(sum(c.get('independent', {}).get('validation_seconds', 0) for c in attempts), 6),
            'snapshots': [c['artifact'] for c in attempts if 'artifact' in c],
        }
        previous = completion
    return result


def export(workspace, destination, result):
    destination.mkdir(parents=True, exist_ok=False)
    control = workspace / '.measurement'
    for name in ['calls.jsonl', 'client.events.jsonl', 'client.stderr.txt', 'final.txt', 'prompt.txt', 'client-command.json', 'fixed_hashes.json']:
        source = control / name
        if source.exists():
            text = source.read_text().replace(str(workspace), '<WORKSPACE>').replace(str(Path.home()), '<USER_HOME>')
            (destination / name).write_text(text, encoding='utf-8')
    if (control / 'artifacts').exists():
        shutil.copytree(control / 'artifacts', destination / 'artifacts')
    (destination / 'result.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def native_arguments(tool, case, definition):
    args = ['build'] if tool == 'agent' else []
    args += ['--output', f'out/{case}.epub', '--title', definition['title'], '--language', 'en', '--author', 'Benchmark Author']
    for source in definition.get('inputs', []):
        args += ['--input', source]
    if definition.get('cover'):
        args += ['--cover', definition['cover']]
    if definition.get('split_level'):
        args += ['--split-level', str(definition['split_level'])]
    if tool == 'agent':
        args += ['--json']
    return args


def smoke(args):
    # Converter/scorer integration smoke only; never counted as an agent run.
    for tool, archive in [('agent', args.agent_archive), ('pandoc', args.pandoc_archive)]:
        workspace = args.work / f'smoke-{tool}'
        config = prepare(workspace, tool, args.validator, time.monotonic())
        shutil.copyfile(archive, workspace / 'download/tool.zip')
        config['archive_sha256'] = digest(archive)
        (workspace / '.measurement/config.json').write_text(json.dumps(config))
        def invoke(case, arguments, content=''):
            output = subprocess.run([str(workspace / 'tool'), case, *arguments], cwd=workspace, input=content,
                                    text=True, capture_output=True, timeout=65)
            return output.returncode, json.loads(output.stdout)
        assert invoke('install', [])[0] == 0
        for case in CASES:
            definition = config['cases'][case]
            command = native_arguments(tool, case, definition)
            content = ''
            if case == 'inline':
                if tool == 'agent':
                    command = ['build', '--request', '-', '--output', 'out/inline.epub', '--json']
                    content = json.dumps({'schema_version': 1, 'metadata': {'title': definition['title'], 'language': 'en', 'author': 'Benchmark Author'}, 'inputs': [{'name': 'inline.md', 'format': 'markdown', 'content': definition['content']}]})
                else:
                    command += ['--stdin']
                    content = definition['content']
            code, result = invoke(case, command, content)
            if case == 'recovery':
                assert code != 0, result
                source = workspace / 'work/recovery.md'
                source.write_text(source.read_text().replace('(#missing)', '(#results)'))
                code, result = invoke(case, command + ['--overwrite'])
            if case == 'image' and tool == 'agent':
                assert result.get('unsupported_confirmed'), result
            else:
                assert code == 0, (tool, case, result)
        # Confirm the content checker catches a valid but incorrect publication.
        from score import score
        wrong_case = dict(config['cases']['guide'], markers=['NONEXISTENT-SOURCE-TEXT'])
        assert score(workspace / 'out/guide.epub', wrong_case, args.validator)['status'] == 'failed'
        print(json.dumps({'smoke': tool, 'status': 'passed'}), flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='action', required=True)
    smoke_parser = sub.add_parser('smoke')
    smoke_parser.add_argument('--work', type=Path, required=True)
    smoke_parser.add_argument('--agent-archive', type=Path, required=True)
    smoke_parser.add_argument('--pandoc-archive', type=Path, required=True)
    smoke_parser.add_argument('--validator', type=Path, required=True)
    run_parser = sub.add_parser('run')
    run_parser.add_argument('--work', type=Path, required=True)
    run_parser.add_argument('--out', type=Path, required=True)
    run_parser.add_argument('--pair', type=int, choices=range(1, 4), required=True)
    run_parser.add_argument('--tool', choices=['agent', 'pandoc'], required=True)
    run_parser.add_argument('--validator', type=Path, required=True)
    run_parser.add_argument('--codex', type=Path, required=True)
    args = parser.parse_args()
    if args.action == 'smoke':
        smoke(args)
        return
    # Corpus and recipes are supplied before the cold clock, as completed source
    # material would be in the consuming application. Tool download starts it.
    workspace = args.work.resolve() / f'pair-{args.pair}-{args.tool}'
    config = prepare(workspace, args.tool, args.validator, 0)
    config['started'] = time.monotonic()
    downloaded = fetch(workspace, args.tool)
    config['archive_sha256'] = downloaded['sha256']
    (workspace / '.measurement/config.json').write_text(json.dumps(config))
    run = run_agent(workspace, config, args.codex)
    run.update(pair=args.pair, tool=args.tool, download=downloaded, requested_model=MODEL,
               reasoning_effort='high', service_tier='default', client_version=subprocess.check_output([str(args.codex), '--version'], text=True).strip(),
               host=platform.platform(), effective_model='not exposed by CLI event stream',
               python_version=platform.python_version(), run_order=ORDER.index((args.pair, args.tool)) + 1)
    result = summarize_run(workspace, run)
    export(workspace, args.out.resolve() / f'pair-{args.pair}-{args.tool}', result)
    print(json.dumps(result, ensure_ascii=False, indent=2), flush=True)


if __name__ == '__main__':
    main()
