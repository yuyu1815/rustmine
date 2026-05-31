# Play Set Default Spawn Position Clientbound Framed Dispatch

## Scope

Proves Protocol 775 Play clientbound `minecraft:set_default_spawn_position`
for the official `LevelData.RespawnData.DEFAULT` fixture only.

## Official Evidence

| Source | Evidence |
|---|---|
| `client.jar` | `ClientboundSetDefaultSpawnPositionPacket(LevelData.RespawnData)` |
| `client.jar` | `LevelData.RespawnData.STREAM_CODEC` uses `GlobalPos.STREAM_CODEC`, yaw `FLOAT`, and pitch `FLOAT` |
| Oracle answer | `oracle/answers/775/play_set_default_spawn_position_clientbound_framed_dispatch.answer.jsonl` |

## Boundary

This case does not prove respawn behavior, compass state, player state, world
state, or non-default dimension handling.
