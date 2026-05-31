# Play Tag Query Clientbound Framed Dispatch

## Scope

Proves Protocol 775 Play clientbound `minecraft:tag_query` for one empty
`CompoundTag` fixture only.

## Official Evidence

| Source | Evidence |
|---|---|
| `client.jar` | `ClientboundTagQueryPacket(int, CompoundTag)` |
| `client.jar` | `FriendlyByteBuf.writeNbt`/`readNbt` encode the empty compound branch |
| Oracle answer | `oracle/answers/775/play_tag_query_clientbound_framed_dispatch.answer.jsonl` |

## Boundary

This case does not prove non-empty NBT payload policy, command block/entity
query behavior, or UI behavior.
