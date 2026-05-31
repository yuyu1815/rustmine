# play_player_abilities_clientbound_framed_dispatch

## Spatial Map

```text
client.jar Abilities + ClientboundPlayerAbilitiesPacket(Abilities)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x40
    -> oracle/answers/775/play_player_abilities_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:player_abilities` / `0x40` |
| Official class | `net.minecraft.network.protocol.game.ClientboundPlayerAbilitiesPacket` |
| Official body | flags byte, flying speed float, walking speed float |
| Fixture | invulnerable and may-fly flags with `0.05`/`0.1` speeds |
| Answer | `oracle/answers/775/play_player_abilities_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_player_abilities_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one flags/speed
fixture. It does not prove initialized player ability semantics, movement
readiness, render readiness, or client-load completion.
