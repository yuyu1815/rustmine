# play_set_subtitle_text_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:set_subtitle_text`
framed dispatch/decode for one simple trusted Component fixture.

```text
client.jar Component.literal("rustmine subtitle")
  -> ClientboundSetSubtitleTextPacket(Component)
    -> TRUSTED_STREAM_CODEC writes NBT string component bytes
      -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x70
        -> oracle answer frame/body
          -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_set_subtitle_text_clientbound_framed_dispatch` |
| Official source | `client.jar` `Component.literal(String)`; `ClientboundSetSubtitleTextPacket(Component)`; `ClientboundSetSubtitleTextPacket.STREAM_CODEC`; `ComponentSerialization.TRUSTED_STREAM_CODEC`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_set_subtitle_text_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_subtitle_text_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x70` mapping |

Stop boundary: this is packet framing and body-shape evidence only for the
simple NBT string Component produced by `Component.literal(String)`. It does
not prove rich Component forms, subtitle UI behavior, Play readiness, or
client-load completion.
