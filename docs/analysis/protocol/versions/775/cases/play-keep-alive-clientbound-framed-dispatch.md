# play_keep_alive_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound `minecraft:keep_alive`
packet id/body contract as a reset-proof packet-support slice.

```text
client.jar ClientboundKeepAlivePacket(12345)
  -> ClientboundKeepAlivePacket.STREAM_CODEC
    -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x2c
      -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
        -> oracle/answers/775/play_keep_alive_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/play_keep_alive_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_keep_alive_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundKeepAlivePacket(long)`; `ClientboundKeepAlivePacket.STREAM_CODEC`; `FriendlyByteBuf.readLong/writeLong`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundKeepAlivePacket)` |
| Generated answer | `oracle/answers/775/play_keep_alive_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_keep_alive_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_keep_alive_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Official Body Shape

| Order | Field | Fixture value |
|---|---|---|
| 1 | `id` as big-endian long | `12345` |

The generated official frame is:

```text
2c0000000000003039
```

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
`ClientboundKeepAlivePacket` primitive id fixture only. It does not prove
runtime keep-alive response behavior, Play entry, render readiness, or
client-load completion.
