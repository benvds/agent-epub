"""Instrumented CLI wrapper for synthetic agent evaluations; not a product dependency."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import time


def run(arguments, stdin="", root=None):
    root = Path(root or Path.cwd()).resolve()
    if not arguments or arguments[0] not in ("build", "validate", "capabilities", "--help"):
        return 2, '{"status":"failed","errors":[{"message":"Unsupported evaluation command"}]}', ""
    if "--epubcheck" in arguments:
        return 2, '{"status":"failed","errors":[{"message":"Use built-in checks; the evaluator runs independent validation separately"}]}', ""
    started = time.monotonic()
    result = subprocess.run([str(root / ".evaluation" / "agent-epub"), *arguments], cwd=root, input=stdin, text=True, capture_output=True, timeout=60)
    output = None
    for flag in ("--output", "-o"):
        if flag in arguments:
            output = arguments[arguments.index(flag) + 1]
    if output is None:
        output = next((a.split("=", 1)[1] for a in arguments if a.startswith("--output=")), None)
    record = {"arguments": arguments, "stdin": stdin, "exit_code": result.returncode, "stdout": result.stdout, "stderr": result.stderr, "elapsed_ms": round((time.monotonic() - started) * 1000, 3), "output_name": Path(output).name if output else None}
    if output and result.returncode == 0:
        artifact = root / output
        if artifact.is_file():
            attempts = root / ".evaluation" / "attempts"
            attempts.mkdir(exist_ok=True)
            snapshot = attempts / f"{time.time_ns()}-{artifact.name}"
            shutil.copyfile(artifact, snapshot)
            record["snapshot"] = str(snapshot.relative_to(root))
            record["artifact_sha256"] = hashlib.sha256(snapshot.read_bytes()).hexdigest()
    with (root / ".evaluation" / "calls.jsonl").open("a") as stream:
        stream.write(json.dumps(record) + "\n")
    return result.returncode, result.stdout, result.stderr


if __name__ == "__main__":
    arguments = sys.argv[1:]
    streamed = "--request" in arguments and arguments[arguments.index("--request") + 1] == "-"
    streamed = streamed or "--request=-" in arguments
    code, stdout, stderr = run(arguments, sys.stdin.read() if streamed else "")
    sys.stdout.write(stdout)
    sys.stderr.write(stderr)
    raise SystemExit(code)
