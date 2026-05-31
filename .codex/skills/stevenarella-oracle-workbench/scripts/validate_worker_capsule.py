#!/usr/bin/env python3
"""Validate a direct parent-to-worker capsule without third-party packages."""

from __future__ import annotations

import json
import sys
from pathlib import Path, PurePosixPath


REQUIRED = {
    "schema_version",
    "worker_role",
    "objective",
    "startup_context",
    "allowed_reads",
    "allowed_writes",
    "required_evidence",
    "required_checks",
    "exit_condition",
    "stop_boundary",
    "write_policy",
    "return_contract",
}

READ_ONLY_ROLES = {
    "rustmine_test_surface_mapper",
    "rustmine_compatibility_critic",
}

RUST_ROLE = "rustmine_rust_implementer"

BROAD_PATHS = {
    ".",
    "./",
    "docs",
    "docs/",
    "docs/analysis",
    "docs/analysis/",
    ".codex",
    ".codex/",
    "stevenarella",
    "stevenarella/",
    "oracle",
    "oracle/",
}

GLOB_CHARS = set("*?[]{}")
SCHEMA_PATH = Path(__file__).resolve().parents[1] / "schemas" / "worker-capsule.schema.json"


def fail(message: str) -> None:
    raise SystemExit(f"worker capsule invalid: {message}")


def load_json(path: str) -> object:
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def load_contract_constants() -> dict[str, object]:
    schema = load_json(str(SCHEMA_PATH))
    if not isinstance(schema, dict):
        fail("worker capsule schema is not an object")
    try:
        role_enum = schema["properties"]["worker_role"]["enum"]
        baseline_props = schema["properties"]["write_policy"]["properties"]["diff_baseline"]["properties"]
    except KeyError as e:
        fail(f"worker capsule schema is missing expected key: {e}")
    return {
        "roles": set(role_enum),
        "baseline": {
            "before_command": baseline_props["before_command"]["const"],
            "after_command": baseline_props["after_command"]["const"],
            "delta_rule": baseline_props["delta_rule"]["const"],
        },
    }


def check_path(path: str) -> None:
    if not isinstance(path, str) or not path:
        fail("path must be a non-empty string")
    if path in BROAD_PATHS:
        fail(f"path is too broad: {path}")
    if path.startswith("/") or path.startswith("./"):
        fail(f"path must be normalized repo-relative: {path}")
    if "\\" in path or any(ch in path for ch in GLOB_CHARS):
        fail(f"path must not contain backslashes or glob characters: {path}")
    parts = PurePosixPath(path).parts
    if ".." in parts or "." in parts:
        fail(f"path must not contain dot segments: {path}")


def check_path_entries(entries: object, *, write: bool) -> set[str]:
    if not isinstance(entries, list):
        fail("path entry collections must be arrays")
    seen: set[str] = set()
    for entry in entries:
        if not isinstance(entry, dict):
            fail("path entries must be objects")
        required = {"path", "kind", "scope", "reason"}
        missing = required - set(entry)
        extra = set(entry) - required
        if missing:
            fail(f"path entry missing fields: {sorted(missing)}")
        if extra:
            fail(f"path entry has extra fields: {sorted(extra)}")
        path = entry["path"]
        check_path(path)
        if path in seen:
            fail(f"duplicate path entry: {path}")
        seen.add(path)
        if entry["kind"] not in {"file", "directory"}:
            fail(f"invalid path kind for {path}: {entry['kind']}")
        if entry["scope"] not in {"exact", "subtree"}:
            fail(f"invalid path scope for {path}: {entry['scope']}")
        if write and (entry["kind"] != "file" or entry["scope"] != "exact"):
            fail(f"allowed_writes must use exact file paths only: {path}")
        if not isinstance(entry["reason"], str) or not entry["reason"]:
            fail(f"path entry needs a reason: {path}")
    return seen


def validate_string_array(value: object, *, name: str) -> None:
    if not isinstance(value, list) or not value:
        fail(f"{name} must be a non-empty array")
    if len(value) != len(set(value)):
        fail(f"{name} must not contain duplicates")
    if not all(isinstance(item, str) and item for item in value):
        fail(f"{name} entries must be non-empty strings")


def validate_startup_context(value: object) -> None:
    if not isinstance(value, dict):
        fail("startup_context must be an object")
    if set(value) != {"current_location", "known_facts", "do_not_read_by_default"}:
        fail("startup_context must contain only current_location, known_facts, and do_not_read_by_default")
    if not isinstance(value["current_location"], str) or not value["current_location"]:
        fail("startup_context.current_location must be a non-empty string")
    validate_string_array(value["known_facts"], name="startup_context.known_facts")
    check_path_entries(value["do_not_read_by_default"], write=False)


def validate_capsule(capsule: object, constants: dict[str, object]) -> tuple[str, set[str]]:
    if not isinstance(capsule, dict):
        fail("top-level value must be an object")
    missing = REQUIRED - set(capsule)
    if missing:
        fail(f"missing fields: {sorted(missing)}")
    extra = set(capsule) - (REQUIRED | {"fragile_preconditions", "rust_fix_task_path"})
    if extra:
        fail(f"extra fields: {sorted(extra)}")
    if capsule["schema_version"] != "worker-capsule/v1":
        fail("schema_version must be worker-capsule/v1")

    role = capsule["worker_role"]
    if role not in constants["roles"]:
        fail(f"unknown worker_role: {role}")

    for key in ["objective", "exit_condition", "stop_boundary"]:
        if not isinstance(capsule[key], str) or not capsule[key]:
            fail(f"{key} must be a non-empty string")

    validate_startup_context(capsule["startup_context"])
    reads = check_path_entries(capsule["allowed_reads"], write=False)
    writes = check_path_entries(capsule["allowed_writes"], write=True)
    if not reads:
        fail("allowed_reads must not be empty")

    validate_string_array(capsule["required_evidence"], name="required_evidence")
    validate_string_array(capsule["required_checks"], name="required_checks")

    policy = capsule["write_policy"]
    if not isinstance(policy, dict):
        fail("write_policy must be an object")
    if set(policy) != {"mode", "post_run_diff_check", "diff_baseline"}:
        fail("write_policy must contain only mode, post_run_diff_check, and diff_baseline")
    if policy["mode"] not in {"read_only", "workspace_write"}:
        fail("write_policy.mode is invalid")
    if policy["post_run_diff_check"] is not True:
        fail("write_policy.post_run_diff_check must be true")
    if policy["diff_baseline"] != constants["baseline"]:
        fail("write_policy.diff_baseline must use the standard before/after delta rule")
    if policy["mode"] == "workspace_write" and not writes:
        fail("workspace_write capsules must name at least one allowed_writes entry")
    if policy["mode"] == "read_only" and writes:
        fail("read_only capsules must have no allowed_writes")
    if role in READ_ONLY_ROLES and (policy["mode"] != "read_only" or writes):
        fail(f"{role} must be read_only with no allowed_writes")
    if role == RUST_ROLE:
        if "rust_fix_task_path" not in capsule:
            fail(f"{RUST_ROLE} capsules must include rust_fix_task_path")
        check_path_entries([capsule["rust_fix_task_path"]], write=True)

    contract = capsule["return_contract"]
    if not isinstance(contract, dict):
        fail("return_contract must be an object")
    if set(contract) != {"must_report", "must_not_claim"}:
        fail("return_contract must contain only must_report and must_not_claim")
    validate_string_array(contract["must_report"], name="return_contract.must_report")
    validate_string_array(contract["must_not_claim"], name="return_contract.must_not_claim")

    return role, writes


def validate_rust_subset(rust_task: object, capsule_writes: set[str]) -> None:
    if not isinstance(rust_task, dict):
        fail("rust-fix-task must be an object")
    scope = rust_task.get("allowed_write_scope")
    if not isinstance(scope, list) or not all(isinstance(p, str) for p in scope):
        fail("rust-fix-task.allowed_write_scope must be a string array")
    for path in scope:
        check_path(path)
    extra = sorted(set(scope) - capsule_writes)
    if extra:
        fail(f"rust allowed_write_scope is outside worker capsule allowed_writes: {extra}")


def main(argv: list[str]) -> int:
    if len(argv) not in {2, 3}:
        print(
            "usage: validate_worker_capsule.py WORKER_CAPSULE_JSON [RUST_FIX_TASK_JSON]\n"
            "RUST_FIX_TASK_JSON is required when worker_role is rustmine_rust_implementer.",
            file=sys.stderr,
        )
        return 2
    constants = load_contract_constants()
    role, capsule_writes = validate_capsule(load_json(argv[1]), constants)
    if role == RUST_ROLE and len(argv) != 3:
        fail(f"{RUST_ROLE} validation requires RUST_FIX_TASK_JSON")
    if len(argv) == 3:
        if role != RUST_ROLE:
            fail("RUST_FIX_TASK_JSON may only be supplied for the Rust worker role")
        validate_rust_subset(load_json(argv[2]), capsule_writes)
    print("worker capsule valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
