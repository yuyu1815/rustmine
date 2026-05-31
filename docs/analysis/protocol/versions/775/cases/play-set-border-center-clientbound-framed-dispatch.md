# play_set_border_center_clientbound_framed_dispatch

## Spatial Map

```text
client.jar WorldBorder.setCenter -> ClientboundSetBorderCenterPacket(WorldBorder)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x58
    -> oracle/answers/775/play_set_border_center_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:set_border_center` / `0x58` |
| Official class | `net.minecraft.network.protocol.game.ClientboundSetBorderCenterPacket` |
| Official body | new center X double, new center Z double |
| Fixture | new center X `12.5`, new center Z `-34.75` |
| Answer | `oracle/answers/775/play_set_border_center_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_border_center_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive
border-center fixture. It does not prove world-border runtime behavior, world
state, render readiness, or client-load completion.
