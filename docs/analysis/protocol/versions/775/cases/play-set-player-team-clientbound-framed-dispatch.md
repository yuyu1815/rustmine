# Play Set Player Team Clientbound Framed Dispatch

## Scope

Proves Protocol 775 Play clientbound `minecraft:set_player_team` for one
official remove-mode fixture only.

## Official Evidence

| Source | Evidence |
|---|---|
| `client.jar` | `ClientboundSetPlayerTeamPacket.createRemovePacket(PlayerTeam)` |
| `client.jar` | remove branch writes only team name `String` plus method byte `1` |
| Oracle answer | `oracle/answers/775/play_set_player_team_clientbound_framed_dispatch.answer.jsonl` |

## Boundary

This case does not prove add/change/join/leave bodies, team parameters, colors,
visibility, collision, player membership, scoreboard lifecycle, or UI behavior.
