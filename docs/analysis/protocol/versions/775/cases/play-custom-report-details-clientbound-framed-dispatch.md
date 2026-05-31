# play_custom_report_details_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound
`minecraft:custom_report_details` framed dispatch/decode for the empty map
fixture only.

```text
client.jar ClientboundCustomReportDetailsPacket(Map.of())
  -> STREAM_CODEC writes zero VarInt count
    -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x88
      -> oracle answer frame/body
        -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_custom_report_details_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundCustomReportDetailsPacket(Map<String, String>)`; `ClientboundCustomReportDetailsPacket.STREAM_CODEC`; `ByteBufCodecs.map(..., maxCount=32)`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_custom_report_details_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_custom_report_details_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x88` mapping |

Stop boundary: this is packet framing and body-shape evidence only for the
empty map fixture. It does not prove non-empty report detail entries, report UI
behavior, or client-load completion.
