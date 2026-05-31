# play_ping_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundPingPacket(int)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x3d
    -> oracle/answers/775/play_ping_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Case id | `play_ping_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundPingPacket(int)`; `ClientboundPingPacket.STREAM_CODEC`; `FriendlyByteBuf.readInt/writeInt`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundPingPacket)`; `ClientCommonPacketListener.handlePing(ClientboundPingPacket)` |
| Generated answer | `oracle/answers/775/play_ping_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_ping_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_ping_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This proves Play clientbound `minecraft:ping` / `0x3d` packet framing and
dispatch/decode for one primitive `int` id fixture only. It does not prove
runtime pong response behavior, Play entry, render readiness, or client-load
completion.
