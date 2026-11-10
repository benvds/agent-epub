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
parser.add_argument("--conveniences", action="store_true", help="Include 40 tables, 40 notes and 80 section entries")
args = parser.parse_args()
if args.runs < 1:
    parser.error("--runs must be positive")
binary = args.binary.resolve()
with tempfile.TemporaryDirectory(prefix="agent-epub-benchmark-") as tmp:
    source = Path(tmp) / "book.md"
    chapters = []
    for i in range(40):
        text = f"# Chapter {i + 1}\n\n"
        if args.conveniences:
            text += "## Findings\n\n"
        text += "reading " * 2500
        if args.conveniences:
            text += f"\n\n### Results\n\n| Item | Count |\n| :--- | ---: |\n| Alpha | 12 |\n| Beta | 8 |\n\nClaim[^n{i}], repeated[^n{i}].\n\n[^n{i}]: Synthetic evidence with a [chapter link](#chapter-{i + 1}).\n"
        chapters.append(text)
    source.write_text("\n\n".join(chapters))
    durations = []
    for _ in range(args.runs):
        started = time.perf_counter()
        command = [str(binary), "build", "--input", str(source), "--output", str(Path(tmp) / "book.epub"), "--title", "Benchmark", "--language", "en", "--overwrite"]
        if args.cover:
            command.extend(["--cover", str(args.cover.resolve())])
        subprocess.run(command, check=True, capture_output=True)
        durations.append((time.perf_counter() - started) * 1000)
    print(json.dumps({"date": time.strftime("%Y-%m-%d"), "platform": platform.platform(), "architecture": platform.machine(), "logical_cpus": os.cpu_count(), "binary_version": subprocess.check_output([str(binary), "--version"], text=True).strip(), "body_words": 100000, "body_words_scope": "Base prose; titles, table cells and note text are additional.", "conveniences": {"tables": 40, "footnotes": 40, "references": 80, "section_entries": 80} if args.conveniences else None, "chapters": 40, "source_bytes": source.stat().st_size, "cover_bytes": args.cover.stat().st_size if args.cover else 0, "runs": args.runs, "scope": "Process startup, source reading, cover decoding when supplied, generation, built-in checks and output publication. Excludes external EPUBCheck, source creation and transfers. Warm filesystem caches; local measurement, not a general guarantee.", "median_ms": round(statistics.median(durations), 3), "p95_ms": round(sorted(durations)[math.ceil(0.95 * len(durations)) - 1], 3), "max_ms": round(max(durations), 3), "samples_ms": [round(s, 3) for s in durations]}, indent=2))
