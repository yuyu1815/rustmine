# Next Task

Purpose: compact recovery state for the next AI run. Keep this file short so
startup stays cheap.

## Current Location

| Field | Value |
|---|---|
| Area | Stevenarella implementation structure cleanup |
| Current task | KISS-first folder/file split for protocol version organization. The first committed slice made `stevenarella/protocol/src/protocol/versions/` the module home and gave Protocol 775 (`v26_1_2`) a packet subfolder. Login, Configuration, and Play clientbound packet decode now live under `v26_1_2/internal_protocol/`; Configuration direction readers are split into `serverbound` and `clientbound`, Configuration finish-configuration handling is split into a focused shared submodule, Configuration clientbound custom-payload, custom-report-details, keep-alive/ping, update, resource-pack, dialog, cookie, server-links, code-of-conduct, select-known-packs, transfer, registry-data, and disconnect/reset-chat handling are split into focused submodules, Configuration serverbound custom-click-action, accept-code-of-conduct, keep-alive, and custom-payload handling are split into focused submodules, and Play clientbound scoreboard, sound, update, entity, text, dialog, waypoint, set-time, set-default-spawn-position, set-cursor-item, set-player-inventory, tag-query, test-instance-block-status, disconnect, custom-report-details, and server-links handling are split into focused submodules. |
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
  -> start from the committed Login, Configuration direction, Configuration finish-configuration, Configuration clientbound custom-payload, Configuration clientbound custom-report-details, Configuration clientbound keep-alive/ping, Configuration clientbound update, Configuration clientbound resource-pack, Configuration clientbound dialog, Configuration clientbound cookie, Configuration clientbound server-links, Configuration clientbound code-of-conduct, Configuration clientbound select-known-packs, Configuration clientbound transfer, Configuration clientbound registry-data, Configuration clientbound disconnect/reset-chat, Configuration serverbound custom-click-action, Configuration serverbound accept-code-of-conduct, Configuration serverbound keep-alive, Configuration serverbound custom-payload, Play clientbound, Play scoreboard, Play sound, Play update, Play entity, Play text, Play dialog, Play waypoint, Play set-time, Play set-default-spawn-position, Play set-cursor-item, Play set-player-inventory, Play tag-query, Play test-instance-block-status, Play disconnect, Play custom-report-details, and Play server-links internal-protocol extraction
  -> choose one small owner at a time inside `stevenarella/protocol/src/protocol/versions/v26_1_2/`
  -> next likely target is splitting large internal-protocol files only if it improves reviewability; avoid generic `decode`, `helpers`, `utils`, unclear abbreviations, or packet names that overfit one numeric protocol version
  -> keep packet IDs, packet shapes, and codec behavior unchanged unless the slice explicitly justifies a behavior edit
  -> run `cargo fmt --check`, `cargo check`, and `cargo test` in the affected crate before committing
  -> after consuming any planner, implementation, oracle, docs, mapper, or review result, delete/clear or discard that agent session and cache; never reuse it for the next batch
```

## Recovery Rule

Update this file only when current location, immediate next action, blocker, or
stop boundary changes. Store evidence and durable rationale in the owning
`docs/analysis/` shard.
