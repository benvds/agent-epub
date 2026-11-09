#!/usr/bin/env python3
"""Prepare, execute and independently score real agent tool-use evaluations."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import time
import xml.etree.ElementTree as ET
import zipfile

HERE = Path(__file__).resolve().parent
DC = "{http://purl.org/dc/elements/1.1/}"
OPF = "{http://www.idpf.org/2007/opf}"
XHTML = "{http://www.w3.org/1999/xhtml}"


def prepare(binary, workspace, covers):
    workspace.mkdir(parents=True, exist_ok=False)
    for name in ("inputs", "assets", "requests", "out", ".evaluation"):
        (workspace / name).mkdir()
    shutil.copy2(binary, workspace / ".evaluation" / "agent-epub")
    shutil.copy2(HERE / "eval_runtime.py", workspace / ".evaluation" / "eval_runtime.py")
    (workspace / "epub").write_text(f"#!{sys.executable}\nimport runpy\nfrom pathlib import Path\nrunpy.run_path(str(Path(__file__).parent / '.evaluation' / 'eval_runtime.py'), run_name='__main__')\n")
    (workspace / "epub").chmod(0o755)
    for name in ("cover.png", "cover.jpg"):
        shutil.copy2(covers / name, workspace / "assets" / name)
    shutil.copyfile(covers / "cover.jpg", workspace / "assets" / "misnamed.data")
    shutil.copyfile(covers / "cover.png", workspace / "assets" / "cover with spaces.png")
    cases = []
    for i in range(1, 21):
        identifier = f"{i:02}"
        marker = f"PRESERVE-{identifier}-CONTENT"
        mode = "direct" if i <= 6 else "request_file" if i <= 12 else "stream_request"
        markdown = i not in (1, 6, 13, 18)
        name = f"source-{identifier}.{'md' if markdown else 'txt'}"
        text = f"# First\n\n{marker}. **Strong** & café.\n\n# Second\n\n[Return](#first)." if markdown else f"{marker}. Literal <tag> & **markers**.\nSecond line."
        chapters = 2 if markdown else 1
        if i == 2:
            text = f"Preamble {marker}.\n\n# First\n\nOne.\n\n# Second\n\nTwo."
            chapters = 3
        if i == 3:
            text = f"# Repeat\n\n{marker}.\n\n# Repeat\n\n[Back](#repeat)."
        if i == 4:
            text = f"# Café 日本語\n\n{marker}. naïef Ελληνικά.\n\n# End\n\n[Back](#café-日本語)."
        if i == 16:
            text = f"# Code\n\n{marker}.\n\n```text\n<literal> & code\n```"
            chapters = 1
        (workspace / "inputs" / name).write_text(text)
        inputs = [{"path": f"inputs/{name}"}]
        if i >= 13:
            inputs = [{"name": name, "format": "markdown" if markdown else "text", "content": text}]
        if i == 15:
            inputs = [{"name": "one.md", "format": "markdown", "content": f"# One\n\n{marker}. [next](two.md#two)."}, {"name": "two.md", "format": "markdown", "content": "# Two\n\n[back](one.md#one)."}]
            chapters = 2
        if i == 5:
            (workspace / "inputs" / "extra.txt").write_text("EXTRA-SOURCE-MARKER")
            inputs.append({"path": "inputs/extra.txt"})
            chapters += 1
        cover = None
        if i in (6, 8, 10, 11, 12, 14, 17, 19, 20):
            cover = {"path": "assets/" + ("cover.png" if i % 2 == 0 else "cover.jpg"), "alt": f"Cover of evaluation {identifier}"}
        if i == 11:
            cover["path"] = "assets/misnamed.data"
        if i == 12:
            cover["path"] = "assets/cover with spaces.png"
        case = {"id": identifier, "mode": mode, "output": f"out/{identifier}.epub", "metadata": {"title": f"Evaluation {identifier} & café", "language": "nl" if i == 7 else "en-US" if i == 9 else "en"}, "inputs": inputs, "cover": cover, "overwrite": i == 18, "instruction": "Preserve the supplied content and input order. Use built-in checks and JSON results."}
        if i == 9:
            case["metadata"]["author"] = "O'Writer & Team"
        if i == 18:
            (workspace / case["output"]).write_text("Previous edition; replace this as requested.")
        cases.append(case)
        case["expected"] = {"chapters": chapters, "markers": [marker] + (["EXTRA-SOURCE-MARKER"] if i == 5 else [])}
    (workspace / ".evaluation" / "groundtruth.json").write_text(json.dumps(cases, indent=2))
    public = [{k: v for k, v in case.items() if k != "expected"} for case in cases]
    (workspace / "cases.json").write_text(json.dumps(public, ensure_ascii=False, indent=2))
    fixed = [p for folder in ("inputs", "assets") for p in (workspace / folder).iterdir()] + [workspace / "cases.json", workspace / "epub", workspace / ".evaluation" / "agent-epub", workspace / ".evaluation" / "eval_runtime.py", workspace / ".evaluation" / "groundtruth.json"]
    (workspace / ".evaluation" / "fixed_hashes.json").write_text(json.dumps({str(p.relative_to(workspace)): hashlib.sha256(p.read_bytes()).hexdigest() for p in fixed}, indent=2))


def prompt():
    return """Evaluate the local EPUB CLI as its user. Work only in this directory. First run ./epub capabilities, then read cases.json and complete every case. Do not read or modify .evaluation, ./epub, inputs, assets or cases.json; the evaluator scores actual CLI calls and artifacts. Do not read the product implementation, call external services, install tools or delegate to other agents. You may write JSON request files under requests/ and artifacts under out/.
Each case specifies the desired metadata, ordered inputs, optional cover, output and input mode. direct means use direct input flags; request_file means write a version-1 JSON book request then use --request; stream_request means send that request through stdin with --request -. The tool's capabilities describe the schema. Omit an absent optional cover rather than passing null. Resolve relative file references correctly. For stream requests containing files, pass --base-dir .; for request files in requests/, use --base-dir . too. Request metadata belongs inside the request, not conflicting flags. Use --json for every build and validate. Use built-in checks; the evaluator runs EPUBCheck separately. Respect each case's overwrite choice.
Attempt every case once before correcting any failure. Then repair failed invocations if necessary, without changing supplied text or fixtures. Check the results and validate completed books. Do not claim success without actual tool output. End with a concise count of successful cases and any remaining failures. In a tool-call runtime without file tools, write_request creates request JSON under requests/."""


def native(provider, executable, workspace, timeout):
    if provider == "openai":
        arguments = [executable, "exec", "--ephemeral", "--ignore-user-config", "--skip-git-repo-check", "--sandbox", "workspace-write", "--json", "-C", str(workspace), "-o", str(workspace / ".evaluation" / "final.txt"), "-"]
    else:
        arguments = [executable, "--print", "--safe-mode", "--strict-mcp-config", "--mcp-config", '{"mcpServers":{}}', "--no-session-persistence", "--permission-mode", "dontAsk", "--tools", "Read,Write,Bash", "--allowedTools", "Read,Write,Bash(./epub *)", "--effort", "low", "--max-budget-usd", "3", "--output-format", "json"]
    started = time.monotonic()
    process = subprocess.Popen(arguments, cwd=workspace, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, start_new_session=True)
    try:
        stdout, stderr = process.communicate(prompt(), timeout=timeout)
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGTERM)
        stdout, stderr = process.communicate(timeout=15)
    (workspace / ".evaluation" / "agent.stdout").write_text(stdout)
    (workspace / ".evaluation" / "agent.stderr").write_text(stderr)
    return {"provider": provider, "client_version": subprocess.check_output([executable, "--version"], text=True, stderr=subprocess.DEVNULL).strip(), "return_code": process.returncode, "elapsed_seconds": round(time.monotonic() - started, 3), "model": "CLI default; effective model identity was not exposed in recorded client events"}


def check_book(path, case, workspace, epubcheck):
    result = subprocess.run([str(workspace / ".evaluation" / "agent-epub"), "validate", str(path), "--json", "--epubcheck", str(epubcheck)], capture_output=True, text=True, timeout=60)
    if result.returncode:
        return False, "independent_validation_failed"
    with zipfile.ZipFile(path) as z:
        package = ET.fromstring(z.read("EPUB/package.opf"))
        metadata = package.find(OPF + "metadata")
        for field, name in (("title", "title"), ("language", "language"), ("author", "creator")):
            if field in case["metadata"] and metadata.findtext(DC + name) != case["metadata"][field]:
                return False, "incorrect_metadata"
        summary = json.loads(result.stdout)
        if summary["chapters"] != case["expected"]["chapters"] or summary["cover"] != bool(case["cover"]):
            return False, "incorrect_chapters_or_cover"
        text = " ".join(" ".join(ET.fromstring(z.read(n)).itertext()) for n in z.namelist() if n.startswith("EPUB/chapter-") and n.endswith(".xhtml"))
        if any(text.count(marker) != 1 for marker in case["expected"]["markers"]):
            return False, "content_not_preserved"
        if case["cover"]:
            item = next(i for i in package.find(OPF + "manifest") if "cover-image" in i.get("properties", "").split())
            if z.read("EPUB/" + item.get("href")) != (workspace / case["cover"]["path"]).read_bytes():
                return False, "cover_bytes_changed"
            cover_page = ET.fromstring(z.read("EPUB/cover.xhtml"))
            image = next(cover_page.iter(XHTML + "img"))
            if image.get("alt") != case["cover"]["alt"]:
                return False, "cover_alternative_text_changed"
    return True, "passed"


def score(workspace, epubcheck, run):
    if run.get("model", "").startswith("client default;"):
        run["model"] = "CLI default; effective model identity was not exposed in recorded client events"
    fixed = json.loads((workspace / ".evaluation" / "fixed_hashes.json").read_text())
    altered = [name for name, digest in fixed.items() if not (workspace / name).is_file() or hashlib.sha256((workspace / name).read_bytes()).hexdigest() != digest]
    cases = json.loads((workspace / ".evaluation" / "groundtruth.json").read_text())
    log = workspace / ".evaluation" / "calls.jsonl"
    calls = [json.loads(s) for s in log.read_text().splitlines()] if log.exists() else []
    results = []
    for case in cases:
        attempts = [c for c in calls if c["arguments"][0] == "build" and c["output_name"] == Path(case["output"]).name]
        first = False
        reason = "not_attempted"
        if attempts:
            attempt = attempts[0]
            reason = "first_request_failed"
            if attempt["exit_code"] == 0 and "snapshot" in attempt:
                first, reason = check_book(workspace / attempt["snapshot"], case, workspace, epubcheck)
                args = attempt["arguments"]
                request = args[args.index("--request") + 1] if "--request" in args else next((a.split("=", 1)[1] for a in args if a.startswith("--request=")), None)
                mode = "direct" if request is None else "stream_request" if request == "-" else "request_file"
                if mode != case["mode"]:
                    first, reason = False, "incorrect_input_mode"
                try:
                    structured = json.loads(attempt["stdout"])
                    if structured.get("status") != "success":
                        first, reason = False, "missing_structured_success"
                except ValueError:
                    first, reason = False, "non_json_result"
        final = False
        if (workspace / case["output"]).is_file():
            try:
                final, _ = check_book(workspace / case["output"], case, workspace, epubcheck)
            except (ValueError, zipfile.BadZipFile, ET.ParseError):
                pass
        results.append({"case": case["id"], "mode": case["mode"], "cover": bool(case["cover"]), "first_request_passed": first, "final_passed": final, "attempts": len(attempts), "first_result": reason})
    first = sum(c["first_request_passed"] for c in results)
    return {"date": time.strftime("%Y-%m-%d"), "run": run, "tool_version": subprocess.check_output([str(workspace / ".evaluation" / "agent-epub"), "--version"], text=True).strip(), "tool_binary_sha256": hashlib.sha256((workspace / ".evaluation" / "agent-epub").read_bytes()).hexdigest(), "provider_transcript_sha256": hashlib.sha256((workspace / ".evaluation" / "agent.stdout").read_bytes()).hexdigest(), "corpus_version": 1, "cases": len(cases), "first_request_passed": first, "first_request_percent": first / len(cases) * 100, "final_passed": sum(c["final_passed"] for c in results), "immutable_fixture_changes": altered, "capabilities_called": any(c["arguments"][0] == "capabilities" for c in calls), "target_met": first >= 19 and all(c["final_passed"] for c in results) and not altered, "scope": "20 synthetic tasks in one native-agent session. Actual instrumented CLI calls and first-output snapshots are scored independently. This is a bounded usability evaluation, not a population-wide reliability estimate or a test of every client UI.", "results": results}


def main():
    parser = argparse.ArgumentParser()
    sub = parser.add_subparsers(dest="command", required=True)
    p = sub.add_parser("prepare")
    p.add_argument("--binary", type=Path, required=True)
    p.add_argument("--covers", type=Path, required=True)
    p.add_argument("--workspace", type=Path, required=True)
    p = sub.add_parser("run")
    p.add_argument("--provider", choices=["openai", "claude"], required=True)
    p.add_argument("--executable", required=True)
    p.add_argument("--workspace", type=Path, required=True)
    p.add_argument("--timeout", type=int, default=900)
    p = sub.add_parser("score")
    p.add_argument("--workspace", type=Path, required=True)
    p.add_argument("--epubcheck", type=Path, required=True)
    p.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    workspace = args.workspace.resolve()
    if args.command == "prepare":
        prepare(args.binary.resolve(), workspace, args.covers.resolve())
    elif args.command == "run":
        run = native(args.provider, args.executable, workspace, args.timeout)
        (workspace / ".evaluation" / "run.json").write_text(json.dumps(run, indent=2))
        print(json.dumps(run))
    else:
        report = score(workspace, args.epubcheck.resolve(), json.loads((workspace / ".evaluation" / "run.json").read_text()))
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps({k: v for k, v in report.items() if k != "results"}))


if __name__ == "__main__":
    main()
