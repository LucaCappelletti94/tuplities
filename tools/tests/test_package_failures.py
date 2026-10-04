import json
import os
import subprocess
import tempfile
import unittest
from pathlib import Path


SCRIPT = Path(__file__).resolve().parents[1] / "verify_package.sh"


class PackageFailureTests(unittest.TestCase):
    def test_missing_source_records_failed_step(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / "missing-source"
            diagnostics = root / "diagnostics"
            result = subprocess.run(
                ["sh", str(SCRIPT), "--allow-dirty"],
                env={
                    **os.environ,
                    "TUPLITIES_PACKAGE_DIR": str(source),
                    "TUPLITIES_VERIFICATION_DIR": str(diagnostics),
                },
                capture_output=True,
                text=True,
                timeout=15,
            )
            self.assertEqual(result.returncode, 1)
            summary_path, = diagnostics.glob("run-*/summary.json")
            summary = json.loads(summary_path.read_text())
            self.assertIsNotNone(summary["failure"])
            self.assertEqual(
                [(step["step"], step["status"]) for step in summary["steps"]],
                [("checkout-manifest", "failed")],
            )


if __name__ == "__main__":
    unittest.main()
