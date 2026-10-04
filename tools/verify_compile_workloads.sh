#!/bin/sh
set -eu

root=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)

exec python3 -B - "$root" <<'PY'
import json
import os
import shlex
import shutil
import signal
import subprocess
import sys
import tempfile
import time
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(sys.argv[1]).resolve()
BUILD_TIMEOUT = 1200
META_TIMEOUT = 120
RUN_TIMEOUT = 60
VERSION_TIMEOUT = 30


def resolve(path):
    path = Path(path)
    return path if path.is_absolute() else (ROOT / path).resolve()


PACKAGE_DIR = resolve(os.environ.get("TUPLITIES_PACKAGE_DIR", str(ROOT / "tuplities")))
OUTPUT_ROOT = resolve(
    os.environ.get(
        "TUPLITIES_VERIFICATION_DIR",
        str(ROOT / "target" / "verification" / "compile-workloads"),
    )
)
CARGO = shlex.split(os.environ.get("CARGO", "cargo"))

CASES = [
    ("recursive8", 8, [], None),
    ("recursive32", 32, [], None),
    ("recursive64", 64, [], None),
    ("recursive128", 128, [], None),
    ("flat8", 8, ["flatten-nest"], 8),
    ("flat32", 32, ["flatten-nest", "size-32"], 32),
    ("flat128", 128, ["flatten-nest", "size-128"], 128),
]


class CaseFailure(Exception):
    pass


def nested_literal(values):
    out = f"({values[-1]},)"
    for value in reversed(values[:-1]):
        out = f"({value}, {out})"
    return out


def generate_main(case, width, flat_width):
    values = [1000 + 7 * i for i in range(width)]
    option_values = [20000 + 13 * i for i in range(width)]
    flat_values = [30000 + 11 * i for i in range(flat_width)] if flat_width else []
    middle = width // 2
    split_at = width // 2
    remove_at = width // 4
    insert_value = 900000
    mutated_value = 800000
    mutated = list(values)
    mutated[middle] = mutated_value
    inserted_values = values[:1] + [insert_value] + values[1:]
    removed_values = values[:remove_at] + values[remove_at + 1:]
    u32 = lambda count: ", ".join(["u32"] * count)
    lines = [
        '#![recursion_limit = "512"]',
        "use tuplities::prelude::*;",
        "",
        "fn main() {",
        f"    type List = neplety!({u32(width)});",
        f"    assert_eq!(List::LEN, {width});",
        f"    let mut list: List = neple!({', '.join(map(str, values))});",
        "",
        f"    assert_eq!(*NestedTupleIndex::<typenum::U0>::nested_index(&list), {values[0]});",
        f"    assert_eq!(*NestedTupleIndex::<typenum::U{middle}>::nested_index(&list), {values[middle]});",
        f"    assert_eq!(*NestedTupleIndex::<typenum::U{width - 1}>::nested_index(&list), {values[-1]});",
        "",
        f"    type Options = neplety!({', '.join(['Option<u32>'] * width)});",
        f"    let options: Options = neple!({', '.join(f'Some({v})' for v in option_values)});",
        "    let transposed: Option<List> = options.transpose();",
        f"    assert_eq!(transposed, Some({nested_literal(option_values)}));",
        "",
        f"    let (prefix, suffix): (neplety!({u32(split_at)}), neplety!({u32(width - split_at)})) =",
        f"        NestedTupleSplit::<typenum::U{split_at}>::nested_split(list);",
        f"    assert_eq!(prefix, {nested_literal(values[:split_at])});",
        f"    assert_eq!(suffix, {nested_literal(values[split_at:])});",
        "",
        f"    let inserted = NestedTupleInsert::<typenum::U1, u32>::nested_insert(list, {insert_value});",
        f"    assert_eq!(inserted, {nested_literal(inserted_values)});",
        "",
        f"    let (removed, remainder) = NestedTupleRemove::<typenum::U{remove_at}>::nested_remove(list);",
        f"    assert_eq!(removed, {values[remove_at]});",
        f"    assert_eq!(remainder, {nested_literal(removed_values)});",
        "",
        "    let reversed = NestedTupleReverse::nested_reverse(list);",
        f"    assert_eq!(reversed, {nested_literal(values[::-1])});",
        "",
        f"    *NestedTupleIndexMut::<typenum::U{middle}>::nested_index_mut(&mut list) = {mutated_value};",
        f"    assert_eq!(*NestedTupleIndex::<typenum::U{middle}>::nested_index(&list), {mutated_value});",
        "    let final_reversed = NestedTupleReverse::nested_reverse(list);",
        f"    assert_eq!(final_reversed, {nested_literal(mutated[::-1])});",
    ]
    scenarios = (
        "construction,indexing-shared,indexing-mutable,option-transpose,"
        "split,insert,remove,reverse"
    )
    if flat_width:
        flat_csv = ", ".join(map(str, flat_values))
        lines += [
            "",
            f"    let flat = ({flat_csv});",
            "    let round_trip = flat.nest().flatten();",
        ]
        lines += [f"    assert_eq!(round_trip.{index}, {value});" for index, value in enumerate(flat_values)]
        scenarios += f",flat-roundtrip-{flat_width}"
    lines += [
        "",
        f'    println!("compile-workload case={case} width={width} scenarios={scenarios} status=ok");',
        "}",
        "",
    ]
    return "\n".join(lines), scenarios


def generate_manifest(case, package_dir, features):
    extra = ""
    if features:
        extra = ", features = [" + ", ".join(f'"{feature}"' for feature in features) + "]"
    return (
        "[package]\n"
        f'name = "workload-{case}"\n'
        'version = "0.0.0"\n'
        'edition = "2021"\n'
        "publish = false\n"
        "\n"
        "[workspace]\n"
        "\n"
        "[dependencies]\n"
        f'tuplities = {{ path = "{package_dir}", default-features = false{extra} }}\n'
        'typenum = "1"\n'
    )


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
    start = time.monotonic()
    try:
        completed = execute(cmd, cwd, env, timeout)
        output = completed.stdout + completed.stderr
        exit_code = completed.returncode
    except subprocess.TimeoutExpired as exc:
        output = "".join(
            part
            for part in (exc.stdout or "", exc.stderr or "")
            if isinstance(part, str)
        )
        exit_code = -1
    duration = time.monotonic() - start
    with open(log_path, "a") as handle:
        handle.write(header + output + "--- summary ---\n")
        handle.write(f"duration={duration:.2f}s exit={exit_code}\n")
    return exit_code


def capture(cmd, cwd, env, timeout):
    completed = execute(cmd, cwd, env, timeout)
    if completed.returncode != 0:
        raise CaseFailure(f"command failed: {shlex.join(map(str, cmd))}\n{completed.stderr}")
    return completed.stdout


def record_versions(case_dir, packages):
    env = os.environ
    cargo_version = capture(CARGO + ["--version"], case_dir, env, VERSION_TIMEOUT).strip()
    rustc = ["rustc", "--version"]
    if len(CARGO) > 1 and CARGO[1].startswith("+"):
        rustc = ["rustup", "run", CARGO[1][1:], *rustc]
    rustc_version = capture(rustc, case_dir, env, VERSION_TIMEOUT).strip()
    lines = [
        f"cargo={cargo_version}",
        f"rustc={rustc_version}",
    ]
    for name in sorted(packages):
        package = packages[name]
        lines.append(
            f"package {name} version={package['version']} "
            f"rust_version={package.get('rust_version') or 'unspecified'}"
        )
    for key in ("CARGO", "RUSTC_WRAPPER", "CARGO_BUILD_RUSTC_WRAPPER"):
        if key in env:
            lines.append(f"{key}={env[key]}")
    (case_dir / "versions.txt").write_text("\n".join(lines) + "\n")


def run_case(case, width, features, flat_width, run_dir, temp_base):
    src_dir = temp_base / case
    (src_dir / "src").mkdir(parents=True)
    main_source, scenarios = generate_main(case, width, flat_width)
    manifest = generate_manifest(case, PACKAGE_DIR, features)
    (src_dir / "Cargo.toml").write_text(manifest)
    (src_dir / "src" / "main.rs").write_text(main_source)

    case_dir = run_dir / case
    case_dir.mkdir(parents=True)
    (case_dir / "generated").mkdir()
    shutil.copy2(src_dir / "Cargo.toml", case_dir / "generated" / "Cargo.toml")
    shutil.copy2(src_dir / "src" / "main.rs", case_dir / "generated" / "main.rs")
    target_dir = case_dir / "target"
    env = os.environ

    (case_dir / "config.json").write_text(
        json.dumps(
            {
                "case": case,
                "width": width,
                "features": features,
                "flat_width": flat_width,
                "package_dir": str(PACKAGE_DIR),
                "scenarios": scenarios,
            },
            indent=2,
        )
        + "\n"
    )

    built_package_ids = set()
    for build_index in (1, 2):
        cmd = ["nice", "-n", "10", *CARGO, "build", "--release", "--timings", "--message-format=json-render-diagnostics", "-j", "4"]
        if build_index == 2:
            cmd.append("--locked")
        build_env = {**env, "CARGO_TARGET_DIR": str(target_dir)}
        identity = f"{run_dir.name}/{case}/build{build_index}"
        exit_code = run_logged(
            cmd, src_dir, build_env, case_dir / f"build{build_index}.log", identity, BUILD_TIMEOUT
        )
        if exit_code != 0:
            raise CaseFailure(f"build {build_index} exit={exit_code}, log={case_dir / f'build{build_index}.log'}")
        shutil.copytree(target_dir / "cargo-timings", case_dir / "timings" / f"build{build_index}")
        for line in (case_dir / f"build{build_index}.log").read_text().splitlines():
            if line.startswith("{"):
                event = json.loads(line)
                if event.get("reason") == "compiler-artifact":
                    built_package_ids.add(event["package_id"])

    metadata_text = capture(
        [*CARGO, "metadata", "--format-version", "1", "--frozen"],
        src_dir, env, META_TIMEOUT
    )
    metadata = json.loads(metadata_text)
    (case_dir / "metadata.json").write_text(metadata_text)
    packages = {package["name"]: package for package in metadata["packages"]}
    expected_packages = {"workload-" + case, "tuplities", "typenum"}
    if set(packages) != expected_packages:
        raise CaseFailure(f"resolved packages {sorted(packages)} != {sorted(expected_packages)}")

    dependency_tree = capture([*CARGO, "tree", "--frozen"], src_dir, env, META_TIMEOUT)
    feature_tree = capture([*CARGO, "tree", "--frozen", "-e", "features"], src_dir, env, META_TIMEOUT)
    (case_dir / "dependency-tree.txt").write_text(dependency_tree)
    (case_dir / "feature-tree.txt").write_text(feature_tree)
    if features:
        if "flatten-nest" not in feature_tree or "alloc" not in feature_tree:
            raise CaseFailure("feature tree lacks flatten-nest/alloc")
        if flat_width in (32, 128) and f"size-{flat_width}" not in feature_tree:
            raise CaseFailure(f"feature tree lacks size-{flat_width}")
    else:
        for feature in ("flatten-nest", "alloc", "size-"):
            if feature in feature_tree:
                raise CaseFailure(f"recursive case activated feature {feature}")

    package_names = {package["id"]: package["name"] for package in metadata["packages"]}
    unit_names = {package_names[package_id] for package_id in built_package_ids}
    if unit_names != expected_packages:
        raise CaseFailure(f"built packages {sorted(unit_names)} != {sorted(expected_packages)}")
    (case_dir / "built-packages.json").write_text(json.dumps(sorted(unit_names)) + "\n")

    record_versions(case_dir, packages)

    executable = target_dir / "release" / f"workload-{case}"
    expected_line = f"compile-workload case={case} width={width} scenarios={scenarios} status=ok"
    exit_code = run_logged(
        [executable], src_dir, env, case_dir / "run.log", f"{run_dir.name}/{case}/run", RUN_TIMEOUT
    )
    if exit_code != 0:
        raise CaseFailure(f"executable exit={exit_code}, log={case_dir / 'run.log'}")
    stdout = (case_dir / "run.log").read_text().split("--- output ---\n", 1)[1]
    stdout = stdout.split("--- summary ---\n", 1)[0].strip()
    if stdout != expected_line:
        raise CaseFailure(f"stdout {stdout!r} != expected {expected_line!r}")


def main():
    if not PACKAGE_DIR.is_dir():
        print(f"tuplities package dir missing: {PACKAGE_DIR}", file=sys.stderr)
        return 1
    run_dir = OUTPUT_ROOT / (datetime.now(timezone.utc).strftime("run-%Y%m%d-%H%M%S-pid") + str(os.getpid()))
    run_dir.mkdir(parents=True)
    temp_base = Path(tempfile.mkdtemp(prefix="tuplities-compile-workloads-"))
    failures = []
    try:
        for case, width, features, flat_width in CASES:
            try:
                run_case(case, width, features, flat_width, run_dir, temp_base)
                print(f"ok {case}")
            except CaseFailure as failure:
                failures.append(case)
                print(f"FAILED {case}: {failure}", file=sys.stderr)
    finally:
        shutil.rmtree(temp_base)
    print(f"run dir: {run_dir}")
    if failures:
        print(f"failed cases: {', '.join(failures)}", file=sys.stderr)
        return 1
    return 0


sys.exit(main())
PY
