# Failure Format

Every oracle failure must fit this shape:

```text
WHY
  The official behavior or compatibility rule being violated.

WHAT
  The observed Stevenarella mismatch.

ANSWER
  The official answer from an oracle artifact.

EVIDENCE
  Official jar/source paths, answer path, and test command.

NEXT_OWNER
  oracle_workbench | rust_worker | direction_review | blocked
```

Write machine-readable failures to:

```text
oracle/failures/<protocol_version>/<case-id>.why-what-answer.jsonl
```

If the oracle case, contract, or harness is wrong, keep ownership in
`oracle_workbench`. If the official answer is sound and Stevenarella differs,
emit a `rust-fix-task` packet and hand off to `stevenarella-rust-worker`.
