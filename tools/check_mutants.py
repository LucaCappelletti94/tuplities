"""Grade complete mutation runs against source-bound reviewed equivalents."""

import argparse
import hashlib
import json
import sys
from pathlib import Path

REVIEWED_FILENAME = Path(".cargo") / "mutants-reviewed-equivalents.json"
ALLOWED_EXIT_CODES = (0, 2)
BASELINE_SCENARIO = "Baseline"
SUMMARY_FIELDS = {
    "CaughtMutant": "caught",
    "MissedMutant": "missed",
    "Timeout": "timeout",
    "Unviable": "unviable",
}
HEX_DIGITS = set("0123456789abcdef")
EXIT_OK = 0
EXIT_FAILED = 1


def _sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _is_count(value):
    return isinstance(value, int) and not isinstance(value, bool)


def _is_hash(value):
    return isinstance(value, str) and len(value) == 64 and set(value) <= HEX_DIGITS


def _function_label(mutant):
    function = mutant.get("function")
    if not isinstance(function, dict):
        return None
    name = function.get("function_name")
    if not isinstance(name, str) or not name:
        return None
    return_type = function.get("return_type")
    if isinstance(return_type, str) and return_type:
        return f"{name} {return_type}"
    return name


def _load_json(path, problems, label):
    try:
        raw = path.read_text(encoding="utf-8")
    except OSError as exc:
        problems.append(f"cannot read {label} {path}: {exc}")
        return None
    try:
        data = json.loads(raw)
    except json.JSONDecodeError as exc:
        problems.append(f"corrupt {label} {path}: invalid JSON ({exc.msg} at line {exc.lineno} column {exc.colno})")
        return None
    if not isinstance(data, dict):
        problems.append(f"corrupt {label} {path}: top level is not a JSON object")
        return None
    return data


def _grade_baseline(outcome, problems):
    if outcome.get("summary") != "Success":
        problems.append(f"baseline failed (summary {outcome.get('summary')!r}), so the run is not a normal complete run")
        return
    phases = outcome.get("phase_results")
    if not isinstance(phases, list) or not phases:
        problems.append("incomplete report: baseline has no phase results")
        return
    if [phase.get("phase") if isinstance(phase, dict) else None for phase in phases] != ["Build", "Test"]:
        problems.append("incomplete report: baseline must complete Build and Test")
    for phase in phases:
        if not isinstance(phase, dict) or phase.get("process_status") != "Success":
            name = phase.get("phase") if isinstance(phase, dict) else None
            status = phase.get("process_status") if isinstance(phase, dict) else None
            problems.append(f"baseline phase {name!r} did not succeed (status {status!r})")

def _failed_status(status):
    return (
        isinstance(status, dict)
        and len(status) == 1
        and next(iter(status)) in ("Failure", "Signalled")
        and _is_count(next(iter(status.values())))
        and next(iter(status.values())) > 0
    )


def _grade_mutant_phases(outcome, name, problems):
    summary = outcome.get("summary")
    if summary not in ("MissedMutant", "CaughtMutant", "Unviable"):
        return
    phases = outcome.get("phase_results")
    expected = ["Build"] if summary == "Unviable" else ["Build", "Test"]
    if (
        not isinstance(phases, list)
        or any(not isinstance(phase, dict) for phase in phases)
        or [phase.get("phase") for phase in phases] != expected
    ):
        problems.append(f"incomplete phase results for mutant {name}")
        return
    statuses = [phase.get("process_status") for phase in phases]
    if summary == "Unviable":
        valid = _failed_status(statuses[0])
    elif summary == "CaughtMutant":
        valid = statuses[0] == "Success" and _failed_status(statuses[1])
    else:
        valid = statuses == ["Success", "Success"]
    if not valid:
        problems.append(f"phase results disagree with {summary} for mutant {name}")


def _grade_report(report, problems):
    outcomes = report.get("outcomes")
    if not isinstance(outcomes, list):
        problems.append("incomplete report: outcomes is missing or not a list")
        return None, []
    baselines = []
    mutants = []
    seen = set()
    for index, outcome in enumerate(outcomes):
        if not isinstance(outcome, dict):
            problems.append(f"malformed outcome {index}: not a JSON object")
            continue
        if outcome.get("scenario") == BASELINE_SCENARIO:
            baselines.append(outcome)
            continue
        scenario = outcome.get("scenario")
        if not isinstance(scenario, dict) or not isinstance(scenario.get("Mutant"), dict):
            problems.append(f"malformed outcome {index}: unrecognized scenario {scenario!r}")
            continue
        mutant = scenario["Mutant"]
        name = mutant.get("name")
        if not isinstance(name, str) or not name:
            problems.append(f"malformed outcome {index}: mutant name is missing")
            continue
        file = mutant.get("file")
        label = _function_label(mutant)
        replacement = mutant.get("replacement")
        if (
            not isinstance(file, str) or not file
            or "function" not in mutant
            or (mutant["function"] is not None and label is None)
            or not isinstance(replacement, str)
        ):
            problems.append(f"malformed mutant {name}: invalid file, function or replacement")
            continue
        if name in seen:
            problems.append(f"duplicate outcome: {name}")
            continue
        seen.add(name)
        _grade_mutant_phases(outcome, name, problems)
        mutants.append((name, file, label, replacement, outcome.get("summary")))
    if len(baselines) != 1:
        problems.append(f"incomplete report: {len(baselines)} baseline outcome(s) found, expected exactly 1")
    else:
        _grade_baseline(baselines[0], problems)

    counts = {field: 0 for field in SUMMARY_FIELDS.values()}
    for name, _, _, _, summary in mutants:
        field = SUMMARY_FIELDS.get(summary)
        if field is None:
            problems.append(f"unrecognized summary {summary!r} for mutant {name}")
            continue
        counts[field] += 1

    for field in ("missed", "caught", "timeout", "unviable"):
        reported = report.get(field)
        if not _is_count(reported):
            problems.append(f"corrupt report: {field} total is missing or not an integer")
        elif reported != counts[field]:
            problems.append(f"totals disagree: {field} is {reported} but {counts[field]} mutant outcome(s) have that summary")
    total = report.get("total_mutants")
    if not _is_count(total):
        problems.append("corrupt report: total_mutants is missing or not an integer")
    else:
        if total != len(mutants):
            problems.append(f"totals disagree: total_mutants is {total} but {len(mutants)} mutant outcome(s) are present")
        if total != sum(counts.values()):
            problems.append(f"totals disagree: total_mutants is {total} but the recognized summaries sum to {sum(counts.values())}")
    if counts["timeout"]:
        problems.append(f"incomplete run: {counts['timeout']} mutant(s) timed out. Only complete runs are accepted")

    missed = [
        (name, file, label, replacement)
        for name, file, label, replacement, summary in mutants
        if summary == "MissedMutant"
    ]
    return counts, missed


def _grade_records(reviewed, repo_root, problems):
    records = reviewed.get("records")
    if not isinstance(records, list):
        problems.append("corrupt reviewed records: records is missing or not a list")
        return {}
    index = {}
    for position, record in enumerate(records):
        if not isinstance(record, dict):
            problems.append(f"malformed reviewed record {position}: not a JSON object")
            continue
        file = record.get("file")
        function = record.get("function")
        replacement = record.get("replacement")
        digest = record.get("sha256")
        reason = record.get("reason")
        if (
            not isinstance(file, str) or not file
            or not isinstance(function, str) or not function
            or not isinstance(replacement, str) or not replacement
            or not _is_hash(digest)
            or not isinstance(reason, str) or not reason.strip()
        ):
            problems.append(f"malformed reviewed record {position}: file, function, replacement, sha256 or reason is missing")
            continue
        key = (file, function, replacement)
        if key in index:
            problems.append(f"duplicate reviewed record: {file} {function} {replacement}")
            continue
        source = repo_root / file
        if not source.is_file():
            index[key] = (False, f"source file {file} is missing")
        else:
            fresh = _sha256(source) == digest
            index[key] = (fresh, "source no longer matches the reviewed hash" if not fresh else None)
    for key, (fresh, detail) in index.items():
        if not fresh:
            problems.append(f"stale equivalence record: {key[0]} {key[1]} {key[2]} ({detail})")
    return index


def _match_missed(missed, records, problems):
    accepted = []
    for name, file, label, replacement in missed:
        entry = records.get((file, label, replacement))
        if entry is None:
            problems.append(f"unreviewed missed mutant: {name}")
        elif entry[0]:
            accepted.append(name)
        else:
            problems.append(f"missed mutant covered only by a stale record: {name}")
    return accepted


def _parse_args(argv):
    parser = argparse.ArgumentParser(description="Grade a cargo-mutants run against reviewed mutant equivalents.")
    parser.add_argument("raw_outcomes", type=Path, help="cargo-mutants JSON outcomes report")
    parser.add_argument("--exit-code", type=int, required=True, metavar="N", help="cargo-mutants exit code")
    parser.add_argument(
        "--reviewed",
        type=Path,
        default=None,
        help=f"reviewed records JSON (default {REVIEWED_FILENAME} under the repo root)",
    )
    parser.add_argument(
        "--repo-root",
        type=Path,
        default=Path(__file__).resolve().parent.parent,
        help="repo root that record file paths resolve against",
    )
    return parser.parse_args(argv)


def main(argv=None):
    args = _parse_args(argv)
    repo_root = args.repo_root
    reviewed_path = repo_root / REVIEWED_FILENAME if args.reviewed is None else repo_root / args.reviewed
    problems = []
    if args.exit_code not in ALLOWED_EXIT_CODES:
        problems.append(f"cargo-mutants exit code {args.exit_code} is not a normal complete run (expected 0 or 2)")
    report = _load_json(args.raw_outcomes, problems, "mutation report")
    reviewed = _load_json(reviewed_path, problems, "reviewed records")
    records = _grade_records(reviewed, repo_root, problems) if reviewed is not None else None
    counts = None
    accepted = []
    if report is not None:
        counts, missed = _grade_report(report, problems)
        if counts is not None and args.exit_code in ALLOWED_EXIT_CODES:
            expected_exit = 2 if counts["missed"] else 0
            if args.exit_code != expected_exit:
                problems.append(f"cargo-mutants exit code {args.exit_code} disagrees with missed={counts['missed']}")
        if records is not None:
            accepted = _match_missed(missed, records, problems)
    if problems:
        print(f"mutant grading: FAILED ({len(problems)} problem(s))")
        for problem in problems:
            print(f"  problem: {problem}")
    else:
        print("mutant grading: ok")
    if counts is not None:
        total = counts["missed"] + counts["caught"] + counts["timeout"] + counts["unviable"]
        print(
            f"  total={total} caught={counts['caught']} unviable={counts['unviable']} "
            f"timeout={counts['timeout']} missed={counts['missed']}"
        )
        for name in accepted:
            print(f"  accepted: {name}")
    return EXIT_OK if not problems else EXIT_FAILED


if __name__ == "__main__":
    sys.exit(main())
