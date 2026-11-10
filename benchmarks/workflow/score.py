"""Generator-independent checks for the declared synthetic export corpus."""
import hashlib
from pathlib import PurePosixPath
import posixpath
import subprocess
import time
from urllib.parse import unquote, urlsplit
import xml.etree.ElementTree as ET
import zipfile

XHTML = '{http://www.w3.org/1999/xhtml}'
DC = '{http://purl.org/dc/elements/1.1/}'
OPF = '{http://www.idpf.org/2007/opf}'
EPUB = '{http://www.idpf.org/2007/ops}'


def score(path, case, validator):
    started = time.monotonic()
    issues = []
    validator_result = subprocess.run([str(validator), str(path)], capture_output=True, text=True, timeout=45)
    if validator_result.returncode:
        issues.append('EPUBCheck failed: ' + (validator_result.stdout + validator_result.stderr)[-6000:])
    try:
        with zipfile.ZipFile(path) as archive:
            names = archive.namelist()
            if len(names) != len(set(names)) or archive.testzip():
                issues.append('Archive has duplicate or corrupt members.')
            if names[0] != 'mimetype' or archive.read('mimetype') != b'application/epub+zip':
                issues.append('Invalid EPUB mimetype.')
            container = ET.fromstring(archive.read('META-INF/container.xml'))
            rootfile = next(n for n in container.iter() if n.tag.endswith('}rootfile')).attrib['full-path']
            package = ET.fromstring(archive.read(rootfile))
            base = str(PurePosixPath(rootfile).parent)
            manifest = {n.attrib['id']: n for n in package.findall(f'{OPF}manifest/{OPF}item')}
            resource = lambda item: posixpath.normpath(posixpath.join(base, unquote(item.attrib['href'])))
            spine = [resource(manifest[n.attrib['idref']]) for n in package.findall(f'{OPF}spine/{OPF}itemref')]
            documents = {name: ET.fromstring(archive.read(name)) for name in names
                         if name.endswith(('.xhtml', '.html', '.opf', '.xml', '.svg'))}
            for name, document in documents.items():
                ids = [n.attrib['id'] for n in document.iter() if 'id' in n.attrib]
                if len(ids) != len(set(ids)):
                    issues.append(f'Duplicate XML IDs in {name}.')
                for node in document.iter():
                    for attr in ('href', 'src'):
                        target = node.attrib.get(attr)
                        if not target or urlsplit(target).scheme or target.startswith('//'):
                            continue
                        url = urlsplit(target)
                        resolved = posixpath.normpath(posixpath.join(posixpath.dirname(name), unquote(url.path))) if url.path else name
                        if resolved not in names:
                            issues.append(f'Missing resource: {name} -> {target}')
                        elif url.fragment and (resolved not in documents or unquote(url.fragment) not in {
                            n.attrib.get('id') for n in documents[resolved].iter()
                        }):
                            issues.append(f'Missing fragment: {name} -> {target}')
            for key, expected in [('title', case['title']), ('language', 'en'), ('creator', 'Benchmark Author')]:
                actual = package.find(f'{OPF}metadata/{DC}{key}')
                if actual is None or actual.text != expected:
                    issues.append(f'Incorrect metadata {key}.')
            bodies = [(name, documents[name].find(f'{XHTML}body')) for name in spine if name in documents]
            texts = [(name, ' '.join(' '.join(body.itertext()).split())) for name, body in bodies if body is not None]
            joined = ' '.join(text for _, text in texts)
            positions = [joined.find(marker) for marker in case['markers']]
            if any(p < 0 for p in positions) or positions != sorted(positions):
                issues.append('Missing or reordered source prose markers.')
            if any(marker in joined for marker in case.get('forbidden', [])):
                issues.append('Forbidden marker remained.')
            nav_item = next(n for n in manifest.values() if 'nav' in n.attrib.get('properties', '').split())
            nav = documents[resource(nav_item)]
            toc = next(n for n in nav.iter(f'{XHTML}nav') if 'toc' in n.attrib.get(f'{EPUB}type', '').split())
            links = [((' '.join(' '.join(n.itertext()).split())), n.attrib.get('href', '')) for n in toc.iter(f'{XHTML}a')]
            labels = [label for label, _ in links]
            if labels != case['contents']:
                issues.append(f'Contents order differs: {labels!r}')
            # Every requested chapter must occupy a distinct reading resource;
            # extra title/cover/contents pages are allowed for either generator.
            chapter_resources = []
            for marker in case['chapter_markers']:
                matches = [name for name, text in texts if marker in text]
                if len(matches) != 1:
                    issues.append(f'Chapter marker must occur once: {marker}')
                else:
                    chapter_resources.append(matches[0])
            if len(chapter_resources) != len(set(chapter_resources)):
                issues.append('Requested chapter boundaries were merged.')
            if case.get('table'):
                tables = [n for _, body in bodies if body is not None for n in body.iter(f'{XHTML}table')]
                if len(tables) != 1 or len(list(tables[0].iter(f'{XHTML}th'))) != 2:
                    issues.append('Expected one semantic two-column table.')
                elif [' '.join(''.join(n.itertext()).split()) for n in tables[0].iter(f'{XHTML}td')] != ['Café & tea', '7', '日本語', '11']:
                    issues.append('Table content differs.')
            if case.get('notes'):
                refs = [(name, n) for name, body in bodies if body is not None for n in body.iter(f'{XHTML}a')
                        if 'noteref' in n.attrib.get(f'{EPUB}type', '').split()]
                if len(refs) < case['notes'] or 'Evidence stays complete.' not in joined:
                    issues.append('Missing note references or note content.')
                for source_name, ref in refs:
                    if not ref.attrib.get('id'):
                        issues.append('Note reference has no return target.')
                    elif not any((posixpath.normpath(posixpath.join(posixpath.dirname(name), unquote(n.attrib.get('href', '').split('#', 1)[0]))) if n.attrib.get('href', '').split('#', 1)[0] else name) == source_name
                                 and n.attrib.get('href', '').split('#')[-1] == ref.attrib['id']
                                 for name, body in bodies if body is not None for n in body.iter(f'{XHTML}a') if '#' in n.attrib.get('href', '')):
                        issues.append('Note reference has no backlink.')
            if case.get('image_sha256'):
                matches = [name for name in names if hashlib.sha256(archive.read(name)).hexdigest() == case['image_sha256']]
                if not matches:
                    issues.append('Requested cover/image bytes missing.')
            if case.get('inline_image'):
                images = [n for _, body in bodies if body is not None for n in body.iter(f'{XHTML}img')]
                if not any(n.attrib.get('alt') == 'Supplied diagram' for n in images):
                    issues.append('Inline image alternative text missing.')
    except (KeyError, ValueError, ET.ParseError, zipfile.BadZipFile, StopIteration) as error:
        issues.append(f'Package inspection failed: {error}')
    return {'status': 'passed' if not issues else 'failed', 'issues': issues,
            'epubcheck_exit': validator_result.returncode,
            'validation_seconds': round(time.monotonic() - started, 6)}
