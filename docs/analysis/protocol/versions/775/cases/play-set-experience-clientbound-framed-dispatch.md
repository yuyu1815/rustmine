# play_set_experience_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundSetExperiencePacket(float, int, int)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x67
    -> oracle/answers/775/play_set_experience_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:set_experience` / `0x67` |
| Official class | `net.minecraft.network.protocol.game.ClientboundSetExperiencePacket` |
| Official body | experience progress float, level VarInt, total experience VarInt |
| Fixture | progress `0.625`, level `42`, total experience `9876` |
| Answer | `oracle/answers/775/play_set_experience_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_experience_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive XP
fixture. It does not prove player XP state, UI behavior, or client-load
completion.
