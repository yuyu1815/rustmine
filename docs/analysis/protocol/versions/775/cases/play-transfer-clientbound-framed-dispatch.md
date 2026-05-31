# play_transfer_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:transfer` framed
dispatch/decode for one host/port fixture.

```text
client.jar ClientboundTransferPacket(String, int)
  -> STREAM_CODEC writes host string + VarInt port
    -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x81
      -> oracle answer frame/body
        -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_transfer_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundTransferPacket(String, int)`; `ClientboundTransferPacket.STREAM_CODEC`; `ClientboundTransferPacket.host()`; `ClientboundTransferPacket.port()`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_transfer_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_transfer_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x81` mapping |

Stop boundary: this is packet framing and body-shape evidence only. It does
not prove runtime transfer handling, reconnect policy, cross-state behavior,
or client-load completion.
