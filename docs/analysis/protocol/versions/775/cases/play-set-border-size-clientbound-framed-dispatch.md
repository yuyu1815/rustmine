# play_set_border_size_clientbound_framed_dispatch

## Spatial Map

```text
client.jar WorldBorder.setSize -> ClientboundSetBorderSizePacket(WorldBorder)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x5a
    -> oracle/answers/775/play_set_border_size_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:set_border_size` / `0x5a` |
| Official class | `net.minecraft.network.protocol.game.ClientboundSetBorderSizePacket` |
| Official body | border size double |
| Fixture | size `512.25` |
| Answer | `oracle/answers/775/play_set_border_size_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_border_size_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive
border-size fixture. It does not prove world-border runtime behavior, world
state, render readiness, or client-load completion.
