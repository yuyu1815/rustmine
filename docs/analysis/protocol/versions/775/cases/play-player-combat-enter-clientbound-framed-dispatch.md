# play_player_combat_enter_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundPlayerCombatEnterPacket.INSTANCE
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x43
    -> oracle/answers/775/play_player_combat_enter_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:player_combat_enter` / `0x43` |
| Official class | `net.minecraft.network.protocol.game.ClientboundPlayerCombatEnterPacket` |
| Official body | singleton unit codec, empty body |
| Fixture | `ClientboundPlayerCombatEnterPacket.INSTANCE` |
| Answer | `oracle/answers/775/play_player_combat_enter_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_player_combat_enter_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, empty body, dispatch, and full body consumption for the singleton
fixture. It does not prove combat runtime behavior, entity state, render
readiness, or client-load completion.
