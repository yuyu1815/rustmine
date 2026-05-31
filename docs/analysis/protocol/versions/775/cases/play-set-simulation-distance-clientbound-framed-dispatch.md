# play_set_simulation_distance_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundSetSimulationDistancePacket(int)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x6f
    -> oracle/answers/775/play_set_simulation_distance_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Packet | `minecraft:set_simulation_distance` / `0x6f` |
| Official class | `net.minecraft.network.protocol.game.ClientboundSetSimulationDistancePacket` |
| Official body | simulation distance VarInt |
| Fixture | simulation distance `10` |
| Answer | `oracle/answers/775/play_set_simulation_distance_clientbound_framed_dispatch.answer.jsonl` |
| Rust proof | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_simulation_distance_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This is packet-support evidence only. It proves official Play clientbound
framing, body bytes, dispatch, and full body consumption for one primitive
simulation distance fixture. It does not prove world ticking, entity
simulation behavior, or client-load completion.
