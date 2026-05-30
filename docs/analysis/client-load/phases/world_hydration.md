# world_hydration

| Field | Value |
|---|---|
| Lens position | 5 of 8 |
| Load claim | Chunks, light, block states, biomes, and world time produce usable world state. |
| Evidence surface | Official source, generated fixture, or smoke/probe |
| Proof label | `unproven` |
| Current proof | none |
| Project-level test/probe | none |
| Candidate checkout owner under test | world/chunk/light runtime handlers |
| Candidate evidence gap | Define chunk/world oracle fixture strategy. |

## Boundary

Do not mark this phase done from packet decode coverage alone. World hydration
needs a world-state fixture, smoke, or probe that demonstrates usable runtime
state.
