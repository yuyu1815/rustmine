# play_entity_position_sync_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:entity_position_sync` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar PositionMoveRotation primitive fixture
  -> ClientboundEntityPositionSyncPacket(int, PositionMoveRotation, boolean)
    -> ClientboundEntityPositionSyncPacket.STREAM_CODEC
      -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x23
        -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
          -> oracle/answers/775/play_entity_position_sync_clientbound_framed_dispatch.answer.jsonl
            -> oracle/test-manifests/775/play_entity_position_sync_clientbound_framed_dispatch.test-manifest.json
              -> oracle/rust-tests/tests/oracle_contracts.rs
                -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_entity_position_sync_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundEntityPositionSyncPacket(int, PositionMoveRotation, boolean)`; `ClientboundEntityPositionSyncPacket.STREAM_CODEC`; `PositionMoveRotation.STREAM_CODEC`; `Vec3.STREAM_CODEC`; `ByteBufCodecs.VAR_INT`; `ByteBufCodecs.BOOL`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundEntityPositionSyncPacket)` |
| Generated answer | `oracle/answers/775/play_entity_position_sync_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_entity_position_sync_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_entity_position_sync_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Official Body Shape

| Order | Field | Fixture value |
|---|---|---|
| 1 | `id` via `ByteBufCodecs.VAR_INT` | `123` |
| 2 | `position` via `Vec3.STREAM_CODEC` | `1.25, 64.5, -2.75` |
| 3 | `deltaMovement` via `Vec3.STREAM_CODEC` | `0.125, 0.0, -0.25` |
| 4 | `yRot` / `xRot` as floats | `45.0`, `-10.0` |
| 5 | `onGround` via `ByteBufCodecs.BOOL` | `true` |

The generated official frame is:

```text
237b3ff40000000000004050200000000000c0060000000000003fc00000000000000000000000000000bfd000000000000042340000c120000001
```

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
`ClientboundEntityPositionSyncPacket` primitive fixture only. It does not
prove initialized `Entity`/`Level` behavior, entity existence, spawn readiness,
render readiness, or client-load completion.
