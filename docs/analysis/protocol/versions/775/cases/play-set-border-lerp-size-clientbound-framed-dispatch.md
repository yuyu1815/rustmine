# play_set_border_lerp_size_clientbound_framed_dispatch

## Spatial Map

```text
client.jar WorldBorder.lerpSizeBetween -> ClientboundSetBorderLerpSizePacket(WorldBorder)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x59
    -> oracle/answers/775/play_set_border_lerp_size_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:set_border_lerp_size` / `0x59` |
| Official class | `net.minecraft.network.protocol.game.ClientboundSetBorderLerpSizePacket` |
| Official body | old size double, new size double, lerp time VarLong |
| Fixture | old size `100.0`, new size `250.5`, lerp time `12345` |
| Answer | `oracle/answers/775/play_set_border_lerp_size_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_border_lerp_size_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive
border-lerp fixture. It does not prove world-border interpolation behavior,
world state, render readiness, or client-load completion.
