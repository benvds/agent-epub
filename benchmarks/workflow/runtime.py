#!/usr/bin/env python3
"""Measured install/discovery/converter/check entry point for isolated sessions."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time
import zipfile

from score import score


def main():
    root = Path.cwd()
    control = root / '.measurement'
    config = json.loads((control / 'config.json').read_text())
    arguments = sys.argv[1:]
    action = arguments.pop(0) if arguments else 'help'
    started = time.monotonic()
    record = {'action': action, 'arguments': arguments, 'start_seconds': started - config['started']}
    code = 0
    result = None
    if action == 'install':
        archive = root / 'download/tool.zip'
        if hashlib.sha256(archive.read_bytes()).hexdigest() != config['archive_sha256']:
            raise RuntimeError('Downloaded archive checksum mismatch.')
        with zipfile.ZipFile(archive) as package:
            target = root / 'installed'
            for name in package.namelist():
                if not (target / name).resolve().is_relative_to(target.resolve()):
                    raise RuntimeError('Unsafe archive member.')
            package.extractall(target)
        executable = next(p for p in target.rglob('agent-epub' if config['tool'] == 'agent' else 'pandoc') if p.is_file())
        executable.chmod(0o755)
        (control / 'executable.txt').write_text(str(executable))
        version = subprocess.check_output([str(executable), '--version'], text=True).splitlines()[0]
        result = {'status': 'installed', 'version': version}
    elif action in ('discover', 'report', 'guide', 'inline', 'recovery', 'image', 'check'):
        executable = Path((control / 'executable.txt').read_text())
        env = dict(os.environ, PANDOC_BINARY=str(executable))
        if action == 'discover':
            command = [str(executable), 'capabilities'] if config['tool'] == 'agent' else [sys.executable, str(control / 'pandoc_epub.py'), '--help']
        elif action == 'check':
            case = arguments.pop(0)
            result = score(root / 'out' / f'{case}.epub', config['cases'][case], config['validator'])
            code = 0 if result['status'] == 'passed' else 4
            record['case'] = case
            command = None
        else:
            case = action
            record['case'] = case
            calls = [json.loads(line) for line in (control / 'calls.jsonl').read_text().splitlines()] if (control / 'calls.jsonl').exists() else []
            attempt = 1 + sum(call['action'] == case for call in calls)
            record['attempt'] = attempt
            if attempt > 3:
                result, code, command = {'status': 'failed', 'issues': ['Three-attempt cap reached.']}, 2, None
            else:
                command = [str(executable), *arguments] if config['tool'] == 'agent' else [sys.executable, str(control / 'pandoc_epub.py'), *arguments]
        if command:
            text = sys.stdin.read() if action not in ('discover', 'check') else None
            converted = subprocess.run(command, input=text, text=True, capture_output=True, env=env, timeout=60)
            record['converter_seconds'] = round(time.monotonic() - started, 6)
            record['converter_exit'] = converted.returncode
            record['converter_stdout'] = converted.stdout
            record['converter_stderr'] = converted.stderr
            code = converted.returncode
            result = {'converter_exit': code, 'stdout': converted.stdout, 'stderr': converted.stderr}
            if action not in ('discover', 'check'):
                artifact = root / 'out' / f'{action}.epub'
                if code == 0 and artifact.is_file():
                    snapshot = control / 'artifacts' / f'{action}-{record["attempt"]}.epub'
                    snapshot.parent.mkdir(exist_ok=True)
                    shutil.copyfile(artifact, snapshot)
                    record['artifact'] = str(snapshot.relative_to(root))
                    record['artifact_sha256'] = hashlib.sha256(snapshot.read_bytes()).hexdigest()
                    checked = score(snapshot, config['cases'][action], config['validator'])
                    result['independent'] = record['independent'] = checked
                    code = 0 if checked['status'] == 'passed' else 4
                    record['outcome'] = 'correct' if code == 0 else 'incorrect'
                elif action == 'image' and config['tool'] == 'agent' and code == 2:
                    try:
                        native = json.loads(converted.stdout)
                        unsupported = native['errors'][0]['issue_code'] == 'unsupported_inline_image'
                    except (KeyError, ValueError, IndexError):
                        unsupported = False
                    record['outcome'] = 'unsupported' if unsupported else 'failed'
                    result['unsupported_confirmed'] = unsupported
                else:
                    record['outcome'] = 'failed'
    else:
        result = {'usage': './tool install | discover | CASE NATIVE_ARGUMENTS | check CASE',
                  'cases': ['report', 'guide', 'inline', 'recovery', 'image']}
    record.update(end_seconds=round(time.monotonic() - config['started'], 6), exit_code=code)
    if result is not None:
        record['result'] = result
    with (control / 'calls.jsonl').open('a') as stream:
        stream.write(json.dumps(record, ensure_ascii=False) + '\n')
    print(json.dumps(result, ensure_ascii=False))
    raise SystemExit(code)


if __name__ == '__main__':
    main()
