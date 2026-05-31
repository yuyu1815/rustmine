# play_open_book_clientbound_framed_dispatch

## Spatial Map

```text
client.jar ClientboundOpenBookPacket(InteractionHand)
  -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x3a
    -> oracle/answers/775/play_open_book_clientbound_framed_dispatch.answer.jsonl
      -> oracle/rust-tests/tests/oracle_contracts.rs exact test
```

| Field | Value |
|---|---|
| Case id | `play_open_book_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundOpenBookPacket(InteractionHand)`; `ClientboundOpenBookPacket.STREAM_CODEC`; `FriendlyByteBuf.readEnum/writeEnum(InteractionHand)`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundOpenBookPacket)`; `ClientGamePacketListener.handleOpenBook(ClientboundOpenBookPacket)` |
| Generated answer | `oracle/answers/775/play_open_book_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_open_book_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_open_book_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Stop Boundary

This proves Play clientbound `minecraft:open_book` / `0x3a` packet framing
and dispatch/decode for one `InteractionHand` fixture only. It does not prove
held item, book UI behavior, inventory state, render readiness, or
client-load completion.
