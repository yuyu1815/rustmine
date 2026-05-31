# Next Task

Purpose: compact recovery state for the next AI run. Keep this file short so
startup stays cheap.

## Current Location

| Field | Value |
|---|---|
| Area | Protocol 775 Play CLIENTBOUND packet support |
| Current task | The parked-row follow-up batches promoted `0x62`, `0x6e`, `0x71`, `0x86`, simple Component text rows `0x70`, `0x72`, `0x79`, `0x7a`, empty ItemStack rows `0x60`, `0x66`, `0x6c`, and entity/runtime rows `0x63`, `0x83`, `0x87` into jar-backed oracle packages and Stevenarella dispatch mappings. The official Protocol 775 Play CLIENTBOUND table currently ends at `0x8c`. The immediate next unproven pointer remains `0x61`, and remaining parked rows stay parked unless a future official-source pass names safe fixtures. |
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
  -> keep `0x61` as the next unproven pointer
  -> do not disturb the parked YELLOW rows without official fixture evidence
  -> there are no further Play CLIENTBOUND rows after `0x8c` in the official table
  -> remaining route is to return to parked rows only with exact official fixture evidence, or switch to another client-load/playability surface
  -> especially parked: `0x61`, `0x64`, `0x6a`-`0x6b`, `0x6d`, `0x74`-`0x75`, `0x7b`, `0x7d`-`0x7e`, `0x82`, `0x84`-`0x85`, `0x8a`, `0x8c`
  -> create oracle packages before any Rust implementation if another safe fixture is identified
  -> avoid editing unrelated oracle/log changes already in the worktree
```

## Recovery Rule

Update this file only when current location, immediate next action, blocker, or
stop boundary changes. Store evidence and durable rationale in the owning
`docs/analysis/` shard.
