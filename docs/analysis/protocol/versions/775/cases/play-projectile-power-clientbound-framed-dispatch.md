# Play projectile_power Clientbound Framed Dispatch

| Field | Value |
|---|---|
| Case | `play_projectile_power_clientbound_framed_dispatch` |
| Packet | `0x87` / `minecraft:projectile_power` |
| Fixture | `ClientboundProjectilePowerPacket(12345, 2.5)` |
| Body | `b9604004000000000000` |
| Scope | Packet framing and dispatch/decode for one primitive entity id plus acceleration power |

## Evidence

Official bytecode for `ClientboundProjectilePowerPacket` shows a public
constructor `(int, double)` and a body codec of entity id VarInt followed by
acceleration power double. The selected fixture requires no projectile entity
object, registry holder, or world state to generate the official answer.

## Stop Boundary

This proof does not establish projectile entity existence, acceleration
behavior, world state, or client-load completion. It is packet-support evidence
for the official primitive body shape only.
