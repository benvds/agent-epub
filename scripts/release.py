#!/usr/bin/env python3
"""Build allowlisted release ZIPs and smoke-test their installed CLI contract.

Python and Cargo are packaging/test tools, not generation dependencies.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import tempfile
import tomllib
import zipfile
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
TARGETS = ("aarch64-apple-darwin", "x86_64-unknown-linux-gnu", "x86_64-pc-windows-msvc")
FILES = (
    "README.md", "LICENSE", "docs/INSTALL.md", "docs/USAGE.md",
    "docs/AGENT_INTERFACE.md", "schemas/book-request.v1.json",
    "examples/introduction.txt", "examples/guide.md", "examples/book-request.json",
    "examples/research-report.md", "examples/report-request.json",
    "examples/report-inline.json", "examples/illustrated-guide.md",
    "examples/illustrated-request.json", "examples/illustrated-inline.json",
    "examples/assets/export-flow.png", "examples/assets/landscape.jpg", "examples/assets/divider.png",
    "examples/title-h2.md",
    "examples/title-h2-request.json", "examples/title-h2-inline.json", "scripts/epubcheck.sh",
    "skills/agent-epub/SKILL.md", "skills/agent-epub/references/examples.md",
)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(binary, args, cwd, env=None, content=None, code=0):
    result = subprocess.run(
        [str(binary), *map(str, args)], cwd=cwd, env=env, input=content,
        capture_output=True, text=True, encoding="utf-8", timeout=60,
    )
    if result.returncode != code:
        raise RuntimeError(f"{args}: exit {result.returncode}, expected {code}\n"
                           f"{result.stdout}\n{result.stderr}")
    return result.stdout


def notices(target):
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--locked", "--format-version", "1",
         "--filter-platform", target], cwd=ROOT, text=True, encoding="utf-8"))
    nodes = {node["id"]: node for node in metadata["resolve"]["nodes"]}
    reached, pending = set(), [metadata["resolve"]["root"]]
    while pending:
        package_id = pending.pop()
        if package_id in reached:
            continue
        reached.add(package_id)
        pending.extend(nodes[package_id]["dependencies"])
    sections = ["Third-party notices for locked dependencies (including build tools).\n"
                "Project sources are MIT; dependencies retain their own terms.\n"]
    for package in sorted(metadata["packages"], key=lambda p: (p["name"], p["version"])):
        if package["id"] not in reached or package["source"] is None:
            continue
        root = Path(package["manifest_path"]).parent
        files = sorted(p for p in root.rglob("*") if p.is_file()
                       and len(p.relative_to(root).parts) <= 3
                       and p.name.lower().startswith(
                           ("license", "licence", "copying", "copyright", "notice")))
        if not files:
            raise RuntimeError(f"No license texts found for {package['name']}")
        sections.append(f"\n{'=' * 72}\n{package['name']} {package['version']}\n"
                        f"License expression: {package['license']}\n"
                        f"Repository: {package.get('repository') or 'not supplied'}\n")
        for file in files:
            sections.append(f"\n--- {file.relative_to(root)} ---\n"
                            + file.read_text(encoding="utf-8"))
    # The standard library is linked into the executable too.
    sysroot = Path(subprocess.check_output(["rustc", "--print", "sysroot"], text=True).strip())
    rust_docs = sysroot / "share/doc/rust"
    rust_licenses = sorted((rust_docs / "licenses").glob("*"))
    if not rust_licenses or not (rust_docs / "COPYRIGHT-library.html").is_file():
        raise RuntimeError("Rust standard-library notices missing from toolchain")
    for file in [rust_docs / "COPYRIGHT-library.html", *rust_licenses]:
        if file.is_file():
            sections.append(f"\n--- Rust standard library: {file.name} ---\n"
                            + file.read_text(encoding="utf-8"))
    return "\n".join(sections)


def pack(args):
    version = tomllib.loads((ROOT / "Cargo.toml").read_text())["package"]["version"]
    binary = args.binary.resolve()
    if run(binary, ["--version"], ROOT).strip() != f"agent-epub {version}":
        raise RuntimeError("Binary version does not match package version")
    out = args.out.resolve()
    out.mkdir(parents=True, exist_ok=True)
    name = f"agent-epub-{version}-{args.target}"
    archive = out / f"{name}.zip"
    with tempfile.TemporaryDirectory() as temporary:
        stage = Path(temporary) / name
        stage.mkdir()
        for relative in FILES:
            destination = stage / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / relative, destination)
        executable = stage / ("agent-epub.exe" if "windows" in args.target else "agent-epub")
        shutil.copyfile(binary, executable)
        executable.chmod(0o755)
        (stage / "scripts/epubcheck.sh").chmod(0o755)
        (stage / "THIRD_PARTY_NOTICES.txt").write_text(notices(args.target), encoding="utf-8")
        result = json.loads(run(executable, ["build", "--request", "examples/report-request.json",
                                           "--output", "examples/sample-report.epub", "--json"], stage))
        if result["checks"]["structural_profile"] != "passed":
            raise RuntimeError("Sample build did not pass structural checks")
        illustrated = json.loads(run(executable, ["build", "--request", "examples/illustrated-request.json",
                                                 "--output", "examples/sample-illustrated.epub", "--json"], stage))
        assert illustrated["images"] == 3 and illustrated["cover"] and illustrated["chapters"] == 3
        with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_DEFLATED) as package:
            for file in sorted(stage.rglob("*")):
                if file.is_file():
                    package.write(file, file.relative_to(stage.parent).as_posix())
    archive.with_suffix(".zip.sha256").write_text(
        f"{digest(archive)}  {archive.name}\n", encoding="ascii")
    print(archive)


def reading_resources(path):
    with zipfile.ZipFile(path) as book:
        return {name: book.read(name) for name in book.namelist()
                if name == "EPUB/nav.xhtml" or name.startswith("EPUB/chapter-")}


def inspect_report(path):
    resources = reading_resources(path)
    ns = {"h": "http://www.w3.org/1999/xhtml", "e": "http://www.idpf.org/2007/ops"}
    first = ET.fromstring(resources["EPUB/chapter-0001.xhtml"])
    second = ET.fromstring(resources["EPUB/chapter-0002.xhtml"])
    assert len(resources) == 3
    assert [cell.text for cell in first.findall(".//h:th", ns)] == ["Item", "Result"]
    assert all(cell.get("scope") == "col" for cell in first.findall(".//h:th", ns))
    assert "Café guide" in "".join(first.itertext())
    assert "a | b" in "".join(first.itertext())
    refs = [a for root in [first, second] for a in root.findall(".//h:a", ns)
            if a.get(f"{{{ns['e']}}}type") == "noteref"]
    assert len(refs) == 3
    nav = ET.fromstring(resources["EPUB/nav.xhtml"])
    assert [a.text for a in nav.findall(".//h:nav/h:ol//h:a", ns)] == [
        "Findings", "Observations", "Method", "Detail", "Summary", "Discussion", "Follow-up"]
    return resources


def smoke(args):
    archive = args.archive.resolve()
    expected = archive.with_suffix(".zip.sha256").read_text(encoding="ascii").split()
    assert expected == [digest(archive), archive.name], "Archive checksum mismatch"
    report = {"archive": archive.name, "sha256": digest(archive), "platform": platform.platform(),
              "machine": platform.machine(), "rust_on_path": False, "java_on_path": False,
              "epubcheck": "not_run", "reader_review": "not_run", "accessibility": "not_run"}
    with tempfile.TemporaryDirectory(prefix="agent-epub-install-") as temporary:
        install = Path(temporary)
        with zipfile.ZipFile(archive) as package:
            for entry in package.infolist():
                assert not entry.filename.startswith("/") and ".." not in Path(entry.filename).parts
            package.extractall(install)
        stage, = [p for p in install.iterdir() if p.is_dir()]
        binary = stage / ("agent-epub.exe" if os.name == "nt" else "agent-epub")
        binary.chmod(0o755)  # ZIP extraction tools do not all preserve executable bits.
        empty_path = install / "empty-path"
        empty_path.mkdir()
        env = {**os.environ, "PATH": str(empty_path)}
        for tool in ["cargo", "rustc", "java"]:
            assert shutil.which(tool, path=env["PATH"]) is None
        report["version"] = run(binary, ["--version"], stage, env).strip()
        capabilities = json.loads(run(binary, ["capabilities"], stage, env))
        assert report["version"] == f"agent-epub {capabilities['tool_version']}"
        assert capabilities["request_schema_document"]["properties"]["schema_version"]["const"] == 1
        assert "rectangular_pipe_tables" in capabilities["markdown_extensions"]
        assert "footnotes_with_backlinks" in capabilities["markdown_extensions"]
        original = inspect_report(stage / "examples/sample-report.epub")
        inline = (stage / "examples/report-inline.json").read_text(encoding="utf-8")
        source = (stage / "examples/research-report.md").read_text(encoding="utf-8")
        assert json.loads(inline)["inputs"][0]["content"] == source
        modes = {
            "direct": (["--input", "examples/research-report.md", "--title",
                        "Agent EPUB sample report", "--language", "en"], None),
            "request": (["--request", "examples/report-request.json"], None),
            "streamed": (["--request", "-"], inline),
        }
        for mode, (options, content) in modes.items():
            output = stage / f"{mode}.epub"
            result = json.loads(run(binary, ["build", *options, "--output", output, "--json"],
                                    stage, env, content))
            assert result["status"] == "success" and Path(result["output"]).is_file()
            assert result["chapters"] == 2
            assert result["checks"] == {"structural_profile": "passed", "epubcheck": "not_run"}
            assert inspect_report(output) == original
            checked = json.loads(run(binary, ["validate", output, "--json"], stage, env))
            assert checked["status"] == "success"
        h2_inline = (stage / "examples/title-h2-inline.json").read_text(encoding="utf-8")
        h2_modes = {
            "direct": (["--input", "examples/title-h2.md", "--split-level", "2", "--title", "Field guide", "--language", "en"], None),
            "request": (["--request", "examples/title-h2-request.json"], None),
            "streamed": (["--request", "-"], h2_inline),
        }
        h2_resources = None
        for mode, (options, content) in h2_modes.items():
            output = stage / f"h2-{mode}.epub"
            result = json.loads(run(binary, ["build", *options, "--output", output, "--json"], stage, env, content))
            assert result["chapters"] == 4 and result["status"] == "success"
            with zipfile.ZipFile(output) as archive:
                resources = {name: archive.read(name) for name in archive.namelist()
                             if name == "EPUB/nav.xhtml" or name.startswith("EPUB/chapter-")}
            if h2_resources is None:
                h2_resources = resources
            assert resources == h2_resources
        assert capabilities["inline_images"] is True
        illustrated_inline = (stage / "examples/illustrated-inline.json").read_text(encoding="utf-8")
        image_modes = {
            "direct": (["--input", "examples/illustrated-guide.md", "--split-level", "2", "--title", "Illustrated guide", "--language", "en", "--cover", "examples/assets/landscape.jpg"], None),
            "request": (["--request", "examples/illustrated-request.json"], None),
            "streamed": (["--request", "-", "--base-dir", "examples"], illustrated_inline),
        }
        def reading_resources(path):
            with zipfile.ZipFile(path) as archive:
                return {name: archive.read(name) for name in archive.namelist()
                        if name != "EPUB/package.opf" and name not in ["mimetype", "META-INF/container.xml"]}
        illustrated_reference = reading_resources(stage / "examples/sample-illustrated.epub")
        for mode, (options, content) in image_modes.items():
            output = stage / f"images-{mode}.epub"
            result = json.loads(run(binary, ["build", *options, "--output", output, "--json"], stage, env, content))
            assert result["images"] == 3 and result["cover"] and result["chapters"] == 3
            assert reading_resources(output) == illustrated_reference
        protected = stage / "direct.epub"
        before = protected.read_bytes()
        failed = json.loads(run(binary, ["build", "--request", "-", "--output", protected,
                                         "--overwrite", "--json"], stage, env,
                                json.dumps({"schema_version": 1, "metadata": {"title": "Bad", "language": "en"},
                                            "inputs": [{"format": "markdown", "content": "![image](https://example.org/image.png)"}]}), 2))
        assert failed["status"] == "failed" and failed["output"] is None
        assert failed["errors"][0]["code"] == "invalid_input"
        issue = failed["errors"][0]
        assert issue["issue_code"] == "unsupported_image_source"
        assert issue["line"] == 1 and issue["column"] == 1 and issue["remedy"]
        assert issue["source"]
        for operation, options in [
            ("build", ["--request", "examples/report-request.json", "--output", protected, "--overwrite"]),
            ("validate", [protected]),
        ]:
            failed = json.loads(run(binary, [operation, *options, "--epubcheck",
                                             stage / "missing-validator", "--json"], stage, env, code=5))
            assert failed["status"] == "failed" and failed["output"] is None
            assert failed["errors"][0]["code"] == "validator_failed"
            assert failed["checks"] == {"structural_profile": "not_claimed", "epubcheck": "not_claimed"}
        conflict = json.loads(run(binary, ["build", "--request", "examples/report-request.json",
                                           "--output", protected, "--json"], stage, env, code=2))
        assert conflict["status"] == "failed" and protected.read_bytes() == before
        report.update(status="passed", input_modes=list(modes), sample_content="passed",
                      failed_and_unrun_checks="passed", output_protection="passed",
                      h2_input_modes="passed", detailed_diagnostics="passed", inline_image_modes="passed")
    args.report.parent.mkdir(parents=True, exist_ok=True)
    args.report.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(report, indent=2))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    create = commands.add_parser("pack")
    create.add_argument("--binary", type=Path, required=True)
    create.add_argument("--target", choices=TARGETS, required=True)
    create.add_argument("--out", type=Path, required=True)
    check = commands.add_parser("smoke")
    check.add_argument("--archive", type=Path, required=True)
    check.add_argument("--report", type=Path, required=True)
    args = parser.parse_args()
    {"pack": pack, "smoke": smoke}[args.command](args)


if __name__ == "__main__":
    main()
