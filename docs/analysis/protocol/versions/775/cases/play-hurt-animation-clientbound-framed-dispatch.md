# play_hurt_animation_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:hurt_animation` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundHurtAnimationPacket(123, 45.5)
  -> ClientboundHurtAnimationPacket.STREAM_CODEC
    -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x2a
      -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
        -> oracle/answers/775/play_hurt_animation_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/play_hurt_animation_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_hurt_animation_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundHurtAnimationPacket(int, float)`; `ClientboundHurtAnimationPacket.STREAM_CODEC`; `FriendlyByteBuf.readVarInt/writeVarInt`; `FriendlyByteBuf.readFloat/writeFloat`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundHurtAnimationPacket)` |
| Generated answer | `oracle/answers/775/play_hurt_animation_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_hurt_animation_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_hurt_animation_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Official Body Shape

| Order | Field | Fixture value |
|---|---|---|
| 1 | `id` as VarInt entity id | `123` |
| 2 | `yaw` as float | `45.5` |

The generated official frame is:

```text
2a7b42360000
```

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
`ClientboundHurtAnimationPacket` primitive fixture only. It does not prove
entity existence, hurt animation semantics, world state, render readiness, or
client-load completion.
