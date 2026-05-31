# Next Task

Purpose: compact recovery state for the next AI run. Keep this file short so
startup stays cheap.

## Current Location

| Field | Value |
|---|---|
| Area | Protocol 775 Play CLIENTBOUND packet support |
| Current task | The parked-row follow-up batches promoted `0x62`, `0x6e`, `0x71`, `0x86`, simple Component text rows `0x70`, `0x72`, `0x79`, `0x7a`, empty ItemStack rows `0x60`, `0x66`, `0x6c`, entity/runtime rows `0x63`, `0x83`, `0x87`, scoped default-spawn/scoreboard/NBT/game-test rows `0x61`, `0x6a`, `0x6d`, `0x7b`, `0x7e`, scoped empty/movement rows `0x7d`, `0x82`, `0x85`, `0x8a`, and combined registry-holder rows `0x75`, `0x84`, `0x8c` into jar-backed oracle packages and Stevenarella dispatch mappings. The official Protocol 775 Play CLIENTBOUND table currently ends at `0x8c`; the remaining unsupported rows are `0x64`, `0x6b`, and `0x74`. A new GameTest-backed entity fixture policy probe now proves real `ServerLevel` entities and stable bytes for all three, but those bytes are not yet normal oracle answer/test artifacts or Rust mappings. |
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
  -> keep `0x64` as the next unproven pointer only as the first unsupported row
  -> use `oracle/harness/java/scripts/run_entity_fixture_policy_probe.sh <tmp-jsonl>` as the official initialized entity fixture proof
  -> integrate the process-owned GameTest probe into oracle case/contract/answer/test-manifest generation before Rust implementation
  -> there are no further Play CLIENTBOUND rows after `0x8c` in the official table
  -> remaining Play CLIENTBOUND route is no longer blocked on finding an entity fixture policy; it is blocked on runner integration and the 3-row oracle/Rust promotion
  -> parked: `0x64`, `0x6b`, `0x74`; non-promoted branches of `0x61`, `0x6a`, `0x6d`, `0x75`, `0x7b`, `0x7d`, `0x7e`, `0x82`, `0x84`, `0x85`, `0x8a`, and `0x8c` remain unsupported
  -> create oracle packages before any Rust implementation using the GameTest fixture output, not hand-written packet bytes
  -> avoid editing unrelated oracle/log changes already in the worktree
```

## Recovery Rule

Update this file only when current location, immediate next action, blocker, or
stop boundary changes. Store evidence and durable rationale in the owning
`docs/analysis/` shard.
