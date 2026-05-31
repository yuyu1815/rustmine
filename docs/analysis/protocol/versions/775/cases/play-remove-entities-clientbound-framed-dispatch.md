# play_remove_entities_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundRemoveEntitiesPacket(int...)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x4d
    -> oracle/answers/775/play_remove_entities_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:remove_entities` / `0x4d` |
| Official class | `net.minecraft.network.protocol.game.ClientboundRemoveEntitiesPacket` |
| Official body | entity-id list through `writeIntIdList` / `readIntIdList` |
| Fixture | primitive entity ids `[123, 4567]` |
| Answer | `oracle/answers/775/play_remove_entities_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_remove_entities_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive
entity-id list fixture. It does not prove entity existence, world state,
render readiness, or client-load completion.
