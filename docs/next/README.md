# Next Task

Purpose: compact recovery state for the next AI run. Keep this file short so
startup stays cheap.

## Current Location

| Field | Value |
|---|---|
| Area | Agent operations topology |
| Current task | Agent flow now distinguishes parent-facing agents from `rustmine_nested_*` planner-to-leaf agents. Parent Codex delegates to Lead/planner; Lead creates validated `context_capsule` packets for at most two nested leaves in one batch; nested leaves return detail to Lead; Lead returns compact status and `reported_checks` only to Parent Codex. For direct parent-to-worker delegation, use validated `worker-capsule/v1` packets so workers read capsule context instead of broad startup docs. For documentation-update churn, use `rustmine_nested_docs_rewriter` with supplied wording and check only write mistakes/scope drift after it writes. |
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
  -> validate any planner-to-leaf context_capsule against context-capsule.schema.json before spawning
  -> for docs update churn, send supplied wording to rustmine_nested_docs_rewriter and review only write mistakes, formatting, link/path breakage, duplicate/missing rows, and scope drift
  -> validate any direct parent-to-worker worker-capsule against worker-capsule.schema.json before spawning
  -> after any workspace-write leaf or direct write-capable agent, compare after-before git status paths against allowed_writes / allowed_write_scope
  -> avoid editing unrelated oracle/log changes already in the worktree
```

## Recovery Rule

Update this file only when current location, immediate next action, blocker, or
stop boundary changes. Store evidence and durable rationale in the owning
`docs/analysis/` shard.
