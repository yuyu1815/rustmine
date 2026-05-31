# play_move_entity_pos_rot_clientbound_framed_dispatch

| Field | Value |
|---|---|
| Packet | `minecraft:move_entity_pos_rot` / `0x36` |
| Official class | `net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$PosRot` |
| Official body | `VarInt entityId`, `short xa`, `short ya`, `short za`, `byte yRot`, `byte xRot`, `boolean onGround` |
| Fixture | `entityId=124`, `xa=16`, `ya=-32`, `za=48`, `yRot=64`, `xRot=-32`, `onGround=false` |
| Answer | `oracle/answers/775/play_move_entity_pos_rot_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_move_entity_pos_rot_clientbound_framed_dispatch_matches_official_oracle_answer` |

This is packet-support evidence only. It proves official Play clientbound
framing, relative-move/rotation body bytes, dispatch, and full body consumption
for one primitive fixture. It does not prove entity existence, movement or
rotation interpolation semantics, initialized `Level` state, render readiness,
or client-load completion.
