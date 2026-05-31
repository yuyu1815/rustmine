# Next Task

Purpose: compact recovery state for the next AI run. Keep this file short so
startup stays cheap.

## Current Location

| Field | Value |
|---|---|
| Area | Protocol 775 Play CLIENTBOUND packet support |
| Current task | The `0x65` / `0x67` / `0x68` / `0x69` / `0x6f` safe batch now has jar-backed oracle packages and Stevenarella dispatch mappings. The immediate next unproven pointer remains `0x60`, but `0x60`-`0x64`, `0x66`, and `0x70`-`0x72` stay parked unless a future official-source pass names safe fixtures. |
| Last touched | `docs/analysis/protocol/versions/775/`, `oracle/`, `stevenarella/protocol/src/protocol/versions/v26_1_2.rs`, `docs/next/` |
| Stop boundary | Do not implement YELLOW rows from names or previous-version witnesses. Do not stage unrelated logs or timestamp-only answer regeneration. |

## Read Next

```text
AGENTS.md
  -> docs/ai/README.md
  -> docs/ai/agent-ops.md
  -> docs/next/README.md
  -> docs/analysis/protocol/README.md
  -> docs/analysis/protocol/versions/775/README.md
  -> docs/analysis/protocol/versions/775/play-clientbound-deferred.md
```

## Immediate Next Action

```text
For the next Protocol 775 Play CLIENTBOUND task:
  -> keep `0x60` as the next unproven pointer
  -> do official-source cartography before selecting another batch
  -> keep `0x60`-`0x64`, `0x66`, and `0x70`-`0x72` YELLOW unless official codec evidence proves a safe fixture
  -> likely inspect `0x73` onward for the next GREEN/BLUE candidates
  -> create oracle packages before any Rust implementation
  -> avoid editing unrelated oracle/log changes already in the worktree
```

## Recovery Rule

Update this file only when current location, immediate next action, blocker, or
stop boundary changes. Store evidence and durable rationale in the owning
`docs/analysis/` shard.
