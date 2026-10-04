#!/usr/bin/env bash
# Verify diesel-builders at git main, or at an explicit paired revision, against the local tuplities checkout.
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd -- "${script_dir}/.." && pwd)"
package_dir="${TUPLITIES_PACKAGE_DIR:-${root}/tuplities}"
cargo_bin="${CARGO:-cargo}"

if [[ ! -f "${package_dir}/Cargo.toml" ]]; then
    echo "error: tuplities manifest not found at ${package_dir}/Cargo.toml" >&2
    exit 1
fi

run_id="$(date -u +%Y%m%dT%H%M%SZ)-$$"
log_dir="${root}/target/verification/diesel-builders/${run_id}"
work_dir="$(mktemp -d "${TMPDIR:-/tmp}/tuplities-diesel-builders-verify.XXXXXX")"
cleanup() { rm -rf "${work_dir}"; }
trap cleanup EXIT
mkdir -p "${log_dir}"

python3 - "${root}" "${package_dir}" "${work_dir}" "${log_dir}" "${cargo_bin}" "${run_id}" <<'PY'
import json
import os
import re
import signal
import subprocess
import sys
from datetime import datetime, timezone

ROOT, PKG_DIR, WORK_DIR, LOG_DIR, CARGO, RUN_ID = sys.argv[1:7]

# Subprocess timeouts in seconds. Every cargo invocation carries one.
TIMEOUTS = {"version": 30, "update": 900, "metadata": 120, "check": 1500, "run": 1500}

# Paired revisions pin the builder source and the shared Diesel lockfile entry.
REV_ENV = {
    "diesel-builders": "DIESEL_BUILDERS_REV",
    "diesel": "DIESEL_REV",
}
REV_RE = re.compile(r"^[0-9a-f]{40}$")
SOURCE_URLS = {
    "diesel-builders": "https://github.com/LucaCappelletti94/diesel-builders",
    "diesel": "https://github.com/diesel-rs/diesel",
}

# Two consumer configurations: the consumer's defaults, and no defaults with
# size-32. Each entry carries the extra feature options its configuration adds.
CONFIGS = [
    ("default", ""),
    ("no-default-size32", 'default-features = false, features = ["size-32"]'),
]

# Every scenario the smoke executable must reach. The harness fails if a
# marker is absent, so an omitted scenario cannot pass silently.
MARKERS = [
    "defaults",
    "missing_email_error_identity",
    "setters_insert_load",
    "index_reads",
    "sql_query_construction",
    "find_after_mutation",
    "upsert",
    "ancestor_bundle",
    "ancestor_read",
    "delete",
]


ENV = {**os.environ, "CARGO_TERM_COLOR": "never"}


class StepFailure(Exception):
    def __init__(self, message, log_path):
        super().__init__(message)
        self.log_path = log_path


class UsageError(Exception):
    pass


def utcnow():
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def write_log(path, text):
    with open(path, "w") as fh:
        fh.write(text)


def run_cargo(config, step, args, cwd, log_path):
    cmd = ["nice", "-n", "10", CARGO, *args]
    if step in ("check", "run"):
        # --locked builds the exact resolution the metadata check saw.
        cmd += ["-j", "4", "--locked"]
    header = (
        f"run={RUN_ID} config={config} step={step} started={utcnow()}\n"
        f"$ {' '.join(cmd)}\n"
    )
    timeout = TIMEOUTS[step]
    with subprocess.Popen(
        cmd,
        cwd=cwd,
        env=ENV,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        start_new_session=True,
    ) as proc:
        try:
            out, err = proc.communicate(timeout=timeout)
        except subprocess.TimeoutExpired:
            os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
            out, err = proc.communicate(timeout=30)
            write_log(
                log_path,
                header + f"TIMED OUT after {timeout}s\n" + out + err,
            )
            raise StepFailure(f"step {step} timed out after {timeout}s", log_path) from None
    status = "ok" if proc.returncode == 0 else f"failed exit_code={proc.returncode}"
    write_log(log_path, header + out + err + f"--- ended={utcnow()} status={status} ---\n")
    if proc.returncode != 0:
        raise StepFailure(f"step {step} failed with exit code {proc.returncode}", log_path)
    return subprocess.CompletedProcess(cmd, proc.returncode, out, err)


def run_versioned(label, cmd):
    proc = subprocess.run(
        cmd, env=ENV, timeout=TIMEOUTS["version"],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=True,
    )
    return f"{label} {proc.stdout.strip()}"


def requested_revisions():
    values = {
        name: (os.environ.get(env) or "").strip()
        for name, env in REV_ENV.items()
    }
    set_names = [name for name, value in values.items() if value]
    if len(set_names) == 1:
        raise UsageError(
            f"{REV_ENV[set_names[0]]} is set without the other revision input; "
            f"{REV_ENV['diesel-builders']} and {REV_ENV['diesel']} must be provided together"
        )
    for name in set_names:
        if not REV_RE.fullmatch(values[name]):
            raise UsageError(
                f"{REV_ENV[name]} must be a full 40-character lowercase git SHA, "
                f"got {values[name]!r}"
            )
    return values if set_names else {}


def source_ref(name, revisions):
    if name == "diesel-builders" and revisions:
        return f'rev = "{revisions[name]}"'
    return 'branch = "main"'


def expected_source_prefix(name, revisions):
    if name == "diesel-builders" and revisions:
        return f"git+{SOURCE_URLS[name]}?rev={revisions[name]}#"
    return f"git+{SOURCE_URLS[name]}?branch=main#"


def dependency_lines(revisions):
    builders_url = SOURCE_URLS["diesel-builders"]
    lines = {}
    for config, extras in CONFIGS:
        line = (
            f'diesel-builders = {{ git = "{builders_url}", '
            f"{source_ref('diesel-builders', revisions)}"
        )
        if extras:
            line += f", {extras}"
        lines[config] = line + " }"
    diesel = (
        f'diesel = {{ git = "{SOURCE_URLS["diesel"]}", '
        f"{source_ref('diesel', revisions)}, "
        'features = ["sqlite", "returning_clauses_for_sqlite_3_35"] }'
    )
    return lines, diesel


def check_metadata(meta, revisions):
    by_name = {}
    for package in meta["packages"]:
        by_name.setdefault(package["name"], []).append(package)

    tuplities = by_name.get("tuplities") or []
    if len(tuplities) != 1:
        raise StepFailure(
            f"expected exactly one tuplities package in the graph, found {len(tuplities)}",
            None,
        )
    entry = tuplities[0]
    expected = os.path.join(PKG_DIR, "Cargo.toml")
    if entry["manifest_path"] != expected or entry.get("source") is not None:
        raise StepFailure(
            f"tuplities did not resolve to the local package {expected}, "
            f"resolved to {entry['manifest_path']} (source={entry.get('source')})",
            None,
        )

    resolved = {}
    for name in SOURCE_URLS:
        entries = by_name.get(name) or []
        if not entries:
            raise StepFailure(f"{name} is missing from the resolved graph", None)
        prefix = expected_source_prefix(name, revisions)
        found = set()
        for package in entries:
            source = package.get("source") or ""
            if not source.startswith(prefix):
                raise StepFailure(
                    f"{name} resolved to the unexpected source {source!r}, expected "
                    f"{prefix} followed by the revision",
                    None,
                )
            found.add(source.rsplit("#", 1)[-1])
        if len(found) != 1:
            raise StepFailure(
                f"{name} resolved to more than one revision, "
                f"{', '.join(sorted(found))}",
                None,
            )
        resolved[name] = next(iter(found))
        if revisions and resolved[name] != revisions[name]:
            raise StepFailure(
                f"{name} resolved to {resolved[name]}, expected {revisions[name]}",
                None,
            )
    return resolved


def parse_markers(output):
    return {
        line[len("scen "):].strip()
        for line in output.splitlines()
        if line.startswith("scen ")
    }


SMOKE_MAIN = r'''
use diesel::prelude::*;
use diesel_builders::prelude::*;
use diesel_builders::{BuilderError, IncompleteBuilderError};

/// User model for the `users` table, mirroring the consumer defaults fixture.
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, PartialEq, TableModel)]
#[diesel(table_name = users)]
#[table_model(surrogate_key)]
pub struct User {
    pub id: i32,
    #[table_model(default = "Guest")]
    pub name: String,
    #[table_model(default = "User")]
    pub role: String,
    #[table_model(default = true)]
    pub active: bool,
    pub bio: Option<String>,
    pub email: String,
}

/// Profile model inheriting from `users` with an inherited ancestor default.
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, PartialEq, TableModel)]
#[diesel(table_name = profiles)]
#[table_model(ancestors(users))]
#[table_model(default(users::bio, "Default bio"))]
pub struct Profile {
    pub id: i32,
    pub location: String,
}

fn establish_connection() -> SqliteConnection {
    let mut conn = SqliteConnection::establish(":memory:").expect("in-memory SQLite connection");
    diesel::sql_query("PRAGMA foreign_keys = ON")
        .execute(&mut conn)
        .expect("foreign key pragma");
    conn
}

fn create_tables(conn: &mut SqliteConnection) -> QueryResult<()> {
    use diesel::RunQueryDsl;
    diesel::sql_query(
        "CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            role TEXT NOT NULL,
            active BOOLEAN NOT NULL,
            bio TEXT,
            email TEXT NOT NULL
        )",
    )
    .execute(conn)?;
    diesel::sql_query(
        "CREATE TABLE profiles (
            id INTEGER PRIMARY KEY NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            location TEXT NOT NULL
        )",
    )
    .execute(conn)?;
    Ok(())
}

fn scenario_defaults() {
    let builder = users::table::builder();
    assert_eq!(
        builder.may_get_column::<users::name>(),
        Some("Guest".to_string())
    );
    assert_eq!(builder.may_get_column::<users::role>(), Some("User".to_string()));
    assert_eq!(builder.may_get_column::<users::active>(), Some(true));
    assert_eq!(builder.may_get_column::<users::bio>(), Some(None));
    assert_eq!(builder.may_get_column::<users::email>(), None);
    println!("scen defaults");
}

fn scenario_missing_email_error_identity(conn: &mut SqliteConnection) {
    let builder = users::table::builder();
    let err = builder.clone().insert(conn).unwrap_err();
    assert!(
        matches!(
            err,
            BuilderError::Incomplete(IncompleteBuilderError::MissingMandatoryField {
                table_name: "users",
                field_name: "email",
            })
        ),
        "expected the missing mandatory field identity, got: {err:?}"
    );
    println!("scen missing_email_error_identity");
}

fn scenario_setters_insert_load(conn: &mut SqliteConnection) -> Result<User, Box<dyn std::error::Error>> {
    let user = users::table::builder()
        .try_name("Admin".to_string())?
        .try_bio(Some("Bio".to_string()))?
        .try_email("test@example.com".to_string())?
        .insert(conn)?;
    let loaded = User::find(user.id(), conn)?;
    assert_eq!(loaded.id, user.id);
    assert_eq!(loaded.name, "Admin");
    assert_eq!(loaded.role, "User");
    assert_eq!(loaded.active, true);
    assert_eq!(loaded.bio, Some("Bio".to_string()));
    assert_eq!(loaded.email, "test@example.com");
    assert_eq!(loaded, user);
    println!("scen setters_insert_load");
    Ok(user)
}

fn scenario_index_reads(user: &User) {
    assert_eq!(*user.id(), user.id);
    assert_eq!(user.name(), "Admin");
    assert_eq!(user.role(), "User");
    assert_eq!(user.active(), &true);
    assert_eq!(user.bio(), &Some("Bio".to_string()));
    println!("scen index_reads");
}

fn scenario_sql_query_construction(
    conn: &mut SqliteConnection,
    user: &User,
) -> Result<(), Box<dyn std::error::Error>> {
    let loaded = <(users::email,) as LoadFirst<SqliteConnection>>::load_first(
        ("test@example.com",),
        conn,
    )?;
    assert_eq!(loaded, *user);
    let many = <(users::email,) as LoadMany<SqliteConnection>>::load_many(
        ("test@example.com",),
        conn,
    )?;
    assert_eq!(many, vec![user.clone()]);
    println!("scen sql_query_construction");
    Ok(())
}

fn scenario_find_after_mutation(
    conn: &mut SqliteConnection,
    user: &mut User,
) -> Result<(), Box<dyn std::error::Error>> {
    let id = *user.id();
    user.name = "Changed name".to_string();
    let found = User::find(&id, conn)?;
    assert_eq!(found.name, "Admin");
    assert_eq!(user.name, "Changed name");
    println!("scen find_after_mutation");
    Ok(())
}

fn scenario_upsert(
    conn: &mut SqliteConnection,
    user: &mut User,
) -> Result<(), Box<dyn std::error::Error>> {
    user.role = "Moderator".to_string();
    let upserted = user.upsert(conn)?;
    assert_eq!(upserted.id, *user.id());
    assert_eq!(upserted.name, "Changed name");
    assert_eq!(upserted.role, "Moderator");
    assert_eq!(upserted.active, true);
    assert_eq!(upserted.bio, Some("Bio".to_string()));
    assert_eq!(upserted.email, "test@example.com");
    let loaded = User::find(user.id(), conn)?;
    assert_eq!(loaded, upserted);
    println!("scen upsert");
    Ok(())
}

fn scenario_ancestor_bundle(
    conn: &mut SqliteConnection,
) -> Result<(Profile, User), Box<dyn std::error::Error>> {
    let builder = profiles::table::builder();
    assert_eq!(
        builder.may_get_column::<users::bio>(),
        Some(Some("Default bio".to_string()))
    );
    assert_eq!(
        builder.may_get_column::<users::name>(),
        Some("Guest".to_string())
    );
    assert_eq!(builder.may_get_column::<profiles::location>(), None);

    let profile = builder
        .try_email("profile@example.com".to_string())?
        .location("Rome")
        .insert(conn)?;
    let profile_user = User::find(profile.id(), conn)?;
    assert_eq!(profile_user.name, "Guest");
    assert_eq!(profile_user.role, "User");
    assert_eq!(profile_user.active, true);
    assert_eq!(profile_user.bio, Some("Default bio".to_string()));
    assert_eq!(profile_user.email, "profile@example.com");
    let loaded = Profile::find(profile.id(), conn)?;
    assert_eq!(loaded.location, "Rome");
    assert_eq!(loaded, profile);
    println!("scen ancestor_bundle");
    Ok((profile, profile_user))
}

fn scenario_ancestor_read(
    conn: &mut SqliteConnection,
    profile: &Profile,
    profile_user: &User,
) -> Result<(), Box<dyn std::error::Error>> {
    let ancestor: User = profile.ancestor(conn)?;
    assert_eq!(ancestor, *profile_user);
    println!("scen ancestor_read");
    Ok(())
}

fn scenario_delete(
    conn: &mut SqliteConnection,
    profile: &Profile,
    user_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let deleted = profile.delete(conn)?;
    assert_eq!(deleted, 1);
    assert!(!Profile::exists(profile.id(), conn)?);
    assert!(!User::exists(profile.id(), conn)?);
    assert!(User::exists(&user_id, conn)?);
    println!("scen delete");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = establish_connection();
    create_tables(&mut conn)?;

    scenario_defaults();
    scenario_missing_email_error_identity(&mut conn);
    let mut user = scenario_setters_insert_load(&mut conn)?;
    scenario_index_reads(&user);
    scenario_sql_query_construction(&mut conn, &user)?;
    scenario_find_after_mutation(&mut conn, &mut user)?;
    scenario_upsert(&mut conn, &mut user)?;
    let (profile, profile_user) = scenario_ancestor_bundle(&mut conn)?;
    scenario_ancestor_read(&mut conn, &profile, &profile_user)?;
    scenario_delete(&mut conn, &profile, *user.id())?;

    println!("smoke complete");
    Ok(())
}
'''

CARGO_TOML = """\
[package]
name = "diesel-builders-downstream"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
{dep_line}
{diesel_line}

[workspace]

[patch."https://github.com/LucaCappelletti94/tuplities"]
tuplities = {{ path = "{pkg_dir}" }}
"""


def materialize_project(project_dir, dep_line, diesel_line):
    os.makedirs(os.path.join(project_dir, "src"), exist_ok=True)
    write_log(
        os.path.join(project_dir, "Cargo.toml"),
        CARGO_TOML.format(
            dep_line=dep_line,
            diesel_line=diesel_line,
            pkg_dir=PKG_DIR,
        ),
    )
    write_log(os.path.join(project_dir, "src", "main.rs"), SMOKE_MAIN)


def main():
    try:
        revisions = requested_revisions()
    except UsageError as exc:
        print(f"error: {exc}", file=sys.stderr)
        sys.exit(2)
    mode = "pinned revisions" if revisions else "moving git main"
    dep_lines, diesel_line = dependency_lines(revisions)

    rustc_version = run_versioned("rustc", ["rustc", "--version"])
    cargo_version = run_versioned("cargo", [CARGO, "--version"])

    results = []
    status = "ok"
    try:
        for config, _extras in CONFIGS:
            project_dir = os.path.join(WORK_DIR, config)
            config_log_dir = os.path.join(LOG_DIR, config)
            os.makedirs(config_log_dir, exist_ok=True)
            materialize_project(project_dir, dep_lines[config], diesel_line)
            try:
                update_args = ["update"]
                if revisions:
                    update_args += ["-p", "diesel", "--precise", revisions["diesel"]]
                run_cargo(
                    config, "update", update_args, project_dir,
                    os.path.join(config_log_dir, "update.log"),
                )
                meta_proc = run_cargo(
                    config, "metadata", ["metadata", "--format-version", "1"], project_dir,
                    os.path.join(config_log_dir, "metadata.log"),
                )
                resolved = check_metadata(json.loads(meta_proc.stdout), revisions)
                run_cargo(
                    config, "check", ["check", "--all-targets"], project_dir,
                    os.path.join(config_log_dir, "check.log"),
                )
                run_proc = run_cargo(
                    config, "run", ["run", "--release"], project_dir,
                    os.path.join(config_log_dir, "run.log"),
                )
                seen = parse_markers(run_proc.stdout)
                missing = [marker for marker in MARKERS if marker not in seen]
                if missing:
                    raise StepFailure(
                        f"scenarios omitted from the smoke run: {', '.join(missing)}",
                        os.path.join(config_log_dir, "run.log"),
                    )
                results.append((config, "ok", resolved))
                print(f"config {config}: ok")
            except StepFailure as exc:
                results.append((config, f"failed: {exc}", {}))
                print(f"config {config}: FAILED ({exc})", file=sys.stderr)
                if exc.log_path:
                    print(f"logs: {exc.log_path}", file=sys.stderr)
                status = "failed"
                break
    finally:
        lines = [
            f"run={RUN_ID} finished={utcnow()} status={status}",
            rustc_version,
            cargo_version,
            f"mode: {mode}",
            f"tuplities package: {os.path.join(PKG_DIR, 'Cargo.toml')}",
        ]
        if revisions:
            lines.append(
                "requested "
                + " ".join(f"{name}={revisions[name]}" for name in SOURCE_URLS)
            )
        for config, result, config_revisions in results:
            lines.append(
                f"config {config}: {result} "
                f"diesel-builders={config_revisions.get('diesel-builders', 'unresolved')} "
                f"diesel={config_revisions.get('diesel', 'unresolved')}"
            )
        write_log(os.path.join(LOG_DIR, "run.log"), "\n".join(lines) + "\n")

    if status != "ok":
        sys.exit(1)
    print(f"all configurations verified ({mode}), logs under {LOG_DIR}")


main()
PY
