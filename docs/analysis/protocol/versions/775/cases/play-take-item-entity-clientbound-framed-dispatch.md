# play_take_item_entity_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:take_item_entity`
framed dispatch/decode for one primitive ids/count fixture.

```text
client.jar ClientboundTakeItemEntityPacket(int, int, int)
  -> STREAM_CODEC writes itemId/playerId/amount VarInts
    -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x7c
      -> oracle answer frame/body
        -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_take_item_entity_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundTakeItemEntityPacket(int, int, int)`; `ClientboundTakeItemEntityPacket.STREAM_CODEC`; `FriendlyByteBuf.readVarInt/writeVarInt` x3; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_take_item_entity_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_take_item_entity_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x7c` mapping |

Stop boundary: this is packet framing and body-shape evidence only. It does not
prove entity existence, item stack contents, collection behavior, Play
readiness, or client-load completion.
