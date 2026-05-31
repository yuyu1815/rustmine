# Play waypoint Clientbound Framed Dispatch

| Field | Value |
|---|---|
| Case | `play_waypoint_clientbound_framed_dispatch` |
| Packet | `0x8a` / `minecraft:waypoint` |
| Fixture | `ClientboundTrackedWaypointPacket.removeWaypoint(UUID)` |
| Body | `010100000000000000000000000000000123116d696e6563726166743a64656661756c740000` |
| Scope | Packet framing and dispatch/decode for official removeWaypoint empty fixture only |

## Evidence

Official bytecode shows `removeWaypoint(UUID)` constructs `Operation.UNTRACK`
with `TrackedWaypoint.empty(UUID)`. The selected fixture does not enter
track/update, position, chunk, or azimuth construction paths.

## Stop Boundary

This proof does not establish waypoint tracking/update semantics, position,
chunk, azimuth, icon behavior, world state, or client-load completion.
