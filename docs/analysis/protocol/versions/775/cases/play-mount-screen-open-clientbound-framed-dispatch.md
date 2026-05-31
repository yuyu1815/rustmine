# play_mount_screen_open_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:mount_screen_open` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundMountScreenOpenPacket(7, 5, 123)
  -> ClientboundMountScreenOpenPacket.STREAM_CODEC
    -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x29
      -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
        -> oracle/answers/775/play_mount_screen_open_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/play_mount_screen_open_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_mount_screen_open_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundMountScreenOpenPacket(int, int, int)`; `ClientboundMountScreenOpenPacket.STREAM_CODEC`; `FriendlyByteBuf.readContainerId/writeContainerId`; `FriendlyByteBuf.readVarInt/writeVarInt`; `FriendlyByteBuf.readInt/writeInt`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundMountScreenOpenPacket)` |
| Generated answer | `oracle/answers/775/play_mount_screen_open_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_mount_screen_open_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_mount_screen_open_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Official Body Shape

| Order | Field | Fixture value |
|---|---|---|
| 1 | `containerId` via `FriendlyByteBuf.writeContainerId` | `7` |
| 2 | `inventoryColumns` via VarInt | `5` |
| 3 | `entityId` via big-endian int | `123` |

The generated official frame is:

```text
2907050000007b
```

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
`ClientboundMountScreenOpenPacket` fixture only. It does not prove mount entity
existence, inventory/menu semantics, screen behavior, render readiness, or
client-load completion.
