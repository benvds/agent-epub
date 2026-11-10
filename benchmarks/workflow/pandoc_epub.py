#!/usr/bin/env python3
"""Small Pandoc EPUB recipe: explicit metadata, chapter level and atomic output."""
import argparse
import os
import posixpath
import xml.etree.ElementTree as ET
import zipfile
from urllib.parse import unquote
from pathlib import Path
import subprocess
import tempfile


def add_note_returns(path):
    """Pandoc EPUB notes have IDs/references but omit visible return links."""
    xhtml = 'http://www.w3.org/1999/xhtml'
    epub = 'http://www.idpf.org/2007/ops'
    ET.register_namespace('', xhtml)
    ET.register_namespace('epub', epub)
    with zipfile.ZipFile(path) as archive:
        entries = [(info, archive.read(info.filename)) for info in archive.infolist()]
    documents = {info.filename: ET.fromstring(data) for info, data in entries if info.filename.endswith('.xhtml')}
    changed = set()
    for name, document in documents.items():
        for reference in document.iter('{' + xhtml + '}a'):
            if 'noteref' not in reference.attrib.get('{' + epub + '}type', '').split():
                continue
            href = reference.attrib.get('href', '')
            resource, fragment = href.split('#', 1)
            target = posixpath.normpath(posixpath.join(posixpath.dirname(name), unquote(resource))) if resource else name
            note = next(n for n in documents[target].iter() if n.attrib.get('id') == unquote(fragment))
            paragraph = ET.SubElement(note, '{' + xhtml + '}p')
            back = ET.SubElement(paragraph, '{' + xhtml + '}a', {
                'href': posixpath.relpath(name, posixpath.dirname(target)) + '#' + reference.attrib['id'],
                'role': 'doc-backlink', 'aria-label': 'Return to note reference'})
            back.text = 'Return to reference'
            changed.add(target)
    if changed:
        replacement = path.with_suffix('.patched.epub')
        with zipfile.ZipFile(replacement, 'w') as archive:
            for info, data in entries:
                archive.writestr(info, ET.tostring(documents[info.filename], encoding='utf-8', xml_declaration=True) if info.filename in changed else data)
        replacement.replace(path)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--input', action='append', default=[])
    parser.add_argument('--stdin', action='store_true', help='Read Markdown content from stdin')
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--title', required=True)
    parser.add_argument('--language', required=True)
    parser.add_argument('--author')
    parser.add_argument('--split-level', type=int, choices=range(1, 7), default=1)
    parser.add_argument('--cover', type=Path)
    parser.add_argument('--overwrite', action='store_true')
    args = parser.parse_args()
    if not args.input and not args.stdin:
        parser.error('Supply --input or --stdin.')
    if args.output.exists() and not args.overwrite:
        parser.error('Output exists; use --overwrite.')
    for source in args.input + ([str(args.cover)] if args.cover else []):
        if args.output.resolve() == Path(source).resolve():
            parser.error('Output cannot replace an input.')
    with tempfile.TemporaryDirectory(dir=args.output.parent, prefix='.pandoc-') as temporary:
        output = Path(temporary) / 'book.epub'
        command = [os.environ['PANDOC_BINARY'], '--from=markdown', '--to=epub3',
                   '--toc', '--toc-depth=6', f'--split-level={args.split_level}',
                   '--metadata', f'title={args.title}', '--metadata', f'lang={args.language}',
                   '--lua-filter', str(Path(__file__).with_name('source_links.lua')),
                   '--output', str(output)]
        if args.author:
            command += ['--metadata', f'author={args.author}']
        if args.cover:
            command += ['--epub-cover-image', str(args.cover)]
        command += ['--resource-path', os.pathsep.join(dict.fromkeys(['.'] + [str(Path(p).parent) for p in args.input]))]
        command += args.input + (['-'] if args.stdin else [])
        result = subprocess.run(command, check=False)
        if result.returncode:
            raise SystemExit(result.returncode)
        add_note_returns(output)
        # This is a benchmark integration recipe, not a general concurrent
        # publication service. Destinations are isolated to one agent session.
        output.replace(args.output)
    print(f'Created {args.output}')


if __name__ == '__main__':
    main()
