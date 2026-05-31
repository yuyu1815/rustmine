# play_update_mob_effect_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:update_mob_effect`
framed dispatch/decode for one bounded official MobEffect holder fixture.

```text
client.jar MobEffects.SPEED
  -> MobEffectInstance(duration=200, amplifier=1)
    -> ClientboundUpdateMobEffectPacket(entity=12345, blend=false)
      -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x84
        -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_update_mob_effect_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundUpdateMobEffectPacket(int, MobEffectInstance, boolean)`; `MobEffects.SPEED`; `MobEffect.STREAM_CODEC`; `ByteBufCodecs.holderRegistry`; `MobEffectInstance(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_update_mob_effect_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_update_mob_effect_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x84` mapping |

Stop boundary: this is packet framing and body-shape evidence only. It does
not prove entity existence, effect application, particles, HUD behavior,
arbitrary MobEffect holders, or client-load completion.
