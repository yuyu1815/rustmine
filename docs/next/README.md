# Next Task

Purpose: compact recovery state for the next AI run. Keep this file short so
startup stays cheap.

## Current Location

| Field | Value |
|---|---|
| Area | Agent operations topology |
| Current task | Agent flow now distinguishes parent-facing agents from `rustmine_nested_*` planner-to-leaf agents. Parent Codex delegates to Lead/planner; Lead creates capsules for nested leaves; nested leaves return detail to Lead; Lead returns compact status only to Parent Codex. |
| Last touched | `AGENTS.md`, `docs/ai/`, `.codex/agents/`, `.codex/config.toml`, `docs/next/`, `docs/analysis/responsibility/` |
| Stop boundary | Do not move oracle facts, generated answers, protocol traceability, or proof status into `docs/ai/` or `docs/next/`. |

## Read Next

```text
AGENTS.md
  -> docs/ai/README.md
  -> docs/ai/agent-ops.md
  -> docs/next/README.md
  -> docs/analysis/responsibility/README.md
  -> docs/analysis/responsibility/agent-ops.md
```

## Immediate Next Action

```text
For the next real task:
  -> read docs/ai/README.md for the fixed startup route
  -> read docs/ai/agent-ops.md as the parent/subagent startup gate
  -> read this file for compact recovery state
  -> choose one owning docs/analysis shard
  -> load only the skill, agent, artifact, or shard named by the task
  -> avoid editing unrelated oracle/log changes already in the worktree
```

## Recovery Rule

Update this file only when current location, immediate next action, blocker, or
stop boundary changes. Store evidence and durable rationale in the owning
`docs/analysis/` shard.
