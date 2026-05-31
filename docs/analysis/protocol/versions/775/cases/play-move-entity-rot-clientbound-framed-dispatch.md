# play_move_entity_rot_clientbound_framed_dispatch

| Field | Value |
|---|---|
| Packet | `minecraft:move_entity_rot` / `0x38` |
| Official class | `net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Rot` |
| Official body | `VarInt entityId`, `byte yRot`, `byte xRot`, `boolean onGround` |
| Fixture | `entityId=125`, `yRot=32`, `xRot=-16`, `onGround=true` |
| Answer | `oracle/answers/775/play_move_entity_rot_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_move_entity_rot_clientbound_framed_dispatch_matches_official_oracle_answer` |

This is packet-support evidence only. It proves official Play clientbound
framing, rotation body bytes, dispatch, and full body consumption for one
primitive fixture. It does not prove entity existence, rotation interpolation
semantics, initialized `Level` state, render readiness, or client-load
completion.
