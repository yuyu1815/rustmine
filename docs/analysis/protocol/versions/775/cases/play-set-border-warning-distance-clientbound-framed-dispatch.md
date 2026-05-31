# play_set_border_warning_distance_clientbound_framed_dispatch

## Spatial Map

```text
client.jar WorldBorder.setWarningBlocks -> ClientboundSetBorderWarningDistancePacket(WorldBorder)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x5c
    -> oracle/answers/775/play_set_border_warning_distance_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:set_border_warning_distance` / `0x5c` |
| Official class | `net.minecraft.network.protocol.game.ClientboundSetBorderWarningDistancePacket` |
| Official body | warning distance blocks VarInt |
| Fixture | warning blocks `7` |
| Answer | `oracle/answers/775/play_set_border_warning_distance_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_border_warning_distance_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive
warning-distance fixture. It does not prove warning UI behavior, world-border
runtime behavior, render readiness, or client-load completion.
