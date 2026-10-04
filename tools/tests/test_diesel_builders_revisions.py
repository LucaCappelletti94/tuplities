import os
import subprocess
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
SCRIPT = REPO_ROOT / "tools" / "verify_diesel_builders.sh"

PINNED_BUILDERS_REV = "7041ce661200fcf644d44c63e15695d880737945"
PINNED_DIESEL_REV = "81f1cdb2927f6d0ca11039c16bca2ebd2c117be2"


class DieselBuildersRevisionTests(unittest.TestCase):
    def run_script(self, env_updates):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        env = dict(os.environ)
        env["CARGO"] = str(Path(directory.name) / "absent-cargo")
        env.pop("DIESEL_BUILDERS_REV", None)
        env.pop("DIESEL_REV", None)
        env.pop("TUPLITIES_PACKAGE_DIR", None)
        env.update(env_updates)
        proc = subprocess.run(
            ["bash", str(SCRIPT)],
            env=env,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=10,
        )
        return proc

    def assert_rejected(self, env_updates):
        proc = self.run_script(env_updates)
        self.assertEqual(proc.returncode, 2, proc.stderr)

    def test_builders_revision_without_diesel_revision_is_rejected(self):
        self.assert_rejected({"DIESEL_BUILDERS_REV": PINNED_BUILDERS_REV})

    def test_diesel_revision_without_builders_revision_is_rejected(self):
        self.assert_rejected({"DIESEL_REV": PINNED_DIESEL_REV})

    def test_short_revision_is_rejected(self):
        self.assert_rejected(
            {"DIESEL_BUILDERS_REV": "abc123", "DIESEL_REV": PINNED_DIESEL_REV},
        )

    def test_uppercase_revision_is_rejected(self):
        self.assert_rejected(
            {
                "DIESEL_BUILDERS_REV": PINNED_BUILDERS_REV.upper(),
                "DIESEL_REV": PINNED_DIESEL_REV,
            },
        )



if __name__ == "__main__":
    unittest.main()
