#!/usr/bin/env python3

import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import shutil
import stat
import subprocess
import sys


SCRIPT_DIR = Path(__file__).resolve().parent
REPOSITORY_ROOT = SCRIPT_DIR.parents[1]
TOPOLOGY_PATH = SCRIPT_DIR / "topology.json"
MAX_OUTPUT_BYTES = 16 * 1024 * 1024


class ShadowFailure(RuntimeError):
    pass


def run(command, *, cwd=REPOSITORY_ROOT, capture=True):
    result = subprocess.run(
        [str(value) for value in command],
        cwd=cwd,
        capture_output=capture,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        if capture:
            raise ShadowFailure(
                f"command failed ({result.returncode}): {' '.join(map(str, command))}\n"
                f"stdout:\n{result.stdout}\nstderr:\n{result.stderr}"
            )
        raise ShadowFailure(
            f"command failed ({result.returncode}): {' '.join(map(str, command))}"
        )
    return result


def sha256_bytes(value):
    return f"sha256:{hashlib.sha256(value).hexdigest()}"


def sha256_json(value):
    encoded = json.dumps(value, separators=(",", ":"), ensure_ascii=False).encode(
        "utf-8"
    )
    return sha256_bytes(encoded)


def write_identity_json(directory, identity, value):
    directory.mkdir(parents=True, exist_ok=True)
    path = directory / f"{identity.removeprefix('sha256:')}.json"
    path.write_text(f"{json.dumps(value, indent=2)}\n", encoding="utf-8")
    return path


def source_revision():
    return run(["git", "rev-parse", "HEAD"]).stdout.strip()


def tracked_files():
    result = subprocess.run(
        ["git", "-C", str(REPOSITORY_ROOT), "ls-files", "-z"],
        capture_output=True,
        check=True,
    )
    return sorted(
        Path(value.decode("utf-8"))
        for value in result.stdout.split(b"\0")
        if value
    )


def changed_paths_from_base(base_revision):
    result = subprocess.run(
        [
            "git",
            "-C",
            str(REPOSITORY_ROOT),
            "diff",
            "--name-only",
            "-z",
            f"{base_revision}...HEAD",
        ],
        capture_output=True,
        check=True,
    )
    paths = [
        Path(value.decode("utf-8"))
        for value in result.stdout.split(b"\0")
        if value
    ]
    if not paths:
        raise ShadowFailure("the selected revision range contains no changed paths")
    return paths


def inherited_environment():
    names = {
        "INCLUDE",
        "LD_LIBRARY_PATH",
        "LIB",
        "LIBPATH",
        "PATH",
        "PATHEXT",
        "SYSTEMROOT",
        "VCINSTALLDIR",
    }
    return sorted(name for name in names if name in os.environ)


def cargo_executable(explicit):
    if explicit:
        return explicit
    if shutil.which("rustup"):
        resolved = Path(run(["rustup", "which", "cargo"]).stdout.strip())
        if resolved.is_file():
            return resolved
    discovered = shutil.which("cargo")
    return Path(discovered) if discovered else Path()


def selected_packages(ferris, topology, changed_paths, full):
    if full:
        return topology["packages"], "full"

    command = [
        ferris,
        "validation-plan",
        "--workspace-id",
        topology["workspace_id"],
        "--manifest-path",
        REPOSITORY_ROOT / "Cargo.toml",
        "--format",
        "json",
    ]
    for changed_path in changed_paths:
        command.extend(["--changed-path", REPOSITORY_ROOT / changed_path])

    document = json.loads(run(command).stdout)
    if document["result_class"] != "success":
        raise ShadowFailure("Ferris validation planning did not succeed")
    record = document["record"]
    if record["fallback"]["required_by_inputs"]:
        return topology["packages"], "full_workspace_fallback"

    selected = set()
    for package in record["selected_packages"]:
        identity = package["package"]["identity"]
        prefix = "cargo-workspace-package:"
        if not identity.startswith(prefix) or "@" not in identity:
            raise ShadowFailure(f"unsupported package identity: {identity}")
        selected.add(identity.removeprefix(prefix).split("@", 1)[0])

    unknown = selected.difference(topology["packages"])
    if unknown:
        raise ShadowFailure(f"Ferris selected unknown PARLOR packages: {sorted(unknown)}")
    ordered = [name for name in topology["packages"] if name in selected]
    if not ordered:
        raise ShadowFailure("Ferris selected no PARLOR packages")
    return ordered, "changed_path"


def cargo_arguments(operation, packages, full):
    arguments = [operation, "--release"]
    if full:
        arguments.append("--workspace")
    else:
        for package in packages:
            arguments.extend(["-p", package])
    return arguments


def create_command(owner, executable, argv, environment, files):
    return {
        "owner": owner,
        "executable": executable,
        "argv": argv,
        "working_directory": ".",
        "inherited_environment": environment,
        "credential_class": "none",
        "files": files,
    }


def prepare(args):
    topology = json.loads(TOPOLOGY_PATH.read_text(encoding="utf-8"))
    changed_paths = args.changed_path
    if args.base_revision:
        changed_paths = changed_paths_from_base(args.base_revision)
    packages, selection = selected_packages(
        args.ferris, topology, changed_paths, args.full
    )

    ferris_root = REPOSITORY_ROOT / ".ferris"
    if ferris_root.exists():
        shutil.rmtree(ferris_root)

    cargo_source = cargo_executable(args.cargo)
    if not cargo_source.is_file():
        raise ShadowFailure("Cargo executable is unavailable")
    executable_name = "cargo.exe" if os.name == "nt" else "cargo"
    executable_relative = Path(".ferris") / "bin" / executable_name
    executable_path = REPOSITORY_ROOT / executable_relative
    executable_path.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(cargo_source, executable_path)
    executable_path.chmod(executable_path.stat().st_mode | stat.S_IXUSR)

    files = []
    for relative_path in [executable_relative, *tracked_files()]:
        path = REPOSITORY_ROOT / relative_path
        files.append(
            {
                "path": relative_path.as_posix(),
                "identity": sha256_bytes(path.read_bytes()),
            }
        )

    environment = inherited_environment()
    owner = "giodl73-repo/PARLOR"
    definitions = [
        (
            "quality-fmt",
            "parlor/ci/quality-fmt",
            ["fmt", "--all", "--", "--check"],
            [],
            300_000,
        ),
        (
            "quality-clippy",
            "parlor/ci/quality-clippy",
            ["clippy", "--workspace", "--all-targets", "--", "-D", "warnings"],
            ["quality-fmt"],
            1_200_000,
        ),
        (
            "build-release",
            "parlor/ci/build-release",
            cargo_arguments("build", packages, selection != "changed_path"),
            ["quality-clippy"],
            1_200_000,
        ),
        (
            "build-tests-release",
            "parlor/ci/build-tests-release",
            [
                *cargo_arguments("test", packages, selection != "changed_path"),
                "--no-run",
            ],
            ["build-release"],
            1_800_000,
        ),
    ]
    for package in packages:
        definitions.append(
            (
                f"test-{package.removeprefix('parlor-')}",
                f"parlor/ci/test/{package}",
                ["test", "--release", "-p", package],
                ["build-tests-release"],
                1_200_000,
            )
        )

    entrypoints = []
    lanes = []
    for entrypoint_id, gate, argv, dependencies, timeout_ms in definitions:
        command = create_command(
            owner,
            executable_relative.as_posix(),
            argv,
            environment,
            files,
        )
        entrypoint = {
            "entrypoint_id": entrypoint_id,
            "entrypoint_identity": "",
            "command": command,
        }
        entrypoint["entrypoint_identity"] = sha256_json(
            {"entrypoint_id": entrypoint_id, "command": command}
        )
        entrypoints.append(entrypoint)
        lanes.append(
            {
                "lane_id": entrypoint_id,
                "owner_gate_id": gate,
                "required": True,
                "depends_on": dependencies,
                "entrypoint_id": entrypoint_id,
                "entrypoint_identity": entrypoint["entrypoint_identity"],
                "command": command,
                "timeout_ms": timeout_ms,
                "stdout_limit_bytes": MAX_OUTPUT_BYTES,
                "stderr_limit_bytes": MAX_OUTPUT_BYTES,
            }
        )

    revision = source_revision()
    declaration = {
        "schema": "ferris.owner-entrypoints/v1",
        "declaration_id": "",
        "source_revision": revision,
        "entrypoints": entrypoints,
    }
    declaration["declaration_id"] = sha256_json(
        {
            "schema": declaration["schema"],
            "source_revision": declaration["source_revision"],
            "entrypoints": declaration["entrypoints"],
        }
    )

    plan = {
        "schema": "ferris.action-plan/v1",
        "action_plan_id": "",
        "repository_id": topology["repository_id"],
        "source_revision": revision,
        "topology_id": topology["topology_id"],
        "declaration_id": declaration["declaration_id"],
        "approval_id": "",
        "lanes": lanes,
    }
    plan["action_plan_id"] = sha256_json(
        {
            "schema": plan["schema"],
            "repository_id": plan["repository_id"],
            "source_revision": plan["source_revision"],
            "topology_id": plan["topology_id"],
            "declaration_id": plan["declaration_id"],
            "lanes": plan["lanes"],
        }
    )

    expires_at = (
        datetime.datetime.now(datetime.timezone.utc)
        + datetime.timedelta(minutes=args.approval_minutes)
    ).replace(microsecond=0)
    approval = {
        "schema": "ferris.execution-approval/v1",
        "approval_id": "",
        "action_plan_id": plan["action_plan_id"],
        "principal": args.principal,
        "allowed_environment": environment,
        "expires_at": expires_at.isoformat().replace("+00:00", "Z"),
        "revoked": False,
    }
    approval["approval_id"] = sha256_json(
        {
            "schema": approval["schema"],
            "action_plan_id": approval["action_plan_id"],
            "principal": approval["principal"],
            "allowed_environment": approval["allowed_environment"],
            "expires_at": approval["expires_at"],
            "revoked": approval["revoked"],
        }
    )
    plan["approval_id"] = approval["approval_id"]

    write_identity_json(
        ferris_root / "entrypoints", declaration["declaration_id"], declaration
    )
    write_identity_json(ferris_root / "approvals", approval["approval_id"], approval)
    write_identity_json(ferris_root / "action-plans", plan["action_plan_id"], plan)
    return {
        "schema": "parlor.ferris-go-preparation/v1",
        "source_revision": revision,
        "selection": selection,
        "selected_packages": packages,
        "action_plan_id": plan["action_plan_id"],
        "approval_id": approval["approval_id"],
        "declaration_id": declaration["declaration_id"],
        "lane_count": len(lanes),
        "bound_file_count": len(files),
    }


def execute(args, preparation):
    outcome = run(
        [args.ferris, "go", "--action-plan", preparation["action_plan_id"]]
    )
    receipt = json.loads(outcome.stdout)
    receipt_path = (
        REPOSITORY_ROOT
        / ".ferris"
        / "receipts"
        / f"{receipt['receipt_id'].removeprefix('sha256:')}.json"
    )
    verification = json.loads(
        run([args.ferris, "verify", receipt_path]).stdout
    )
    if not verification["valid"]:
        raise ShadowFailure("Ferris receipt verification failed")
    return {
        **preparation,
        "receipt_id": receipt["receipt_id"],
        "receipt_path": receipt_path.relative_to(REPOSITORY_ROOT).as_posix(),
        "aggregate_status": receipt["aggregate_status"],
        "lanes": [
            {
                "lane_id": lane["lane_id"],
                "status": lane["status"],
                "elapsed_ms": lane["elapsed_ms"],
            }
            for lane in receipt["lanes"]
        ],
        "receipt_valid": True,
    }


def parse_args():
    parser = argparse.ArgumentParser(
        description="Prepare and run PARLOR's owner-defined Ferris Go shadow"
    )
    parser.add_argument("--ferris", type=Path, required=True)
    parser.add_argument("--cargo", type=Path)
    selection = parser.add_mutually_exclusive_group(required=True)
    selection.add_argument("--full", action="store_true")
    selection.add_argument(
        "--changed-path",
        action="append",
        type=Path,
        help="repository-relative changed path; repeat for multiple paths",
    )
    selection.add_argument(
        "--base-revision",
        help="select paths changed between this revision and HEAD",
    )
    parser.add_argument("--prepare-only", action="store_true")
    parser.add_argument("--principal", default="parlor-local-owner")
    parser.add_argument("--approval-minutes", type=int, default=60)
    return parser.parse_args()


def main():
    args = parse_args()
    try:
        preparation = prepare(args)
        result = preparation if args.prepare_only else execute(args, preparation)
    except (OSError, KeyError, ValueError, json.JSONDecodeError, ShadowFailure) as error:
        print(f"PARLOR Ferris Go shadow failed: {error}", file=sys.stderr)
        return 1
    print(json.dumps(result, indent=2))
    return 0 if result.get("aggregate_status", "succeeded") == "succeeded" else 1


if __name__ == "__main__":
    raise SystemExit(main())
