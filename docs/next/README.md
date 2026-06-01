# Next Task

Purpose: compact recovery state for the next AI run. Keep this file short so
startup stays cheap.

## Current Location

| Field | Value |
|---|---|
| Area | Stevenarella implementation structure cleanup |
| Current task | KISS-first folder/file split for protocol version organization. The first committed slice made `stevenarella/protocol/src/protocol/versions/` the module home and gave Protocol 775 (`v26_1_2`) a packet subfolder. Login and Configuration packet decode now live under `v26_1_2/internal_protocol/` behind specifically named internal-protocol dispatch functions. |
| Last touched | `stevenarella/protocol/src/protocol/versions/`, `docs/next/` |
| Stop boundary | Keep each slice mechanical and reviewable. Do not mix packet ID changes, codec changes, or broad `packet.rs` / `mapped_packet.rs` restructuring into the same commit. Do not stage unrelated logs or generated target files. |

## Read Next

```text
AGENTS.md
  -> docs/ai/README.md
  -> docs/ai/agent-ops.md
  -> docs/next/README.md
  -> docs/analysis/responsibility/README.md
  -> docs/analysis/responsibility/checkout-under-test.md
  -> stevenarella/protocol/src/protocol/
```

## Immediate Next Action

```text
For the next structure-cleanup slice:
  -> start from the committed Login and Configuration internal-protocol decode extraction
  -> choose one small owner at a time inside `stevenarella/protocol/src/protocol/versions/v26_1_2/`
  -> prefer internal-protocol names such as `internal_protocol/play_clientbound.rs`; avoid generic `decode`, `helpers`, `utils`, unclear abbreviations, or packet names that overfit one numeric protocol version
  -> keep packet IDs, packet shapes, and codec behavior unchanged unless the slice explicitly justifies a behavior edit
  -> run `cargo fmt --check`, `cargo check`, and `cargo test` in the affected crate before committing
  -> after consuming any planner, implementation, oracle, docs, mapper, or review result, delete/clear or discard that agent session and cache; never reuse it for the next batch
```

## Recovery Rule

Update this file only when current location, immediate next action, blocker, or
stop boundary changes. Store evidence and durable rationale in the owning
`docs/analysis/` shard.
