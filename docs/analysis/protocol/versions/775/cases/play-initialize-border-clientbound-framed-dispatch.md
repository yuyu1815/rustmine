# play_initialize_border_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:initialize_border` packet id/body contract as a reset-proof
packet-support slice.

```text
primitive border fields
  -> ClientboundInitializeBorderPacket.STREAM_CODEC.decode(...)
    -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x2b
      -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
        -> oracle/answers/775/play_initialize_border_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/play_initialize_border_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_initialize_border_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundInitializeBorderPacket.STREAM_CODEC`; private `ClientboundInitializeBorderPacket(FriendlyByteBuf)`; private `write(FriendlyByteBuf)`; `FriendlyByteBuf.readDouble/writeDouble`; `FriendlyByteBuf.readVarLong/writeVarLong`; `FriendlyByteBuf.readVarInt/writeVarInt`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundInitializeBorderPacket)` |
| Generated answer | `oracle/answers/775/play_initialize_border_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_initialize_border_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_initialize_border_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Official Body Shape

| Order | Field | Fixture value |
|---|---|---|
| 1 | `newCenterX` as double | `12.5` |
| 2 | `newCenterZ` as double | `-7.25` |
| 3 | `oldSize` as double | `100.0` |
| 4 | `newSize` as double | `64.5` |
| 5 | `lerpTime` as VarLong | `12345` |
| 6 | `newAbsoluteMaxSize` as VarInt | `29999984` |
| 7 | `warningBlocks` as VarInt | `5` |
| 8 | `warningTime` as VarInt | `15` |

The generated official frame is:

```text
2b4029000000000000c01d00000000000040590000000000004050200000000000b960f086a70e050f
```

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
`ClientboundInitializeBorderPacket` primitive-field fixture only. It does not
prove world-border runtime behavior, world state, render readiness, or
client-load completion.
