# Next Task

Purpose: compact recovery state for the next AI run. Keep this file short so
startup stays cheap.

## Current Location

| Field | Value |
|---|---|
| Area | Protocol 775 Play CLIENTBOUND packet support |
| Current task | The parked-row follow-up batches promoted `0x62`, `0x6e`, `0x71`, `0x86`, simple Component text rows `0x70`, `0x72`, `0x79`, `0x7a`, empty ItemStack rows `0x60`, `0x66`, `0x6c`, entity/runtime rows `0x63`, `0x83`, `0x87`, scoped default-spawn/scoreboard/NBT/game-test rows `0x61`, `0x6a`, `0x6d`, `0x7b`, `0x7e`, scoped empty/movement rows `0x7d`, `0x82`, `0x85`, `0x8a`, combined registry-holder rows `0x75`, `0x84`, `0x8c`, and final GameTest entity fixture rows `0x64`, `0x6b`, `0x74` into jar-backed oracle packages and Stevenarella dispatch mappings. The official Protocol 775 Play CLIENTBOUND table currently ends at `0x8c`; all rows now have a classification and all safe/proven packet-support rows have scoped implementations. |
| Last touched | `docs/analysis/protocol/versions/775/`, `oracle/harness/java/`, `oracle/`, `stevenarella/protocol/src/protocol/versions/v26_1_2.rs`, `docs/next/` |
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
  -> do not continue broad packet insertion from row names; the official table is classified through `0x8c`
  -> use `oracle/harness/java/scripts/run_entity_fixture_policy_probe.sh <tmp-jsonl>` only when extending the official initialized entity fixture family
  -> create new oracle packages before any future Rust expansion of unsupported branches
  -> there are no further Play CLIENTBOUND rows after `0x8c` in the official table
  -> no parked packet rows remain; non-promoted branches of `0x61`, `0x64`, `0x6a`, `0x6b`, `0x6d`, `0x74`, `0x75`, `0x7b`, `0x7d`, `0x7e`, `0x82`, `0x84`, `0x85`, `0x8a`, and `0x8c` remain unsupported
  -> choose the next route from client-load/playability evidence, not more Protocol 775 Play CLIENTBOUND row completion
  -> avoid editing unrelated oracle/log changes already in the worktree
```

## Recovery Rule

Update this file only when current location, immediate next action, blocker, or
stop boundary changes. Store evidence and durable rationale in the owning
`docs/analysis/` shard.
