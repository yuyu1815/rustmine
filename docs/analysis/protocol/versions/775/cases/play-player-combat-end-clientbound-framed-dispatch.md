# play_player_combat_end_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundPlayerCombatEndPacket(int)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x42
    -> oracle/answers/775/play_player_combat_end_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:player_combat_end` / `0x42` |
| Official class | `net.minecraft.network.protocol.game.ClientboundPlayerCombatEndPacket` |
| Official body | duration VarInt |
| Fixture | primitive duration `123` |
| Answer | `oracle/answers/775/play_player_combat_end_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_player_combat_end_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive
duration fixture. It does not prove combat runtime behavior, entity state,
render readiness, or client-load completion.
