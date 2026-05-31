# play_player_info_remove_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundPlayerInfoRemovePacket(List<UUID>)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x45
    -> oracle/answers/775/play_player_info_remove_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:player_info_remove` / `0x45` |
| Official class | `net.minecraft.network.protocol.game.ClientboundPlayerInfoRemovePacket` |
| Official body | VarInt-prefixed UUID list via `UUIDUtil.STREAM_CODEC` |
| Fixture | two UUIDs: `123e4567-e89b-12d3-a456-426614174045`, `00000000-0000-0000-0000-000000000045` |
| Answer | `oracle/answers/775/play_player_info_remove_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_player_info_remove_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one UUID-list
fixture. It does not prove GameProfile/session/player-list state, Play entry,
render readiness, or client-load completion.
