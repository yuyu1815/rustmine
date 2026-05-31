# Play update_advancements Clientbound Framed Dispatch

| Field | Value |
|---|---|
| Case | `play_update_advancements_clientbound_framed_dispatch` |
| Packet | `0x82` / `minecraft:update_advancements` |
| Fixture | `ClientboundUpdateAdvancementsPacket(false, List.of(), Set.of(), Map.of(), false)` |
| Body | `0000000000` |
| Scope | Packet framing and dispatch/decode for empty advancement collections only |

## Evidence

Official bytecode shows the selected fixture writes reset, empty added,
removed, and progress collections, and show-advancements. The empty collections
avoid advancement holder and progress payload codecs.

## Stop Boundary

This proof does not establish advancement tree, advancement progress, UI
behavior, non-empty holder/progress payloads, or client-load completion.
