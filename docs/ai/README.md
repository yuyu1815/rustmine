# AI Startup Map

`docs/ai/` is fixed startup documentation. Read it at the start of an AI turn
to choose the right route with low token cost. Do not store mutable task state,
proof status, packet facts, or next actions here.

## Read Route

```text
AGENTS.md
  -> docs/ai/README.md
  -> docs/ai/agent-ops.md
  -> docs/next/README.md
  -> one canonical owner below
  -> active skill, agent, artifact, or shard
```

`docs/ai/agent-ops.md` is always part of startup. It decides the parent/subagent
posture before owner selection, so delegation is not discovered late by
accident.

## Route Map

| Need | Read |
|---|---|
| Project glossary | `CONTEXT.md` |
| Current next task / recovery pointer | `docs/next/README.md` |
| AI shared memory route map | `docs/analysis/README.md` |
| Current evidence route | `docs/analysis/current-evidence/README.md` |
| Client-load phase route | `docs/analysis/client-load/README.md` |
| Protocol route | `docs/analysis/protocol/README.md` |
| Responsibility route | `docs/analysis/responsibility/README.md` |
| Agent operation startup rules | `docs/ai/agent-ops.md` |
| Agent-ops responsibility/evidence | `docs/analysis/responsibility/agent-ops.md` |
| Operator collaboration lens | `.codex/skills/yuzu/SKILL.md` |
| Fixed workflows and lenses | `.codex/skills` |
| Codex custom agents | `.codex/agents` |
| Oracle workbench router | `.codex/skills/stevenarella-oracle-workbench/SKILL.md` |
| Oracle case builder | `.codex/skills/stevenarella-oracle-case-builder/SKILL.md` |
| Source policy | `.codex/skills/stevenarella-oracle-workbench/references/source-policy.md` |
| Oracle version manifests | `oracle/versions` |
| Oracle/task schemas | `.codex/skills/stevenarella-oracle-workbench/schemas` |
| Context capsule schema | `.codex/skills/stevenarella-oracle-workbench/schemas/context-capsule.schema.json` |
| Rust fix task schema | `.codex/skills/stevenarella-oracle-workbench/schemas/rust-fix-task.schema.json` |

Version-specific facts route through the manifest directory and
`docs/analysis/protocol/versions/`. Do not infer that one populated version is
always the active target for a later task.

## Spatial Map

```text
docs/ai/
  README.md
    -> fixed startup map
      -> read agent-ops startup gate
        -> choose parent-only or bounded delegation posture
          -> choose one owner
            -> read only what the active task needs

  agent-ops.md
    -> fixed subagent delegation, nesting, return, and wait rules

  00-RESUME.md
    -> compatibility pointer to docs/next/README.md

docs/next/
  README.md
    -> compact mutable recovery and next-task state

docs/analysis/
  -> AI-shared memory, evidence, decisions, uncertainty, traceability

CONTEXT.md
  -> project vocabulary only
    -> not workflow, evidence, or current state
```

## Fixed Layer Rule

`docs/ai/` should change rarely. If information changes because work progressed,
put it in `docs/next/` or the owning `docs/analysis/` shard instead.

| Information | Destination |
|---|---|
| Startup route, safety posture, low-token reading map | `docs/ai/` |
| Stable subagent delegation, nesting, return, and wait rules | `docs/ai/agent-ops.md` and `.codex/agents/*.toml` |
| Current location, next action, immediate blocker, stop boundary | `docs/next/` |
| Evidence, proof state, decisions, analysis notes shared between AI runs | `docs/analysis/` |
| Versioned machine-checkable oracle facts | `oracle/` |
| Project vocabulary | `CONTEXT.md` |

## Update Destinations

Use the document that naturally owns the knowledge:

```text
durable fact changed
  -> update the smallest owning docs/analysis shard

durable vocabulary changed
  -> update CONTEXT.md

new durable area appeared
  -> add a docs/analysis shard or index route

only current location, next action, blocker, or recovery route changed
  -> update docs/next/README.md

nothing durable changed
  -> do not write docs
```

## Route Hygiene

Names are routes. When a file or directory name no longer reflects the current
concept:

1. Check references.
2. Rename or delete the stale route.
3. Prefer canonical paths that express domain shape, such as
   `protocol/versions/<version>/...`.
4. Keep compatibility pointers only when they prevent real breakage.

## Startup Token Budget

The startup route should stay spatial and short:

```text
fixed route:
  AGENTS.md
    -> docs/ai/README.md
      -> docs/ai/agent-ops.md
        -> docs/next/README.md
          -> one owning docs/analysis shard
```

Do not require fresh agents to read every skill, every agent definition, or the
full evidence history before choosing the active owner. The active task should
name the extra shard, skill, artifact, or agent role that needs to be loaded.
