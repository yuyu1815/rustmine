# Play Test Instance Block Status Clientbound Framed Dispatch

## Scope

Proves Protocol 775 Play clientbound `minecraft:test_instance_block_status`
for one simple `Component.literal(String)` status plus absent optional size
fixture only.

## Official Evidence

| Source | Evidence |
|---|---|
| `client.jar` | `ClientboundTestInstanceBlockStatus(Component, Optional.empty())` |
| `client.jar` | `STREAM_CODEC` uses `ComponentSerialization.STREAM_CODEC` plus optional `Vec3i` |
| Oracle answer | `oracle/answers/775/play_test_instance_block_status_clientbound_framed_dispatch.answer.jsonl` |

## Boundary

This case does not prove rich Component handling, present `Vec3i` size bytes,
game-test block semantics, or UI behavior.
