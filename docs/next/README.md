# Next Task

Purpose: compact recovery state for the next AI run. Keep this file short so
startup stays cheap.

## Current Location

| Field | Value |
|---|---|
| Area | Stevenarella 26-only readability cleanup |
| Current task | Re-started from `main` on `codex/stevenarella-26-kiss` to make the current 26 target easier to read without building a generic multi-version structure. Current slices keep behavior unchanged and move cohesive feature groups only: Configuration packet family to `protocol/packet/configuration.rs`, inventory send helpers to `protocol/packet/inventory.rs`, and packet-to-mapped conversion to `protocol/mapped_packet/packet_to_mapped.rs`. This is not a packet-by-packet or one-function-per-file split. |
| Last touched | `docs/next/`, `docs/analysis/current-evidence/structural-scan.md`, `stevenarella/protocol/src/protocol/packet.rs`, `stevenarella/protocol/src/protocol/packet/configuration.rs`, `stevenarella/protocol/src/protocol/packet/inventory.rs`, `stevenarella/protocol/src/protocol/mapped_packet.rs`, `stevenarella/protocol/src/protocol/mapped_packet/packet_to_mapped.rs` |
| Stop boundary | Prioritize 26 working/readable paths over generic version abstraction. Do not reorganize `versions.rs` / `versions/` in this slice. Do not split one packet or one function into its own file unless it is already a coherent feature boundary. Do not change packet IDs, codecs, packet shapes, or protocol behavior. |

## Read Next

```text
AGENTS.md
  -> docs/ai/README.md
  -> docs/ai/agent-ops.md
  -> docs/next/README.md
  -> docs/analysis/responsibility/README.md
  -> docs/analysis/responsibility/checkout-under-test.md
  -> docs/analysis/current-evidence/structural-scan.md
  -> stevenarella/protocol/src/protocol/
```

## Immediate Next Action

```text
For the next 26-only readability-cleanup slice:
  -> keep the route simple: 26 entry -> login -> configuration -> play -> packet families -> helpers
  -> use SRP as "one coherent reason to change", not "one function per file"
  -> use KISS to avoid extra module hops when related code is already easier to read together
  -> split by feature family such as configuration, play, movement, inventory, or entity
  -> prefer one cohesive feature group per commit with behavior unchanged
  -> for remaining thousand-line files, next folder choices should improve the 26 route map before reducing line count
  -> leave `versions.rs` and generic version-router structure alone unless 26 runtime work proves that area is the smallest necessary owner
  -> run `cargo fmt --check`, `cargo check`, and `cargo test` after each Rust slice
  -> after consuming any planner, implementation, oracle, docs, mapper, or review result, delete/clear or discard that agent session and cache; never reuse it for the next batch
```

## Recovery Rule

Update this file only when current location, immediate next action, blocker, or
stop boundary changes. Store evidence and durable rationale in the owning
`docs/analysis/` shard.
