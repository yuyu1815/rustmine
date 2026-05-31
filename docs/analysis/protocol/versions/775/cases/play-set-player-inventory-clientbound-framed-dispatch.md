# play_set_player_inventory_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:set_player_inventory`
framed dispatch/decode for one slot plus the empty ItemStack branch.

```text
client.jar slot 7 + ItemStack.EMPTY
  -> ClientboundSetPlayerInventoryPacket(int, ItemStack)
    -> slot VarInt, then ItemStack.OPTIONAL_STREAM_CODEC VarInt count 0
      -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x6c
        -> oracle answer frame/body
          -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_set_player_inventory_clientbound_framed_dispatch` |
| Official source | `client.jar` `ItemStack.EMPTY`; `ClientboundSetPlayerInventoryPacket(int, ItemStack)`; `ClientboundSetPlayerInventoryPacket.STREAM_CODEC`; `ItemStack.OPTIONAL_STREAM_CODEC`; `ItemStack$1.encode/decode`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_set_player_inventory_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_player_inventory_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x6c` mapping |

Stop boundary: this is packet framing and body-shape evidence only for slot 7
with `ItemStack.EMPTY`. Positive ItemStack counts are rejected before item
registry or component patch bytes. This does not prove player inventory state,
non-empty ItemStack, item registry, component patch, Play readiness, or
client-load completion.
