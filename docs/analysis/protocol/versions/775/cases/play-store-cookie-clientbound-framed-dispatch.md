# play_store_cookie_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:store_cookie` framed
dispatch/decode for one key/payload fixture.

```text
client.jar ClientboundStoreCookiePacket(Identifier, byte[])
  -> STREAM_CODEC writes Identifier + bounded byte array
    -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x78
      -> oracle answer frame/body
        -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_store_cookie_clientbound_framed_dispatch` |
| Official source | `client.jar` `Identifier.parse(String)`; `ClientboundStoreCookiePacket(Identifier, byte[])`; `ClientboundStoreCookiePacket.STREAM_CODEC`; `ClientboundStoreCookiePacket.PAYLOAD_STREAM_CODEC`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_store_cookie_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_store_cookie_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x78` mapping |

Stop boundary: this is packet framing and body-shape evidence only. It does
not prove runtime cookie storage policy, cross-state behavior, or client-load
completion.
