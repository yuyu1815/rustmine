# Play Set Objective Clientbound Framed Dispatch

## Scope

Proves Protocol 775 Play clientbound `minecraft:set_objective` for one
official remove-mode fixture only.

## Official Evidence

| Source | Evidence |
|---|---|
| `client.jar` | `ClientboundSetObjectivePacket(Objective, METHOD_REMOVE)` |
| `client.jar` | remove branch writes only objective name `String` plus method byte `1` |
| Oracle answer | `oracle/answers/775/play_set_objective_clientbound_framed_dispatch.answer.jsonl` |

## Boundary

This case does not prove objective add/change bodies, display Component bytes,
render type, number format, scoreboard lifecycle, or UI behavior.
