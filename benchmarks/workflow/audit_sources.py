#!/usr/bin/env python3
"""Additional offline text audit of final synthetic artifact snapshots.

This does not rerun agents or alter their timing. Every visible source line in
this small declared Markdown corpus must appear in its final reading content;
note reference numbers and generated navigation are excluded from source text.
This recognizer is intentionally limited to the committed corpus's syntax.
"""
import argparse
import json
from pathlib import Path
import re
import xml.etree.ElementTree as ET
import zipfile

HERE = Path(__file__).resolve().parent
XHTML = '{http://www.w3.org/1999/xhtml}'
EPUB = '{http://www.idpf.org/2007/ops}'


def text(node):
    if 'noteref' in node.attrib.get(EPUB + 'type', '').split():
        return ''
    result = node.text or ''
    if node.tag == XHTML + 'img':
        result += node.attrib.get('alt', '')
    for child in node:
        result += text(child) + (child.tail or '') + ' '
    return result


def fragments(source):
    source = re.sub(r'!?\[([^\]]+)\]\([^\n)]*\)', r'\1', source)
    source = re.sub(r'^\[\^[^\]]+\]:\s*', '', source, flags=re.M)
    source = re.sub(r'\[\^[^\]]+\]', '', source)
    for line in source.splitlines():
        line = re.sub(r'^#{1,6}\s*', '', line).strip()
        if not line:
            continue
        if line.startswith('|'):
            cells = [cell.strip() for cell in line.strip('|').split('|')]
            if all(re.fullmatch(r':?-+:?', cell) for cell in cells):
                continue
            yield from cells
        else:
            yield ' '.join(line.split())


def audit(folder):
    cases = json.loads((HERE / 'corpus/cases.json').read_text())
    results = []
    for path in sorted(folder.glob('pair-*/result.json')):
        run = json.loads(path.read_text())
        for name, case in cases.items():
            measured = run['cases'][name]
            if measured['outcome'] != 'correct':
                continue
            sources = [case['content']] if 'content' in case else [(HERE / 'corpus' / source).read_text() for source in case['inputs']]
            artifact = path.parent / 'artifacts' / Path(measured['snapshots'][-1]).name
            chunks = []
            with zipfile.ZipFile(artifact) as archive:
                for member in archive.namelist():
                    if not member.endswith('.xhtml'):
                        continue
                    body = ET.fromstring(archive.read(member)).find(XHTML + 'body')
                    if body is not None:
                        for nav in list(body.iter(XHTML + 'nav')):
                            nav.clear()
                        chunks.append(' '.join(text(body).split()))
            visible = re.sub(r'\s+([,.;:!?])', r'\1', ' '.join(chunks))
            expected = [fragment for source in sources for fragment in fragments(source)]
            missing = [fragment for fragment in expected if fragment not in visible]
            results.append({'pair': run['pair'], 'tool': run['tool'], 'case': name,
                            'source_fragments': len(expected), 'missing': missing,
                            'status': 'passed' if not missing else 'failed'})
    return {'coverage': 'Post-measurement visible-line audit of every final successful synthetic publication. Does not change timing, prompts, checks or retry outcomes.',
            'status': 'passed' if all(not r['missing'] for r in results) else 'failed', 'publications': results}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('runs', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    result = audit(args.runs)
    args.output.write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result, indent=2))
