# Responsibility

Date: 2026-05-30

Purpose: keep AI/workbench responsibility decisions outside `stevenarella/`.
The `stevenarella/` directory is a reset-prone checkout under test, not the
home for persistent AI docs, reset-proof compatibility artifacts, oracle tests,
or traceability.

## Reset Boundary

```text
project root
  -> owns AI docs, oracle artifacts, generated answers, manifests
  -> owns reset-proof compatibility artifacts/tests that must survive checkout resets

stevenarella/
  -> reset-prone code under test
  -> may be changed only by a bounded Rust implementation task
  -> may contain crate-local tests for crate-local invariants when a Rust task allows them
  -> must not own persistent oracle tests, reset-proof compatibility artifacts, or AI operating docs
```

## Area Index

| Area | Detail | Owner/scope |
|---|---|---|
| Agent operations | [agent-ops.md](agent-ops.md) | `docs/ai/`, `.codex/skills/`, `.codex/agents/`, model lanes, schemas |
| Oracle factory | [oracle-factory.md](oracle-factory.md) | `oracle/`, jar-backed answers, test manifests, reset-proof compatibility tests |
| Checkout under test | [checkout-under-test.md](checkout-under-test.md) | `stevenarella/` as reset-prone implementation under test |
| Client-load evidence | [../client-load/README.md](../client-load/README.md) | phase lens and phase-specific proof state |
| Current evidence | [../current-evidence/README.md](../current-evidence/README.md) | proof state and structural observations |
| Protocol version traceability | [../protocol/README.md](../protocol/README.md) | versioned official source -> oracle artifact -> reset-proof test -> internal owner; 775 is the current populated shard |

## Responsibility Gate

Before changing responsibility, owner/scope, domain boundary, or cross-domain
mapping, update this shard and the relevant detail file.
