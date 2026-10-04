#!/bin/sh
set -eu

root=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)

usage() {
    echo "usage: verify_package.sh [--allow-dirty]"
}

allow_dirty=0
for arg in "$@"; do
    case "$arg" in
    --allow-dirty)
        allow_dirty=1
        ;;
    -h|--help)
        usage
        exit 0
        ;;
    *)
        echo "unknown argument: $arg" >&2
        usage >&2
        exit 2
        ;;
    esac
done

if [ "$allow_dirty" = 1 ] || [ "${TUPLITIES_ALLOW_DIRTY:-0}" = 1 ]; then
    allow_dirty=1
else
    allow_dirty=0
fi

exec python3 -B - "$root" "$allow_dirty" <<'PY'
import json
import os
import shlex
import shutil
import signal
import subprocess
import sys
import tempfile
import tarfile
import time
import tomllib
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(sys.argv[1]).resolve()
ALLOW_DIRTY = sys.argv[2] == "1"

PACKAGE_TIMEOUT = 300
META_TIMEOUT = 120
TEST_TIMEOUT = 600
VERSION_TIMEOUT = 30
WORKLOADS_TIMEOUT = 3000

BRIDGE_WIDTHS = [8, 16, 32, 48, 64, 96, 128]
DOCTEST_CASES = (
    ("default", []),
    ("no-default", ["--no-default-features"]),
)


def resolve(path):
    path = Path(path)
    return path if path.is_absolute() else (ROOT / path).resolve()


PACKAGE_DIR = resolve(os.environ.get("TUPLITIES_PACKAGE_DIR", str(ROOT / "tuplities")))
OUTPUT_ROOT = resolve(
    os.environ.get(
        "TUPLITIES_VERIFICATION_DIR",
        str(ROOT / "target" / "verification" / "package"),
    )
)
CARGO = shlex.split(os.environ.get("CARGO", "cargo"))
MSRV_TOOLCHAIN = os.environ.get("TUPLITIES_MSRV_TOOLCHAIN", "1.85.0")
MSRV_CARGO = shlex.split(os.environ.get("TUPLITIES_MSRV_CARGO", f"cargo +{MSRV_TOOLCHAIN}"))


class StepFailure(Exception):
    pass


def dependency_requirement(spec):
    if isinstance(spec, str):
        return spec
    return spec.get("version", "*")


def execute(cmd, cwd, env, timeout):
    command = [str(item) for item in cmd]
    with subprocess.Popen(command, cwd=str(cwd), env=env, stdout=subprocess.PIPE,
                          stderr=subprocess.PIPE, text=True, start_new_session=True) as process:
        try:
            out, err = process.communicate(timeout=timeout)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            out, err = process.communicate(timeout=30)
            raise subprocess.TimeoutExpired(command, timeout, output=out, stderr=err) from None
        return subprocess.CompletedProcess(command, process.returncode, out, err)


def run_logged(cmd, cwd, env, log_path, identity, timeout):
    started = datetime.now(timezone.utc).isoformat()
    header = (
        f"identity={identity}\n"
        f"command={shlex.join(map(str, cmd))}\n"
        f"started={started}\n"
        f"timeout={timeout}s\n"
        f"cwd={cwd}\n"
        "--- output ---\n"
    )
    with open(log_path, "a") as handle:
        handle.write(header)
    start = time.monotonic()
    try:
        completed = execute(cmd, cwd, env, timeout)
        output = completed.stdout + completed.stderr
        exit_code = completed.returncode
        stdout = completed.stdout
    except subprocess.TimeoutExpired as exc:
        stdout = exc.stdout if isinstance(exc.stdout, str) else ""
        output = "".join(
            part
            for part in (exc.stdout or "", exc.stderr or "")
            if isinstance(part, str)
        )
        exit_code = -1
    duration = time.monotonic() - start
    with open(log_path, "a") as handle:
        handle.write(output + "--- summary ---\n")
        handle.write(f"duration={duration:.2f}s exit={exit_code}\n")
    return exit_code, stdout


def capture(cmd, cwd, env, timeout):
    completed = execute(cmd, cwd, env, timeout)
    if completed.returncode != 0:
        raise StepFailure(f"command failed: {shlex.join(map(str, cmd))}\n{completed.stderr}")
    return completed.stdout



def checkout_facts():
    checkout_manifest = tomllib.loads((PACKAGE_DIR / "Cargo.toml").read_text())
    package = checkout_manifest["package"]
    workspace = tomllib.loads((ROOT / "Cargo.toml").read_text()).get("workspace", {})
    workspace_package = workspace.get("package", {})

    def value(field):
        raw = package.get(field)
        if isinstance(raw, dict) and raw.get("workspace"):
            return workspace_package.get(field)
        return raw

    typenum_spec = workspace.get("dependencies", {}).get("typenum")
    if typenum_spec is None:
        typenum_spec = checkout_manifest.get("dependencies", {}).get("typenum")
    if typenum_spec is None:
        raise StepFailure("typenum dependency not found in checkout manifest")
    facts = {
        "name": value("name"),
        "version": value("version"),
        "rust_version": value("rust-version"),
        "typenum_requirement": dependency_requirement(typenum_spec),
    }
    if facts["name"] != "tuplities":
        raise StepFailure(f"package name {facts['name']!r} is not 'tuplities'")
    if not facts["version"] or not facts["rust_version"]:
        raise StepFailure(f"checkout manifest missing version or rust-version: {facts}")
    return facts


def inspect_archive(archive, run_dir, facts):
    root_prefix = f"{facts['name']}-{facts['version']}"
    members = []
    manifest_text = None
    with tarfile.open(archive, "r:gz") as archive_file:
        for member in sorted(archive_file.getmembers(), key=lambda item: item.name):
            members.append(member.name)
            if member.isfile() and member.name == f"{root_prefix}/Cargo.toml":
                manifest_text = archive_file.extractfile(member).read().decode("utf-8")
    (run_dir / "archive-file-list.txt").write_text("\n".join(members) + "\n")
    for member in members:
        if not member.startswith(root_prefix + "/"):
            return f"archive member {member!r} outside root {root_prefix!r}"
        parts = Path(member).parts[1:]
        if "plans" in parts or any(part.startswith("tuplities-") for part in parts[:-1]):
            return f"private plan or satellite package path in archive {member!r}"
    manifests = [member for member in members if member.rsplit("/", 1)[-1] == "Cargo.toml"]
    if manifests != [f"{root_prefix}/Cargo.toml"]:
        return f"archive manifests {manifests} != [{root_prefix}/Cargo.toml]"
    locks = [member for member in members if member.rsplit("/", 1)[-1] == "Cargo.lock"]
    if locks != [f"{root_prefix}/Cargo.lock"]:
        return f"archive locks {locks} != [{root_prefix}/Cargo.lock]"
    if manifest_text is None:
        return "archive manifest unreadable"
    manifest = tomllib.loads(manifest_text)
    package = manifest.get("package", {})
    if package.get("name") != facts["name"]:
        return f"archive package name {package.get('name')!r} != {facts['name']!r}"
    if package.get("version") != facts["version"]:
        return f"archive version {package.get('version')!r} != {facts['version']!r}"
    if package.get("rust-version") != facts["rust_version"]:
        return f"archive rust-version {package.get('rust-version')!r} != {facts['rust_version']!r}"
    dependencies = manifest.get("dependencies", {})
    if set(dependencies) != {"typenum"}:
        return f"archive dependencies {sorted(dependencies)} != ['typenum']"
    typenum_spec = dependencies["typenum"]
    if isinstance(typenum_spec, dict) and typenum_spec.get("optional"):
        return "typenum dependency is optional"
    requirement = dependency_requirement(typenum_spec)
    if requirement != facts["typenum_requirement"]:
        return f"archive typenum requirement {requirement!r} != {facts['typenum_requirement']!r}"
    normalized = {
        "name": package["name"],
        "version": package["version"],
        "rust_version": package["rust-version"],
        "dependencies": {name: dependency_requirement(spec) for name, spec in dependencies.items()},
    }
    (run_dir / "archive-manifest.json").write_text(json.dumps(normalized, indent=2) + "\n")
    (run_dir / "manifests").mkdir()
    (run_dir / "manifests" / "Cargo.toml").write_text(manifest_text)
    return None


def extract_archive(archive, extract_root, facts):
    root_prefix = f"{facts['name']}-{facts['version']}"
    with tarfile.open(archive, "r:gz") as archive_file:
        archive_file.extractall(extract_root, filter="data")
    entries = sorted(entry.name for entry in extract_root.iterdir())
    if entries != [root_prefix]:
        return f"extracted entries {entries} != [{root_prefix}]"
    return None


def check_metadata(meta, facts):
    packages = {package["name"]: package for package in meta["packages"]}
    for name in packages:
        if name != facts["name"] and name.startswith("tuplities-"):
            return f"satellite package in metadata: {name}"
    by_id = {package["id"]: package for package in meta["packages"]}
    members = [by_id[member_id]["name"] for member_id in meta["workspace_members"]]
    if members != [facts["name"]]:
        return f"workspace members {members} != [{facts['name']}]"
    if set(packages) != {facts["name"], "typenum"}:
        return f"metadata packages {sorted(packages)} != [{facts['name']}, 'typenum']"
    package = packages[facts["name"]]
    if package["version"] != facts["version"]:
        return f"package version {package['version']!r} != {facts['version']!r}"
    if package.get("rust_version") != facts["rust_version"]:
        return f"package rust_version {package.get('rust_version')!r} != {facts['rust_version']!r}"
    production = [dep for dep in package["dependencies"] if dep.get("kind") in (None, "normal")]
    if [dep["name"] for dep in production] != ["typenum"]:
        return f"production dependencies {sorted(dep['name'] for dep in production)} != ['typenum']"
    if production[0].get("optional"):
        return "typenum dependency is optional"
    return None


def verify_metadata(text, facts, run_dir):
    try:
        meta = json.loads(text)
    except (ValueError, IndexError) as exc:
        return f"metadata output unparseable: {exc}"
    error = check_metadata(meta, facts)
    if error:
        return error
    (run_dir / "metadata.json").write_text(json.dumps(meta, indent=2) + "\n")
    return None



def main():
    run_dir = OUTPUT_ROOT / (datetime.now(timezone.utc).strftime("run-%Y%m%d-%H%M%S-pid") + str(os.getpid()))
    run_dir.mkdir(parents=True)
    temp_base = Path(tempfile.mkdtemp(prefix="tuplities-package-verify-"))
    extract_root = temp_base / "source"
    extract_root.mkdir()
    steps = []
    facts = None
    failure = None
    archive_path = None

    def record(name, status, duration, log=None):
        entry = {"step": name, "status": status, "duration": round(duration, 2)}
        if log is not None:
            entry["log"] = str(log)
        steps.append(entry)

    def run_step(name, cmd, cwd, env, log, timeout, verify=None):
        identity = f"{run_dir.name}/{name}"
        start = time.monotonic()
        error = None
        try:
            exit_code, stdout = run_logged(cmd, cwd, env, log, identity, timeout)
            if exit_code != 0:
                error = f"exit={exit_code}"
            elif verify is not None:
                error = verify(stdout)
        except Exception as exc:
            error = f"{type(exc).__name__} {exc}"
        duration = time.monotonic() - start
        record(name, "ok" if error is None else "failed", duration, log)
        if error is not None:
            raise StepFailure(f"{name}: {error}, log={log}")

    def process_step(name, func):
        start = time.monotonic()
        try:
            result = func()
        except Exception as exc:
            record(name, "failed", time.monotonic() - start)
            raise StepFailure(f"{name} failed ({type(exc).__name__}) {exc}") from exc
        record(name, "ok", time.monotonic() - start)
        return result

    try:
        facts = process_step("checkout-manifest", checkout_facts)
        name = facts["name"]
        version = facts["version"]

        def versions():
            lines = []
            for label, command in (
                ("cargo", CARGO + ["--version"]),
                ("msrv-cargo", MSRV_CARGO + ["--version"]),
            ):
                lines.append(f"{label}={capture(command, ROOT, os.environ, VERSION_TIMEOUT).strip()}")
            rustc = ["rustc", "--version"]
            if len(MSRV_CARGO) > 1 and MSRV_CARGO[1].startswith("+"):
                rustc = ["rustup", "run", MSRV_CARGO[1][1:], *rustc]
            lines.append(f"msrv-rustc={capture(rustc, ROOT, os.environ, VERSION_TIMEOUT).strip()}")
            (run_dir / "versions.txt").write_text("\n".join(lines) + "\n")

        process_step("versions", versions)

        package_cmd = ["nice", "-n", "10", *CARGO, "package", "-p", name, "--locked", "-j", "4"]
        if ALLOW_DIRTY:
            package_cmd.append("--allow-dirty")
        run_step("package", package_cmd, ROOT, os.environ, run_dir / "package.log", PACKAGE_TIMEOUT)
        archive = ROOT / "target" / "package" / f"{name}-{version}.crate"
        if not archive.is_file():
            raise StepFailure(f"expected archive missing: {archive}")
        archive_path = archive

        def archive_inspect():
            error = inspect_archive(archive, run_dir, facts)
            if error:
                raise StepFailure(error)

        process_step("archive-inspect", archive_inspect)

        def extract():
            error = extract_archive(archive, extract_root, facts)
            if error:
                raise StepFailure(error)

        process_step("extract", extract)
        extracted = extract_root / f"{name}-{version}"

        run_step(
            "metadata",
            ["nice", "-n", "10", *MSRV_CARGO, "metadata", "--format-version", "1", "--locked"],
            extracted,
            os.environ,
            run_dir / "metadata.log",
            META_TIMEOUT,
            verify=lambda text: verify_metadata(text, facts, run_dir),
        )

        for case_name, extra in DOCTEST_CASES:
            doc_env = {**os.environ, "CARGO_TARGET_DIR": str(run_dir / "target" / f"doc-{case_name}")}
            run_step(
                f"doctest-{case_name}",
                ["nice", "-n", "10", *MSRV_CARGO, "test", "-p", name, "--doc", "--locked", "-j", "4", *extra],
                extracted,
                doc_env,
                run_dir / f"doctest-{case_name}.log",
                TEST_TIMEOUT,
            )

        for width in BRIDGE_WIDTHS:
            features = "flatten-nest" if width == 8 else f"flatten-nest,size-{width}"
            bridge_env = {
                **os.environ,
                "CARGO_TARGET_DIR": str(run_dir / "target" / f"bridge-{width}"),
            }
            run_step(
                f"bridge-{width}",
                [
                    "nice", "-n", "10", *MSRV_CARGO, "test", "-p", name,
                    "--no-default-features", f"--features={features}",
                    "--test", "flat_bridge_contracts",
                    "--test", "flat_bridge_sizes",
                    "--locked", "-j", "4",
                ],
                extracted,
                bridge_env,
                run_dir / f"bridge-{width}.log",
                TEST_TIMEOUT,
            )

        harness_env = {
            **os.environ,
            "CARGO": shlex.join(MSRV_CARGO),
            "TUPLITIES_PACKAGE_DIR": str(extracted),
            "TUPLITIES_VERIFICATION_DIR": str(run_dir / "compile-workloads"),
        }
        run_step(
            "compile-workloads",
            [str(ROOT / "tools" / "verify_compile_workloads.sh")],
            ROOT,
            harness_env,
            run_dir / "compile-workloads.log",
            WORKLOADS_TIMEOUT,
        )
    except StepFailure as step_failure:
        failure = str(step_failure)
    finally:
        shutil.rmtree(temp_base)
        summary = {
            "run_dir": str(run_dir),
            "package": (facts or {}).get("name"),
            "version": (facts or {}).get("version"),
            "msrv_toolchain": MSRV_TOOLCHAIN,
            "msrv_cargo": shlex.join(MSRV_CARGO),
            "allow_dirty": ALLOW_DIRTY,
            "archive": str(archive_path) if archive_path else None,
            "steps": steps,
            "failure": failure,
        }
        (run_dir / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")

    print(f"run dir: {run_dir}")
    if archive_path is not None:
        print(f"archive: {archive_path}")
    if failure is not None:
        print(f"FAILED {failure}", file=sys.stderr)
        return 1
    print("ok")
    return 0


sys.exit(main())
PY
