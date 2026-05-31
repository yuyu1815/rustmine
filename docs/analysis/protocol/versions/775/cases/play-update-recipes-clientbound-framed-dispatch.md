# Play update_recipes Clientbound Framed Dispatch

| Field | Value |
|---|---|
| Case | `play_update_recipes_clientbound_framed_dispatch` |
| Packet | `0x85` / `minecraft:update_recipes` |
| Fixture | `ClientboundUpdateRecipesPacket(Map.of(), SelectableRecipe.SingleInputSet.empty())` |
| Body | `0000` |
| Scope | Packet framing and dispatch/decode for empty recipe structures only |

## Evidence

Official bytecode shows `itemSets` and `stonecutterRecipes` are encoded through
map/list codecs. The selected empty map and empty single-input set write two
zero counts and avoid item, SlotDisplay, recipe property, and recipe payloads.

## Stop Boundary

This proof does not establish recipe contents, item semantics, SlotDisplay,
recipe property sets, registry behavior, or client-load completion.
