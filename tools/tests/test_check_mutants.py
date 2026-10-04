import contextlib
import hashlib
import importlib.util
import io
import json
import tempfile
import unittest
from pathlib import Path

TOOLS_DIR = Path(__file__).resolve().parent.parent
GRADER_PATH = TOOLS_DIR / "check_mutants.py"

EQUIVALENTS = [
    (
        "tuplities/src/nested_into_vec.rs",
        "<impl NestedTupleIntoVec<T> for ()>::into_vec",
        "-> Vec<T>",
        "vec![]",
    ),
    (
        "tuplities/src/nested_option.rs",
        "<impl NestedTupleOption for ()>::transpose",
        "-> Option<Self::Transposed>",
        "Some(Default::default())",
    ),
    (
        "tuplities/src/nested_option.rs",
        "<impl NestedTupleOptionWith<H> for ()>::transpose_or",
        "-> Result<Self::Transposed, H>",
        "Ok(Default::default())",
    ),
    (
        "tuplities/src/nested_option_try_from.rs",
        "<impl NestedTupleOptionTryFrom<(), E> for ()>::nested_tuple_option_try_from",
        "-> Result<Self, E>",
        "Ok(Default::default())",
    ),
    (
        "tuplities/src/nested_try_from.rs",
        "<impl NestedTupleTryFrom<(), E> for ()>::nested_tuple_try_from",
        "-> Result<Self, E>",
        "Ok(Default::default())",
    ),
]

FRESH_KEYS = [
    (
        "tuplities/src/nested_into_vec.rs",
        "<impl NestedTupleIntoVec<T> for (T,)>::into_vec",
        "-> Vec<T>",
        "vec![]",
    ),
    (
        "tuplities/src/nested_try_from.rs",
        "<impl NestedTupleTryFrom<(OtherHead,), E> for (Head,)>::nested_tuple_try_from",
        "-> Result<Self, E>",
        "Ok(Default::default())",
    ),
]

UNREVIEWED_KEY = (
    "tuplities/src/nested_option.rs",
    "<impl NestedTupleOption for (Option<T>,)>::transpose",
    "-> Option<Self::Transposed>",
    "Some(Default::default())",
)

SUMMARY_FIELDS = {
    "CaughtMutant": "caught",
    "MissedMutant": "missed",
    "Timeout": "timeout",
    "Unviable": "unviable",
}

SPAN = {"start": {"line": 1, "column": 1}, "end": {"line": 2, "column": 2}}


def _load_grader():
    spec = importlib.util.spec_from_file_location("check_mutants", GRADER_PATH)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _baseline(summary="Success", phases=None):
    if phases is None:
        phases = [
            {"phase": "Build", "duration": 1.0, "process_status": "Success", "argv": ["cargo", "test", "--no-run"]},
            {"phase": "Test", "duration": 1.0, "process_status": "Success", "argv": ["cargo", "test"]},
        ]
    return {
        "scenario": "Baseline",
        "summary": summary,
        "log_path": "log/baseline.log",
        "diff_path": None,
        "phase_results": phases,
    }


def _mutant(file, function_name, return_type, replacement, summary, line=1):
    name = f"{file}:{line}:1: replace {function_name} {return_type} with {replacement}"
    phases = _baseline()["phase_results"]
    if summary == "Unviable":
        phases = [{**phases[0], "process_status": {"Failure": 101}}]
    elif summary == "CaughtMutant":
        phases[1]["process_status"] = {"Failure": 101}
    return {
        "scenario": {
            "Mutant": {
                "name": name,
                "package": "tuplities",
                "file": file,
                "function": {"function_name": function_name, "return_type": return_type, "span": SPAN},
                "span": SPAN,
                "replacement": replacement,
                "genre": "FnValue",
            }
        },
        "summary": summary,
        "log_path": f"log/mutant-{line}.log",
        "diff_path": None,
        "phase_results": phases,
    }


def _report(outcomes):
    totals = {field: 0 for field in SUMMARY_FIELDS.values()}
    for outcome in outcomes:
        if outcome["scenario"] == "Baseline":
            continue
        field = SUMMARY_FIELDS.get(outcome["summary"])
        if field is not None:
            totals[field] += 1
    return {
        "outcomes": outcomes,
        "total_mutants": sum(totals.values()),
        **totals,
        "success": 0,
        "start_time": "2026-10-03T23:17:21.057444822Z",
        "end_time": "2026-10-03T23:21:03.596964341Z",
        "cargo_mutants_version": "27.1.0",
    }


def _make_repo(root):
    """Create the source files for the reviewed records and a fresh review file."""
    for file, _, _, _ in EQUIVALENTS:
        path = root / file
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text("fn fixture() -> u8 { 1 }\n", encoding="utf-8")
    records = []
    for file, function_name, return_type, replacement in EQUIVALENTS:
        records.append(
            {
                "file": file,
                "function": f"{function_name} {return_type}",
                "replacement": replacement,
                "sha256": hashlib.sha256((root / file).read_bytes()).hexdigest(),
                "reason": "the replacement builds the same value the unit-tuple impl returns",
            }
        )
    reviewed = root / "reviewed.json"
    reviewed.write_text(json.dumps({"records": records}, indent=2), encoding="utf-8")
    return reviewed


class GraderTests(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.root = Path(self._tmp.name)
        self.reviewed = _make_repo(self.root)
        self.raw = self.root / "outcomes.json"
        self.grader = _load_grader()

    def write_outcomes(self, outcomes):
        self.raw.write_text(json.dumps(_report(outcomes)), encoding="utf-8")

    def _run(self, exit_code=2):
        argv = [str(self.raw), "--exit-code", str(exit_code), "--reviewed", str(self.reviewed), "--repo-root", str(self.root)]
        buffer = io.StringIO()
        with contextlib.redirect_stdout(buffer):
            code = self.grader.main(argv)
        return code, buffer.getvalue()

    def _accepted_outcomes(self):
        outcomes = [_baseline()]
        for line, key in enumerate(EQUIVALENTS, start=1):
            outcomes.append(_mutant(*key, "MissedMutant", line))
        for line, key in enumerate(FRESH_KEYS, start=10):
            outcomes.append(_mutant(*key, "CaughtMutant", line))
        for line, key in enumerate(FRESH_KEYS, start=20):
            outcomes.append(_mutant(*key, "Unviable", line))
        return outcomes

    def test_known_equivalents_accepted(self):
        self.write_outcomes(self._accepted_outcomes())
        code, out = self._run()
        self.assertEqual(0, code)

    def test_fresh_missed_rejected(self):
        outcomes = self._accepted_outcomes()
        outcomes.append(_mutant(*UNREVIEWED_KEY, "MissedMutant", 30))
        self.write_outcomes(outcomes)
        code, out = self._run()
        self.assertNotEqual(0, code)
        self.assertIn(UNREVIEWED_KEY[1], out)

    def test_changed_source_invalidates_acceptance(self):
        self.write_outcomes(self._accepted_outcomes())
        path = self.root / "tuplities" / "src" / "nested_option.rs"
        path.write_text(path.read_text(encoding="utf-8") + "// touched\n", encoding="utf-8")
        code, out = self._run()
        self.assertNotEqual(0, code)

    def test_timeout_rejected(self):
        outcomes = [_baseline(), _mutant(*FRESH_KEYS[0], "Timeout", 1), _mutant(*FRESH_KEYS[1], "CaughtMutant", 2)]
        self.write_outcomes(outcomes)
        code, out = self._run(exit_code=2)
        self.assertNotEqual(0, code)

    def test_baseline_failure_rejected(self):
        failed = _baseline(
            summary="Failure",
            phases=[{"phase": "Build", "duration": 1.0, "process_status": "Failure", "argv": ["cargo", "test", "--no-run"]}],
        )
        self.write_outcomes([failed, _mutant(*FRESH_KEYS[0], "CaughtMutant", 1)])
        code, out = self._run()
        self.assertNotEqual(0, code)

    def test_tool_exit_code_rejected(self):
        self.write_outcomes(self._accepted_outcomes())
        code, out = self._run(exit_code=1)
        self.assertNotEqual(0, code)

    def test_corrupt_report_rejected(self):
        self.raw.write_text("{ not json", encoding="utf-8")
        code, out = self._run()
        self.assertNotEqual(0, code)

    def test_incomplete_report_rejected(self):
        self.write_outcomes(self._accepted_outcomes())
        data = json.loads(self.raw.read_text(encoding="utf-8"))
        data["missed"] = data["missed"] - 1
        self.raw.write_text(json.dumps(data), encoding="utf-8")
        code, out = self._run()
        self.assertNotEqual(0, code)

    def test_missing_baseline_rejected(self):
        self.write_outcomes(self._accepted_outcomes())
        data = json.loads(self.raw.read_text(encoding="utf-8"))
        data["outcomes"] = data["outcomes"][1:]
        self.raw.write_text(json.dumps(data), encoding="utf-8")
        code, out = self._run()
        self.assertNotEqual(0, code)

    def test_duplicate_reviewed_records_rejected(self):
        self.write_outcomes(self._accepted_outcomes())
        data = json.loads(self.reviewed.read_text(encoding="utf-8"))
        data["records"].append(dict(data["records"][0]))
        self.reviewed.write_text(json.dumps(data), encoding="utf-8")
        code, out = self._run()
        self.assertNotEqual(0, code)

    def test_duplicate_outcome_rejected(self):
        outcomes = self._accepted_outcomes()
        duplicate = _mutant(*FRESH_KEYS[0], "CaughtMutant", 40)
        outcomes.append(duplicate)
        outcomes.append(duplicate)
        self.write_outcomes(outcomes)
        code, out = self._run()
        self.assertNotEqual(0, code)

    def test_known_equivalent_absent_ok(self):
        outcomes = [_baseline()]
        for line, key in enumerate(FRESH_KEYS, start=1):
            outcomes.append(_mutant(*key, "CaughtMutant", line))
        for line, key in enumerate(FRESH_KEYS, start=10):
            outcomes.append(_mutant(*key, "Unviable", line))
        self.write_outcomes(outcomes)
        code, out = self._run(exit_code=0)
        self.assertEqual(0, code)

    def test_stale_record_without_survivor_rejected(self):
        outcomes = [_baseline()]
        for line, key in enumerate(FRESH_KEYS, start=1):
            outcomes.append(_mutant(*key, "CaughtMutant", line))
        for line, key in enumerate(FRESH_KEYS, start=10):
            outcomes.append(_mutant(*key, "Unviable", line))
        self.write_outcomes(outcomes)
        path = self.root / "tuplities" / "src" / "nested_option.rs"
        path.write_text(path.read_text(encoding="utf-8") + "// touched\n", encoding="utf-8")
        code, out = self._run(exit_code=0)
        self.assertNotEqual(0, code)

    def test_malformed_reviewed_record_rejected(self):
        self.write_outcomes(self._accepted_outcomes())
        data = json.loads(self.reviewed.read_text(encoding="utf-8"))
        data["records"][0]["reason"] = ""
        data["records"][1]["sha256"] = "zzz"
        self.reviewed.write_text(json.dumps(data), encoding="utf-8")
        code, out = self._run()
        self.assertNotEqual(0, code)

    def test_unrecognized_summary_rejected(self):
        outcomes = [_baseline()]
        for line, key in enumerate(FRESH_KEYS, start=1):
            outcomes.append(_mutant(*key, "CaughtMutant", line))
        outcomes.append(_mutant(*UNREVIEWED_KEY, "Bogus", 30))
        self.write_outcomes(outcomes)
        code, out = self._run()
        self.assertNotEqual(0, code)

    def test_baseline_without_test_phase_rejected(self):
        outcomes = self._accepted_outcomes()
        outcomes[0]["phase_results"] = outcomes[0]["phase_results"][:1]
        self.write_outcomes(outcomes)
        code, _ = self._run()
        self.assertNotEqual(0, code)

    def test_mutant_without_completed_phases_rejected(self):
        outcomes = self._accepted_outcomes()
        del outcomes[1]["phase_results"]
        self.write_outcomes(outcomes)
        code, _ = self._run()
        self.assertNotEqual(0, code)

    def test_exit_code_disagrees_with_survivors_rejected(self):
        self.write_outcomes(self._accepted_outcomes())
        code, _ = self._run(exit_code=0)
        self.assertNotEqual(0, code)

    def test_caught_nonfunction_and_deletion_mutants_accepted(self):
        operator = _mutant(*FRESH_KEYS[0], "CaughtMutant")
        operator["scenario"]["Mutant"]["function"] = None
        operator["scenario"]["Mutant"]["genre"] = "BinaryOperator"
        operator["scenario"]["Mutant"]["replacement"] = "-"
        deletion = _mutant(*FRESH_KEYS[1], "CaughtMutant")
        deletion["scenario"]["Mutant"]["genre"] = "MatchArm"
        deletion["scenario"]["Mutant"]["replacement"] = ""
        self.write_outcomes([_baseline(), operator, deletion])
        code, _ = self._run(exit_code=0)
        self.assertEqual(0, code)


if __name__ == "__main__":
    unittest.main()
