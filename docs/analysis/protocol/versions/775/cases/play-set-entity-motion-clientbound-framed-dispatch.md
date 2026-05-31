# play_set_entity_motion_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundSetEntityMotionPacket(int, Vec3)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x65
    -> oracle/answers/775/play_set_entity_motion_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:set_entity_motion` / `0x65` |
| Official class | `net.minecraft.network.protocol.game.ClientboundSetEntityMotionPacket` |
| Official body | entity id VarInt, Vec3.LP_STREAM_CODEC movement |
| Fixture | entity id `12345`, movement `(1.25, -0.5, 0.125)` |
| Answer | `oracle/answers/775/play_set_entity_motion_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_entity_motion_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive
entity id plus movement fixture. It does not prove entity existence, movement
semantics, world state, render readiness, or client-load completion.
