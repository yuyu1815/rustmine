# play_disconnect_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound `minecraft:disconnect`
packet id/body contract as a reset-proof packet-support slice.

```text
client.jar Component.literal("")
  -> ClientboundDisconnectPacket(Component)
    -> ClientboundDisconnectPacket.STREAM_CODEC
      -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x20
        -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
          -> oracle/answers/775/play_disconnect_clientbound_framed_dispatch.answer.jsonl
            -> oracle/test-manifests/775/play_disconnect_clientbound_framed_dispatch.test-manifest.json
              -> oracle/rust-tests/tests/oracle_contracts.rs
                -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_disconnect_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `Component.literal(String)`; `ClientboundDisconnectPacket(Component)`; `ClientboundDisconnectPacket.STREAM_CODEC`; `ComponentSerialization.TRUSTED_CONTEXT_FREE_STREAM_CODEC`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundDisconnectPacket)` |
| Generated answer | `oracle/answers/775/play_disconnect_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_disconnect_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_disconnect_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Official Body Shape

| Order | Field | Fixture value |
|---|---|---|
| 1 | `reason` via `ComponentSerialization.TRUSTED_CONTEXT_FREE_STREAM_CODEC` | empty literal Component |

The generated official frame is:

```text
20080000
```

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
`ClientboundDisconnectPacket` fixture only. It does not prove UI disconnect
handling, screen flow, runtime Play entry, world load, spawn readiness, render
readiness, or client-load completion.
