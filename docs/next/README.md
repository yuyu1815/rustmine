# Next Task

Purpose: compact recovery state for the next AI run. Keep this file short so
startup stays cheap.

## Current Location

| Field | Value |
|---|---|
| Area | Agent operations topology |
| Current task | Startup routing now uses fixed `docs/ai/`, mutable recovery lives in `docs/next/`, and AI-shared evidence/decisions live in `docs/analysis/`. Operational flow was moved out of `docs/analysis/responsibility/agent-ops.md` into `docs/ai/README.md`. |
| Last touched | `AGENTS.md`, `docs/ai/`, `docs/next/`, `docs/analysis/responsibility/` |
| Stop boundary | Do not move oracle facts, generated answers, protocol traceability, or proof status into `docs/ai/` or `docs/next/`. |

## Read Next

```text
AGENTS.md
  -> docs/ai/README.md
  -> docs/next/README.md
  -> docs/analysis/responsibility/README.md
  -> docs/analysis/responsibility/agent-ops.md
```

## Immediate Next Action

```text
For the next real task:
  -> read docs/ai/README.md for the fixed startup route
  -> read this file for compact recovery state
  -> choose one owning docs/analysis shard
  -> load only the skill, agent, artifact, or shard named by the task
  -> avoid editing unrelated oracle/log changes already in the worktree
```

## Recovery Rule

Update this file only when current location, immediate next action, blocker, or
stop boundary changes. Store evidence and longer reasoning in the owning
`docs/analysis/` shard.
