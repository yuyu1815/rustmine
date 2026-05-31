# Responsibility

Date: 2026-05-30

Purpose: keep AI/workbench responsibility decisions outside `stevenarella/`.
The `stevenarella/` directory is a reset-prone checkout under test, not the
home for persistent AI docs, reset-proof compatibility artifacts, oracle tests,
or traceability.

## Reset Boundary

| Surface | Owns | Must not own |
|---|---|---|
| Project root | AI docs, oracle artifacts, generated answers, manifests, reset-proof compatibility artifacts/tests | reset-prone implementation checkout state |
| `stevenarella/` | code under test, bounded implementation changes, allowed crate-local tests | persistent AI docs, reset-proof compatibility artifacts, oracle tests, traceability |

## Analysis No-Flow Boundary

Responsibility shards under `docs/analysis/responsibility/` record ownership,
boundaries, and rationale. They should not become operating manuals.

| Content shape | Owner |
|---|---|
| Fixed startup route or parent-side agent operation | `docs/ai/` |
| Role-specific subagent behavior | `.codex/agents/*.toml` |
| Reusable implementation or oracle instructions | `.codex/skills/` |
| Current location, next action, blocker, stop boundary | `docs/next/README.md` |
| Responsibility, boundary, rationale, uncertainty | `docs/analysis/responsibility/` |

## Area Index

| Area | Detail | Owner/scope |
|---|---|---|
| Agent operations | `docs/analysis/responsibility/agent-ops.md` | `docs/ai/`, `docs/next/`, `.codex/agents/`, `.codex/config.toml`, agent-facing skill routes |
| Oracle factory | `docs/analysis/responsibility/oracle-factory.md` | `oracle/`, jar-backed answers, test manifests, reset-proof compatibility tests |
| Checkout under test | `docs/analysis/responsibility/checkout-under-test.md` | `stevenarella/` as reset-prone implementation under test |
| Client-load evidence | `docs/analysis/client-load/README.md` | phase lens and phase-specific proof state |
| Current evidence | `docs/analysis/current-evidence/README.md` | proof state and structural observations |
| Protocol version traceability | `docs/analysis/protocol/README.md` | versioned evidence links, oracle artifacts, reset-proof tests, and internal ownership; 775 is the current populated shard |

## Responsibility Gate

Before changing responsibility, owner/scope, domain boundary, or cross-domain
mapping, update this shard and the relevant detail file.
