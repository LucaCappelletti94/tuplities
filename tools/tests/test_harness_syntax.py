import importlib.util
import tempfile
import unittest
from pathlib import Path


SPEC = importlib.util.spec_from_file_location(
    "verify_harness_syntax", Path(__file__).parents[1] / "verify_harness_syntax.py"
)
MODULE = importlib.util.module_from_spec(SPEC)


class HarnessSyntaxTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        SPEC.loader.exec_module(MODULE)

    def check_source(self, source, suffix=".sh"):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / ("harness" + suffix)
            path.write_text(source)
            MODULE.check(path)

    def test_valid_quoted_python_blocks(self):
        self.check_source(
            "python3 - <<'PY'\nvalue = 1\nPY\n"
            'python3 - <<"PYTHON"\nvalue = 2\nPYTHON\n'
        )

    def test_invalid_embedded_python_reports_original_line(self):
        with self.assertRaises(SyntaxError) as failure:
            self.check_source("#!/bin/sh\npython3 - <<'PY'\nif:\nPY\n")
        self.assertEqual(failure.exception.lineno, 3)

    def test_unterminated_python_block_is_rejected(self):
        with self.assertRaises(ValueError):
            self.check_source("python3 - <<'PY'\nvalue = 1\n")

    def test_invalid_python_file_is_rejected(self):
        with self.assertRaises(SyntaxError):
            self.check_source("if:\n", ".py")


if __name__ == "__main__":
    unittest.main()
