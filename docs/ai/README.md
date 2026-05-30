# AI Orientation

`docs/ai/` gives reading routes, not conclusions. Use it to recover where to
look next, then read the canonical owner for any durable fact, proof status, or
decision.

## Read Route

```text
AGENTS.md
  -> docs/ai/00-RESUME.md
  -> docs/ai/README.md
  -> canonical owner below
  -> active skill, agent, artifact, or shard
```

## Route Map

| Need | Read |
|---|---|
| Project glossary | [../../CONTEXT.md](../../CONTEXT.md) |
| Current recovery pointer | [00-RESUME.md](00-RESUME.md) |
| Analysis route map | [../analysis/README.md](../analysis/README.md) |
| Current evidence route | [../analysis/current-evidence/README.md](../analysis/current-evidence/README.md) |
| Client-load phase route | [../analysis/client-load/README.md](../analysis/client-load/README.md) |
| Protocol route | [../analysis/protocol/README.md](../analysis/protocol/README.md) |
| Responsibility route | [../analysis/responsibility/README.md](../analysis/responsibility/README.md) |
| Agent-ops responsibility | [../analysis/responsibility/agent-ops.md](../analysis/responsibility/agent-ops.md) |
| Operator collaboration lens | [../../.codex/skills/yuzu/SKILL.md](../../.codex/skills/yuzu/SKILL.md) |
| Fixed workflows and lenses | [../../.codex/skills/](../../.codex/skills/) |
| Codex custom agents | [../../.codex/agents/](../../.codex/agents/) |
| Source policy | [../../.codex/skills/stevenarella-oracle-workbench/references/source-policy.md](../../.codex/skills/stevenarella-oracle-workbench/references/source-policy.md) |
| Oracle version manifests | [../../oracle/versions/](../../oracle/versions/) |
| Oracle/task schemas | [../../.codex/skills/stevenarella-oracle-workbench/schemas/](../../.codex/skills/stevenarella-oracle-workbench/schemas/) |
| Subagent task schema | [../../.codex/skills/stevenarella-oracle-workbench/schemas/subagent-task.schema.json](../../.codex/skills/stevenarella-oracle-workbench/schemas/subagent-task.schema.json) |
| Rust fix task schema | [../../.codex/skills/stevenarella-oracle-workbench/schemas/rust-fix-task.schema.json](../../.codex/skills/stevenarella-oracle-workbench/schemas/rust-fix-task.schema.json) |

Version-specific facts route through the manifest directory and
`docs/analysis/protocol/versions/`. Do not infer that one populated version is
always the active target for a later task.

## Spatial Map

```text
docs/ai/
  README.md
    -> route map
      -> canonical owner
        -> active shard, workflow, lens, agent role, or schema

  00-RESUME.md
    -> current location and recovery pointer only

CONTEXT.md
  -> project vocabulary only
    -> not workflow, evidence, or current state
```

Do not add path-only pointer files here. Add a new `docs/ai/` file only when it
carries orientation value beyond listing paths.
