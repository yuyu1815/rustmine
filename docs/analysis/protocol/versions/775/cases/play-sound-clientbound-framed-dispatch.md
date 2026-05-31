# play_sound_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:sound` framed
dispatch/decode for one bounded official SoundEvent holder fixture.

```text
client.jar SoundEvents.AMBIENT_CAVE
  -> ClientboundSoundPacket(... SoundSource.MASTER ...)
    -> SoundEvent.STREAM_CODEC holder bytes
      -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x75
        -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_sound_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundSoundPacket(Holder<SoundEvent>, SoundSource, double, double, double, float, float, long)`; `SoundEvents.AMBIENT_CAVE`; `SoundEvent.STREAM_CODEC`; `ByteBufCodecs.holder`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_sound_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_sound_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x75` mapping |

Stop boundary: this is packet framing and body-shape evidence only. It does
not prove arbitrary SoundEvent holders, world sound playback, sound asset
readiness, or client-load completion.
