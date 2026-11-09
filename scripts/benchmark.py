#!/usr/bin/env python3
"""Development benchmark; the EPUB generator itself has no Python dependency."""
import argparse
import json
import math
import os
from pathlib import Path
import platform
import statistics
import subprocess
import tempfile
import time

parser = argparse.ArgumentParser()
parser.add_argument("binary", type=Path)
parser.add_argument("--runs", type=int, default=30)
parser.add_argument("--cover", type=Path)
args = parser.parse_args()
if args.runs < 1:
    parser.error("--runs must be positive")
binary = args.binary.resolve()
with tempfile.TemporaryDirectory(prefix="agent-epub-benchmark-") as tmp:
    source = Path(tmp) / "book.md"
    source.write_text("\n\n".join(f"# Chapter {i + 1}\n\n" + "reading " * 2500 for i in range(40)))
    durations = []
    for _ in range(args.runs):
        started = time.perf_counter()
        command = [str(binary), "build", "--input", str(source), "--output", str(Path(tmp) / "book.epub"), "--title", "Benchmark", "--language", "en", "--overwrite"]
        if args.cover:
            command.extend(["--cover", str(args.cover.resolve())])
        subprocess.run(command, check=True, capture_output=True)
        durations.append((time.perf_counter() - started) * 1000)
    print(json.dumps({"date": time.strftime("%Y-%m-%d"), "platform": platform.platform(), "architecture": platform.machine(), "logical_cpus": os.cpu_count(), "binary_version": subprocess.check_output([str(binary), "--version"], text=True).strip(), "body_words": 100000, "chapters": 40, "source_bytes": source.stat().st_size, "cover_bytes": args.cover.stat().st_size if args.cover else 0, "runs": args.runs, "scope": "Process startup, source reading, cover decoding when supplied, generation, built-in checks and output publication. Excludes external EPUBCheck, source creation and transfers. Warm filesystem caches; local measurement, not a general guarantee.", "median_ms": round(statistics.median(durations), 3), "p95_ms": round(sorted(durations)[math.ceil(0.95 * len(durations)) - 1], 3), "max_ms": round(max(durations), 3), "samples_ms": [round(s, 3) for s in durations]}, indent=2))
