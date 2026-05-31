# play_clear_dialog_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:clear_dialog` framed
dispatch/decode for the singleton empty-body fixture.

```text
client.jar ClientboundClearDialogPacket.INSTANCE
  -> STREAM_CODEC unit writes empty body
    -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x8b
      -> oracle answer frame/body
        -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_clear_dialog_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundClearDialogPacket.INSTANCE`; `ClientboundClearDialogPacket.STREAM_CODEC`; `StreamCodec.unit(INSTANCE)`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_clear_dialog_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_clear_dialog_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x8b` mapping |

Stop boundary: this is packet framing and body-shape evidence only. It does
not prove dialog UI behavior, `show_dialog` semantics, or client-load
completion.
