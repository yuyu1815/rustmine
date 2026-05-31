# Next Task

Purpose: compact recovery state for the next AI run. Keep this file short so
startup stays cheap.

## Current Location

| Field | Value |
|---|---|
| Area | Protocol 775 Play CLIENTBOUND packet support |
| Current task | The `0x77` / `0x78` / `0x7f` / `0x80` / `0x81` safe follow-up batch now has jar-backed oracle packages and Stevenarella dispatch mappings. The immediate next unproven pointer remains `0x60`, but `0x60`-`0x64`, `0x66`, `0x70`-`0x72`, `0x74`-`0x75`, `0x79`-`0x7b`, `0x7d`-`0x7e`, and `0x82`-`0x86` stay parked unless a future official-source pass names safe fixtures. |
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
  -> do not disturb the parked YELLOW rows without official fixture evidence
  -> the next contiguous unproven region after `0x86` has read-only official-source cartography for `0x87`-`0x8c`
  -> keep parked YELLOW rows parked unless official codec evidence proves a safe fixture
  -> likely inspect safe follow-ups `0x88` custom_report_details empty map, `0x89` server_links empty list, and `0x8b` clear_dialog singleton for the next GREEN/BLUE candidates
  -> create oracle packages before any Rust implementation
  -> avoid editing unrelated oracle/log changes already in the worktree
```

## Recovery Rule

Update this file only when current location, immediate next action, blocker, or
stop boundary changes. Store evidence and durable rationale in the owning
`docs/analysis/` shard.
