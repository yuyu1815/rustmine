# play_show_dialog_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:show_dialog` framed
dispatch/decode for one direct NoticeDialog holder fixture.

```text
client.jar Holder.direct(NoticeDialog)
  -> ClientboundShowDialogPacket.STREAM_CODEC
    -> Dialog.STREAM_CODEC direct-holder branch
      -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x8c
        -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_show_dialog_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundShowDialogPacket(Holder<Dialog>)`; `ClientboundShowDialogPacket.STREAM_CODEC`; `Dialog.STREAM_CODEC`; `ByteBufCodecs.holder`; `NoticeDialog(CommonDialogData, NoticeDialog.DEFAULT_ACTION)`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_show_dialog_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_show_dialog_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x8c` mapping |

Stop boundary: this is packet framing and body-shape evidence only. It does
not prove dialog UI display behavior, screen flow, registry-backed dialogs,
custom actions, or client-load completion.
