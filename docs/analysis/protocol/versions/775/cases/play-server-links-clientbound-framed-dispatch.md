# play_server_links_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:server_links` framed
dispatch/decode for the empty list fixture only.

```text
client.jar ClientboundServerLinksPacket(List.of())
  -> STREAM_CODEC writes zero VarInt count
    -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x89
      -> oracle answer frame/body
        -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_server_links_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundServerLinksPacket(List<ServerLinks.UntrustedEntry>)`; `ClientboundServerLinksPacket.STREAM_CODEC`; `ServerLinks.UNTRUSTED_LINKS_STREAM_CODEC`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_server_links_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_server_links_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x89` mapping |

Stop boundary: this is packet framing and body-shape evidence only for the
empty list fixture. It does not prove non-empty server link entries, link UI
behavior, or client-load completion.
