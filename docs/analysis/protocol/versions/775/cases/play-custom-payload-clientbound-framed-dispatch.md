# play_custom_payload_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound `minecraft:custom_payload`
packet id/body contract as a reset-proof packet-support slice.

```text
client.jar BrandPayload("rustmine-play-oracle-brand")
  -> ClientboundCustomPayloadPacket(CustomPacketPayload)
    -> ClientboundCustomPayloadPacket.GAMEPLAY_STREAM_CODEC
      -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x18
        -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
          -> oracle/answers/775/play_custom_payload_clientbound_framed_dispatch.answer.jsonl
            -> oracle/test-manifests/775/play_custom_payload_clientbound_framed_dispatch.test-manifest.json
              -> oracle/rust-tests/tests/oracle_contracts.rs
                -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_custom_payload_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `BrandPayload(String)`; `BrandPayload.STREAM_CODEC`; `ClientboundCustomPayloadPacket(CustomPacketPayload)`; `ClientboundCustomPayloadPacket.GAMEPLAY_STREAM_CODEC`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundCustomPayloadPacket)` |
| Generated answer | `oracle/answers/775/play_custom_payload_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_custom_payload_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_custom_payload_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Official Body Shape

| Order | Field | Fixture value |
|---|---|---|
| 1 | Custom payload id | `minecraft:brand` |
| 2 | BrandPayload body | `rustmine-play-oracle-brand` |

The generated official frame is:

```text
180f6d696e6563726166743a6272616e641a727573746d696e652d706c61792d6f7261636c652d6272616e64
```

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
`ClientboundCustomPayloadPacket` BrandPayload fixture only. It does not prove
arbitrary plugin-channel handling, payload routing policy, runtime Play entry,
world load, spawn readiness, render readiness, or client-load completion.
