# play_set_held_slot_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundSetHeldSlotPacket(int)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x69
    -> oracle/answers/775/play_set_held_slot_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:set_held_slot` / `0x69` |
| Official class | `net.minecraft.network.protocol.game.ClientboundSetHeldSlotPacket` |
| Official body | slot VarInt |
| Fixture | slot `6` |
| Answer | `oracle/answers/775/play_set_held_slot_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_held_slot_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive
slot fixture. It does not prove inventory contents, UI behavior, or
client-load completion.
