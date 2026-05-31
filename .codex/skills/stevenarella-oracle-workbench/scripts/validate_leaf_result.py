#!/usr/bin/env python3
"""Validate a nested leaf result against its context capsule return contract."""

from __future__ import annotations

import json
import sys


def fail(message: str) -> None:
    raise SystemExit(f"leaf result invalid: {message}")


def load_json(path: str) -> object:
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def validate(capsule: object, result: object) -> None:
    if not isinstance(capsule, dict) or not isinstance(result, dict):
        fail("capsule and result must be JSON objects")
    if result.get("schema_version") != "leaf-result/v1":
        fail("schema_version must be leaf-result/v1")
    for key in ["batch", "agent_role"]:
        if result.get(key) != capsule.get(key):
            fail(f"{key} must match the context capsule")
    if result.get("result") not in {"accepted", "revised", "blocked"}:
        fail("result must be accepted, revised, or blocked")
    reported = result.get("must_report")
    if not isinstance(reported, dict):
        fail("must_report must be an object")
    required = capsule.get("return_contract", {}).get("must_report")
    if not isinstance(required, list) or not required:
        fail("capsule return_contract.must_report must be a non-empty array")
    missing = [key for key in required if key not in reported]
    empty = [key for key in required if key in reported and reported[key] in ("", [], {}, None)]
    if missing:
        fail(f"must_report missing keys: {missing}")
    if empty:
        fail(f"must_report has empty values: {empty}")
    forbidden_claims = result.get("must_not_claim", [])
    if forbidden_claims is not None:
        if not isinstance(forbidden_claims, list):
            fail("must_not_claim must be an array when present")
        if not all(isinstance(item, str) and item for item in forbidden_claims):
            fail("must_not_claim entries must be non-empty strings")


def main(argv: list[str]) -> int:
    if len(argv) != 3:
        print("usage: validate_leaf_result.py CAPSULE_JSON LEAF_RESULT_JSON", file=sys.stderr)
        return 2
    validate(load_json(argv[1]), load_json(argv[2]))
    print("leaf result valid")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
