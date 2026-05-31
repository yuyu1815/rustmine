# play_move_vehicle_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundMoveVehiclePacket(Vec3, float, float)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x39
    -> oracle/answers/775/play_move_vehicle_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Case id | `play_move_vehicle_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundMoveVehiclePacket(Vec3, float, float)`; `ClientboundMoveVehiclePacket.STREAM_CODEC`; `Vec3.STREAM_CODEC`; `ByteBufCodecs.FLOAT`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundMoveVehiclePacket)`; `ClientGamePacketListener.handleMoveVehicle(ClientboundMoveVehiclePacket)` |
| Generated answer | `oracle/answers/775/play_move_vehicle_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_move_vehicle_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_move_vehicle_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This proves Play clientbound `minecraft:move_vehicle` / `0x39` packet
framing and dispatch/decode for one primitive vehicle position/rotation
fixture only. It does not prove vehicle existence, movement semantics,
initialized Level state, render readiness, or client-load completion.
