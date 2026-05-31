# play_game_event_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound `minecraft:game_event`
packet id/body contract as a reset-proof packet-support slice.

```text
client.jar ClientboundGameEventPacket.START_RAINING
  -> ClientboundGameEventPacket(Type, float)
    -> ClientboundGameEventPacket.STREAM_CODEC
      -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x26
        -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
          -> oracle/answers/775/play_game_event_clientbound_framed_dispatch.answer.jsonl
            -> oracle/test-manifests/775/play_game_event_clientbound_framed_dispatch.test-manifest.json
              -> oracle/rust-tests/tests/oracle_contracts.rs
                -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_game_event_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundGameEventPacket(Type, float)`; `ClientboundGameEventPacket.STREAM_CODEC`; private `ClientboundGameEventPacket(FriendlyByteBuf)`; private `write(FriendlyByteBuf)`; `FriendlyByteBuf.readUnsignedByte/writeByte`; `FriendlyByteBuf.readFloat/writeFloat`; `ClientboundGameEventPacket.Type` id table; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundGameEventPacket)` |
| Generated answer | `oracle/answers/775/play_game_event_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_game_event_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_game_event_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Official Body Shape

| Order | Field | Fixture value |
|---|---|---|
| 1 | `event` Type id as unsigned byte | `START_RAINING` / `1` |
| 2 | `param` as float | `0.5` |

The generated official frame is:

```text
26013f000000
```

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
`ClientboundGameEventPacket` fixture only. It does not prove game event
semantics, initialized `Level`/player/weather state, render readiness, or
client-load completion.
