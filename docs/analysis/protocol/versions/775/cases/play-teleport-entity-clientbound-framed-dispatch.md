# Play teleport_entity Clientbound Framed Dispatch

| Field | Value |
|---|---|
| Case | `play_teleport_entity_clientbound_framed_dispatch` |
| Packet | `0x7d` / `minecraft:teleport_entity` |
| Fixture | `ClientboundTeleportEntityPacket.teleport(12345, PositionMoveRotation(...), Set.of(), false)` |
| Body | `b9603ff40000000000004004000000000000c00e00000000000000000000000000000000000000000000000000000000000042340000412000000000000000` |
| Scope | Packet framing and dispatch/decode for one primitive teleport fixture with empty relatives |

## Evidence

Official bytecode shows a public primitive constructor and `teleport` factory
taking an entity id, `PositionMoveRotation`, relative set, and onGround flag.
The selected fixture does not construct or fake an `Entity` object.

## Stop Boundary

This proof does not establish entity existence, movement semantics, relative
movement policy, world state, or client-load completion.
