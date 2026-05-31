# play_sound_entity_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:sound_entity`
framed dispatch/decode for one initialized official GameTest entity-sound
fixture.

```text
client.jar GameTestMainUtil server
  -> GameTestHelper.spawn(PIG) in ServerLevel
    -> SoundEvents.AMBIENT_CAVE + SoundSource.MASTER
      -> ClientboundSoundEntityPacket(... Entity ...)
        -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x74
          -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_sound_entity_clientbound_framed_dispatch` |
| Official source | `client.jar` `GameTestMainUtil.runGameTestServer(...)`; `TestFunctionLoader.registerLoader(...)`; `GameTestHelper.spawn(EntityType.PIG, Vec3)`; `SoundEvents.AMBIENT_CAVE`; `ClientboundSoundEntityPacket(Holder<SoundEvent>, SoundSource, Entity, float, float, long)`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_sound_entity_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_sound_entity_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x74` mapping |

Stop boundary: this is packet framing and body-shape evidence only for
`SoundEvents.AMBIENT_CAVE`, source `master`, source pig id `1`, volume `0.75`,
pitch `1.25`, and seed `123456789`. Other sound holders, sources, entity ids,
volume, pitch, or seed values are rejected before broader sound, entity, world,
or client-load semantics.
