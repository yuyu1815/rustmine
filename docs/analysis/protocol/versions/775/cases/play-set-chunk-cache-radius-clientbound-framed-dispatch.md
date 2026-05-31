# play_set_chunk_cache_radius_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundSetChunkCacheRadiusPacket(int)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x5f
    -> oracle/answers/775/play_set_chunk_cache_radius_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:set_chunk_cache_radius` / `0x5f` |
| Official class | `net.minecraft.network.protocol.game.ClientboundSetChunkCacheRadiusPacket` |
| Official body | radius VarInt |
| Fixture | radius `12` |
| Answer | `oracle/answers/775/play_set_chunk_cache_radius_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_chunk_cache_radius_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive
radius fixture. It does not prove chunk loading, world hydration, render
readiness, or client-load completion.
