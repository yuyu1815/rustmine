# AGENTS.md

The operator is a visual-spatial learner. Prefer maps, flow diagrams,
tables, and explicit spatial relationships.

## Goal

Build Stevenarella into a playable Minecraft-compatible client for the
user's own server.

```text
Handshake -> Login -> Configuration -> Play -> Spawn -> Move -> Interact -> Inventory -> Combat
```

Protocol-version work, oracle artifacts, tests, smoke runs, and analysis memos
are tools for client-load and playability work along that route. Protocol 775
is the current populated version shard/example, not a universal root rule.
Docs-only, review-only, protocol-only, and tooling tasks should first follow
their owning surface/domain; use corridor phase selection when the task is
about load or playability progress.

## Read First

```text
docs/ai/README.md
docs/ai/agent-ops.md
docs/next/README.md
```

`docs/ai/` is the fixed startup and routing layer. `docs/ai/agent-ops.md` is a
startup gate, not a conditional subagent reference: read it before choosing
whether parent Codex should work alone or delegate a bounded packet.
`docs/next/` is the compact recovery and next-task layer. Then load only the
analysis shard, skill, agent, or artifact named by the active task.

## Canonical AI Surfaces

```text
AGENTS.md
  thin router and hard safety rules

CONTEXT.md
  project glossary: shared vocabulary only, not AI operating rules or evidence

docs/ai/
  fixed AI startup, routing, and safety map; not mutable task state

docs/next/
  compact mutable recovery card: current location, next action, blockers,
  and stop boundary

.codex/agents/
  Codex app/CLI project-scoped subagent role/team definitions;
  parent-facing agents and rustmine_nested_* planner-to-leaf agents

.codex/skills/
  reusable procedures and role contracts; load only when the active task
  calls for them

.codex/skills/stevenarella-oracle-workbench/schemas/
  machine-checkable task, answer, and failure packets

oracle/
  version manifests, cases, contracts, generated official answers, test manifests

docs/analysis/
  AI-shared memory: human-readable evidence, client-load phase lens,
  traceability, and responsibility decisions
```

Do not add AI operating rules anywhere else.

## Required Responsibility Gate

Before changing responsibility, owner/scope, domain boundary, or cross-domain
mapping, route the update through the responsibility index and the smallest
owning detail doc:

```text
docs/analysis/responsibility/README.md
  -> docs/analysis/responsibility/<owning-detail>.md
```

## Protocol Version Guardrails

| Rule | Requirement |
|---|---|
| No prediction | Do not invent packet IDs, codecs, state transitions, registry data, or name meanings. |
| Active version first | Use the active protocol version manifest and analysis shard for protocol facts; Protocol 775 is only the current populated shard/example. |
| Client-load scope first | Do not collapse "client loads" into packet work; choose a load phase for client-load/playability tasks, not for unrelated docs, review, protocol-only, or tooling work. |
| Official answer first | Expected values must come from `client.jar`, `server.jar`, generated oracle artifacts, or named smoke/probe evidence for non-packet load phases. |
| References are witnesses | MCProtocolLib, Azalea, minecraft-data, and node-minecraft-protocol explain or cross-check; official jars win. |
| Rust worker boundary | A Rust implementation worker may read oracle artifacts but must not create or edit expected answers. |
| Oracle worker boundary | An oracle worker may read jars and write contracts/tests/answers but must not edit Stevenarella Rust implementation. |
| Names are boundaries | Do not rename, flatten, merge, or generalize official/internal names without documenting the mapping. |

## Work Selection

Use the smallest owning surface/domain that can make the change or preserve the
evidence:

```text
read active task scope
  -> choose owner from docs/analysis/responsibility/README.md
  -> read the relevant shard
  -> read docs/next/README.md only for current recovery and next-task state
  -> if client-load/playability, use the phase map as the diagnostic lens
  -> if protocol work, choose the active version manifest/case
  -> name evidence and stop boundary
  -> run the matching agent skill only when the task calls for it
  -> update the owner shard and resume recovery pointer when changed
```

## Skills

| Skill source | Use when |
|---|---|
| `.codex/skills/yuzu/SKILL.md` | Reading the operator's collaboration lens: visual maps, evidence skepticism, non-fixating docs, route naming, and helper-output trust boundaries. |
| `.codex/skills/client-load-compatibility/SKILL.md` | Mapping client-load claims, playable readiness, load phases, and phase-specific oracle/test surfaces. |
| `.codex/skills/stevenarella-oracle-workbench/SKILL.md` | Lightweight routing for oracle work: choose source policy, case-builder, failure-format, model-lane, or schema route without loading the full workflow. |
| `.codex/skills/stevenarella-oracle-case-builder/SKILL.md` | Building or updating one bounded jar-backed oracle case package: case, contract, answer, manifest, Rust oracle test, traceability row, or failure packet. |
| `.codex/skills/stevenarella-rust-worker/SKILL.md` | Implementing Rust changes from an oracle failure packet. |

## Codex Custom Agents

Project-scoped Codex app/CLI subagent role definitions live in:

```text
.codex/agents/
.codex/config.toml
```

Use them only to split or review bounded compatibility work. They do not replace
the fixed `.codex/skills/` workflows, oracle schemas, or responsibility gate.

The startup route already reads the parent/subagent gate:

```text
docs/ai/agent-ops.md
```

After that gate, use `.codex/agents/*.toml` only for a selected bounded role.
Subagents are evidence and work-package helpers only. Parent Codex remains the
user-facing owner of route choice, recovery state, and final answer.

## Model Lanes

Model and worker responsibilities live in:

```text
.codex/skills/stevenarella-oracle-workbench/references/model-lanes.toml
```

Do not let a lower-capacity extraction lane choose product route, protocol
meaning, or implementation scope.

## Before Ending

Update `docs/next/README.md` only when the current location, next action,
blocker, or recovery pointer changed. Put evidence and proof-status updates in
`docs/analysis/current-evidence/` or the relevant domain shard. Do not rely on
chat history for recovery.
