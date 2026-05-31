# play_move_entity_pos_clientbound_framed_dispatch

| Field | Value |
|---|---|
| Packet | `minecraft:move_entity_pos` / `0x35` |
| Official class | `net.minecraft.network.protocol.game.ClientboundMoveEntityPacket$Pos` |
| Official body | `VarInt entityId`, `short xa`, `short ya`, `short za`, `boolean onGround` |
| Fixture | `entityId=123`, `xa=4096`, `ya=-2048`, `za=128`, `onGround=true` |
| Answer | `oracle/answers/775/play_move_entity_pos_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_move_entity_pos_clientbound_framed_dispatch_matches_official_oracle_answer` |

This is packet-support evidence only. It proves official Play clientbound
framing, relative-move body bytes, dispatch, and full body consumption for one
primitive fixture. It does not prove entity existence, movement interpolation
semantics, initialized `Level` state, render readiness, or client-load
completion.
