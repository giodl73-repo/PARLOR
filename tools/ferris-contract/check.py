#!/usr/bin/env python3

import argparse
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile


SCRIPT_DIR = Path(__file__).resolve().parent
REPOSITORY_ROOT = SCRIPT_DIR.parents[1]
CONTRACT_PATH = SCRIPT_DIR / "contract.json"


class ContractFailure(RuntimeError):
    pass


def run(command, *, cwd, env=None):
    try:
        result = subprocess.run(
            [str(value) for value in command],
            cwd=cwd,
            env=env,
            capture_output=True,
            text=True,
            check=False,
        )
    except OSError as error:
        raise ContractFailure(
            f"command could not start: {' '.join(map(str, command))}: {error}"
        ) from error
    if result.returncode != 0:
        raise ContractFailure(
            f"command failed ({result.returncode}): {' '.join(map(str, command))}\n"
            f"stdout:\n{result.stdout}\nstderr:\n{result.stderr}"
        )
    return result


def require(actual, expected, label):
    if actual != expected:
        raise ContractFailure(f"{label}: expected {expected!r}, observed {actual!r}")


def prepare_ferris(contract, ferris_source, temporary_root):
    pin = contract["ferris"]["commit"]
    if ferris_source is None:
        source = temporary_root / "ferris"
        run(["git", "init", "--quiet", source], cwd=temporary_root)
        run(
            [
                "git",
                "-C",
                source,
                "remote",
                "add",
                "origin",
                contract["ferris"]["repository"],
            ],
            cwd=temporary_root,
        )
        run(
            ["git", "-C", source, "fetch", "--quiet", "--depth", "1", "origin", pin],
            cwd=temporary_root,
        )
        run(
            ["git", "-C", source, "checkout", "--quiet", "--detach", "FETCH_HEAD"],
            cwd=temporary_root,
        )
    else:
        source = ferris_source.resolve()

    observed_pin = run(["git", "rev-parse", "HEAD"], cwd=source).stdout.strip()
    require(observed_pin, pin, "FERRIS source revision")
    require(
        run(["git", "status", "--porcelain"], cwd=source).stdout,
        "",
        "FERRIS source cleanliness",
    )

    target = temporary_root / "target"
    run(
        [
            "cargo",
            "build",
            "--locked",
            "--manifest-path",
            source / "Cargo.toml",
            "--target-dir",
            target,
            "-p",
            "ferris-cli",
            "--bin",
            "cargo-ferris",
        ],
        cwd=source,
    )
    executable = target / "debug" / (
        "cargo-ferris.exe" if os.name == "nt" else "cargo-ferris"
    )
    if not executable.is_file():
        raise ContractFailure(f"built cargo-ferris executable is missing: {executable}")
    return executable


def invoke_validation_plan(executable, workspace_id, changed_path):
    nested_directory = REPOSITORY_ROOT / "crates" / "parlor-go" / "src"
    result = run(
        [
            executable,
            "ferris",
            "validation-plan",
            "--workspace-id",
            workspace_id,
            "--changed-path",
            REPOSITORY_ROOT / changed_path,
            "--format",
            "json",
        ],
        cwd=nested_directory,
    )
    if result.stderr:
        raise ContractFailure("cargo ferris emitted unexpected stderr")
    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError as error:
        raise ContractFailure(f"cargo ferris returned invalid JSON: {error}") from error


def validate_common(document, contract):
    ferris = contract["ferris"]
    require(document["schema"], ferris["result_schema"], "result schema")
    require(document["command_version"], ferris["command_version"], "command version")
    require(document["semantic_command_id"], "validation-plan", "semantic command")
    require(document["result_class"], "success", "result class")
    require(document["process_exit_code"], 0, "process exit code")
    require(document["diagnostics"], [], "diagnostics")

    record = document["record"]
    require(record["schema"], ferris["plan_schema"], "plan schema")
    require(record["workspace_id"], contract["workspace_id"], "workspace ID")
    require(record["executable"], False, "plan executable flag")
    require(record["selected_manifest"], "Cargo.toml", "selected manifest")
    require(record["workspace_root"], ".", "portable workspace root")
    require(record["evidence"]["owner"], "Cargo", "evidence owner")
    require(record["evidence"]["offline"], True, "offline metadata")
    require(record["evidence"]["rustup_auto_install"], False, "rustup auto-install")
    return record


def validate_owner_contract(contract):
    readme = (REPOSITORY_ROOT / "README.md").read_text(encoding="utf-8")
    for command in contract["owner_validation"]:
        if command not in readme:
            raise ContractFailure(
                f"owner validation command is no longer documented: {command}"
            )


def validate_accepted_leaf(executable, contract):
    expected = contract["accepted_leaf"]
    document = invoke_validation_plan(
        executable, contract["workspace_id"], expected["changed_path"]
    )
    record = validate_common(document, contract)
    require(len(record["inputs"]), 1, "accepted input count")
    require(
        record["inputs"][0]["disposition"],
        expected["input_disposition"],
        "accepted input disposition",
    )

    selected = sorted(
        (
            package["package"]["identity"],
            package["disposition"],
        )
        for package in record["selected_packages"]
    )
    expected_selected = sorted(
        (package["identity"], package["disposition"])
        for package in expected["selected_packages"]
    )
    require(selected, expected_selected, "selected package closure")

    activities = sorted(activity["family"] for activity in record["selected_activities"])
    require(activities, sorted(expected["activity_families"]), "selected activities")
    for activity in record["selected_activities"]:
        require(activity["owner"], "Cargo", "selected activity owner")
        require(
            activity["package_scope"],
            "selected_package_closure",
            "selected activity scope",
        )
        require(
            sorted(activity["package_identities"]),
            sorted(identity for identity, _ in expected_selected),
            "selected activity packages",
        )
    require(
        record["fallback"]["required_by_inputs"],
        False,
        "accepted fallback requirement",
    )


def validate_repository_fallback(executable, contract):
    expected = contract["repository_fallback"]
    document = invoke_validation_plan(
        executable, contract["workspace_id"], expected["changed_path"]
    )
    record = validate_common(document, contract)
    require(len(record["inputs"]), 1, "fallback input count")
    require(
        record["inputs"][0]["disposition"],
        expected["input_disposition"],
        "fallback input disposition",
    )
    require(record["selected_packages"], [], "fallback selected packages")
    require(record["selected_activities"], [], "fallback selected activities")

    fallback = record["fallback"]
    require(fallback["required_by_inputs"], True, "fallback requirement")
    require(
        sorted(package["identity"] for package in fallback["packages"]),
        sorted(expected["package_identities"]),
        "fallback package set",
    )
    activities = sorted(activity["family"] for activity in fallback["activities"])
    require(activities, sorted(expected["activity_families"]), "fallback activities")
    for activity in fallback["activities"]:
        require(activity["owner"], "Cargo", "fallback activity owner")
        require(
            activity["package_scope"],
            "full_workspace_fallback",
            "fallback activity scope",
        )
        require(
            sorted(activity["package_identities"]),
            sorted(expected["package_identities"]),
            "fallback activity packages",
        )


def parse_args():
    parser = argparse.ArgumentParser(
        description="Verify PARLOR's exact FERRIS validation-plan consumer contract."
    )
    parser.add_argument(
        "--ferris-source",
        type=Path,
        help="Use an existing exact-pin FERRIS checkout instead of fetching it.",
    )
    return parser.parse_args()


def main():
    args = parse_args()
    contract = json.loads(CONTRACT_PATH.read_text(encoding="utf-8"))
    require(
        contract["schema"],
        "parlor.ferris-consumer-contract/v1",
        "consumer contract schema",
    )
    validate_owner_contract(contract)

    with tempfile.TemporaryDirectory(prefix="parlor-ferris-contract-") as directory:
        temporary_root = Path(directory)
        executable = prepare_ferris(contract, args.ferris_source, temporary_root)
        validate_accepted_leaf(executable, contract)
        validate_repository_fallback(executable, contract)

    print(
        "PARLOR FERRIS contract passed: exact pin, parlor-go reverse cone, "
        "and repository fallback."
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ContractFailure, KeyError, TypeError, ValueError) as error:
        print(f"PARLOR FERRIS contract failed: {error}", file=sys.stderr)
        sys.exit(1)
