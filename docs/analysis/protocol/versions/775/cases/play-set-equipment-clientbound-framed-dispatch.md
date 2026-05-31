# play_set_equipment_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:set_equipment`
framed dispatch/decode for one MAINHAND empty ItemStack entry.

```text
client.jar entity id 123 + EquipmentSlot.MAINHAND + ItemStack.EMPTY
  -> ClientboundSetEquipmentPacket(int, List<Pair<EquipmentSlot, ItemStack>>)
    -> slot byte 0, then ItemStack.OPTIONAL_STREAM_CODEC VarInt count 0
      -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x66
        -> oracle answer frame/body
          -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_set_equipment_clientbound_framed_dispatch` |
| Official source | `client.jar` `ItemStack.EMPTY`; `EquipmentSlot.MAINHAND`; `ClientboundSetEquipmentPacket(int, List<Pair<EquipmentSlot, ItemStack>>)`; `ClientboundSetEquipmentPacket.STREAM_CODEC`; `ItemStack.OPTIONAL_STREAM_CODEC`; `ItemStack$1.encode/decode`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_set_equipment_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_equipment_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x66` mapping |

Stop boundary: this is packet framing and body-shape evidence only for one
MAINHAND `ItemStack.EMPTY` entry. Multi-entry continuation and positive
ItemStack counts are rejected before broader parsing. This does not prove
entity existence, equipment behavior, non-empty ItemStack, item registry,
component patch, Play readiness, or client-load completion.
