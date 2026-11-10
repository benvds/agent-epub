#!/usr/bin/env python3
"""Reproduce the descriptive summary from retained measured sessions."""
import argparse
import json
from pathlib import Path
import statistics


def stats(values):
    return {'n': len(values), 'median': round(statistics.median(values), 6),
            'minimum': min(values), 'maximum': max(values)} if values else {'n': 0}


def summarize(folder):
    runs = [json.loads(p.read_text()) for p in sorted(folder.glob('pair-*/result.json'))]
    summary = {'runs': len(runs), 'tools': {}, 'paired_differences_seconds_agent_minus_pandoc': {}}
    for tool in ['agent', 'pandoc']:
        selected = [r for r in runs if r['tool'] == tool]
        record = {'runs': len(selected), 'cases': {},
                  'shell_calls': stats([r['shell_tool_calls'] for r in selected]),
                  'runtime_calls': stats([r['runtime_calls'] for r in selected]),
                  'download_seconds': stats([r['download']['seconds'] for r in selected]),
                  'download_bytes': sorted({r['download']['bytes'] for r in selected}),
                  'usage': {key: sum(r['usage'][key] for r in selected) for key in ['input_tokens', 'cached_input_tokens', 'output_tokens', 'reasoning_output_tokens']},
                  'manual_interventions': sum(r['manual_interventions'] for r in selected),
                  'timeouts': sum(r['timed_out'] for r in selected),
                  'failed_sessions': sum(r['return_code'] != 0 for r in selected)}
        record['usage']['uncached_input_tokens'] = record['usage']['input_tokens'] - record['usage']['cached_input_tokens']
        for case in ['report', 'guide', 'inline', 'recovery', 'image']:
            samples = [r['cases'][case] for r in selected]
            record['cases'][case] = {
                'outcomes': {outcome: sum(c['outcome'] == outcome for c in samples) for outcome in ['correct', 'incorrect', 'unsupported', 'failed']},
                'seconds': stats([c['segment_seconds'] for c in samples if c['outcome'] == 'correct' and c['segment_seconds'] is not None]),
                'attempts': sum(c['attempts'] for c in samples), 'retries': sum(c['retries'] for c in samples),
                'first_attempt_correct': sum(c['first_attempt_correct'] for c in samples),
                'converter_seconds': stats([c['converter_seconds'] for c in samples]),
                'validation_seconds': stats([c['validation_seconds'] for c in samples]),
            }
        clean = [r['cases'][case] for r in selected for case in ['report', 'guide', 'inline']]
        record['clean_exports'] = {'first_attempt_correct': sum(c['first_attempt_correct'] for c in clean), 'attempted': len(clean)}
        breakdown = []
        for run in selected:
            path = folder / f'pair-{run["pair"]}-{tool}' / 'calls.jsonl'
            calls = [json.loads(line) for line in path.read_text().splitlines()]
            stop = run['cases']['recovery']['completion_seconds']
            if stop is None:
                continue
            calls = [c for c in calls if c['end_seconds'] <= stop]
            install = sum(c['end_seconds'] - c['start_seconds'] for c in calls if c['action'] == 'install')
            discovery = sum(c['converter_seconds'] for c in calls if c['action'] == 'discover')
            converter = sum(c.get('converter_seconds', 0) for c in calls if c['action'] not in ['install', 'discover'])
            validation = sum(c.get('independent', {}).get('validation_seconds', 0) for c in calls)
            runtime = sum(c['end_seconds'] - c['start_seconds'] for c in calls)
            breakdown.append({'download': run['download']['seconds'], 'install': install, 'discovery': discovery,
                              'conversion': converter, 'validation': validation,
                              'model_client_shell_and_other': stop - run['download']['seconds'] - runtime,
                              'supported_workflow_total': stop})
        record['supported_workflow_breakdown_seconds'] = {key: stats([row[key] for row in breakdown]) for key in breakdown[0]} if breakdown else {}
        summary['tools'][tool] = record
    for case in ['report', 'guide', 'inline', 'recovery']:
        paired = []
        for pair in [1, 2, 3]:
            a = next((r['cases'][case] for r in runs if r['pair'] == pair and r['tool'] == 'agent'), None)
            b = next((r['cases'][case] for r in runs if r['pair'] == pair and r['tool'] == 'pandoc'), None)
            if a and b and a['outcome'] == b['outcome'] == 'correct':
                paired.append(round(a['segment_seconds'] - b['segment_seconds'], 6))
        summary['paired_differences_seconds_agent_minus_pandoc'][case] = dict(stats(paired), samples=paired)
    totals = []
    for pair in [1, 2, 3]:
        a = next((r['cases']['recovery']['completion_seconds'] for r in runs if r['pair'] == pair and r['tool'] == 'agent'), None)
        b = next((r['cases']['recovery']['completion_seconds'] for r in runs if r['pair'] == pair and r['tool'] == 'pandoc'), None)
        if a is not None and b is not None:
            totals.append(round(a - b, 6))
    summary['paired_differences_seconds_agent_minus_pandoc']['supported_workflow_total'] = dict(stats(totals), samples=totals)
    return summary


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('runs', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    result = summarize(args.runs)
    args.output.write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result, indent=2))
