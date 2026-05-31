# play_set_health_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundSetHealthPacket(float, int, float)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x68
    -> oracle/answers/775/play_set_health_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:set_health` / `0x68` |
| Official class | `net.minecraft.network.protocol.game.ClientboundSetHealthPacket` |
| Official body | health float, food VarInt, saturation float |
| Fixture | health `18.5`, food `17`, saturation `4.25` |
| Answer | `oracle/answers/775/play_set_health_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_health_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive
health/food/saturation fixture. It does not prove player health state, UI
behavior, or client-load completion.
