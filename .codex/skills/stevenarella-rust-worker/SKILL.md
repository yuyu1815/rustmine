---
name: stevenarella-rust-worker
description: Implement bounded Rust changes from versioned oracle failures without reading jars or changing expected answers. Use when a why/what/answer packet or rust-fix-task asks Stevenarella to conform to an existing oracle artifact.
---

# Stevenarella Rust Worker

## Owner

```text
oracle failure packet
  -> bounded Rust implementation
  -> named Rust test
```

This skill may edit only the `allowed_write_scope` in the task packet.

## Workflow

1. Read the `rust-fix-task` packet.
2. Read the named contract and answer artifacts.
3. Do not read `client.jar`, `server.jar`, decompiled jar source, or reference repositories.
4. Do not edit `oracle/contracts/**`, `oracle/answers/**`, or `.codex/skills/stevenarella-oracle-workbench/schemas/**`.
5. Change only the allowed Rust files.
6. Use reset-proof project-level oracle tests under `oracle/rust-tests/` as the compatibility default.
7. Keep crate-local tests valid for crate-local invariants, public APIs, and integration behavior inside `allowed_write_scope`.
8. Run the named failing test.
9. Stop when the test passes or the task packet is insufficient.

## Forbidden

```text
creating expected bytes
changing oracle answers
choosing packet facts
broad protocol research
expanding write scope
```

## Output

```text
scope:
files_changed:
test:
result:
blocker:
```
