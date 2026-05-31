# play_start_configuration_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:start_configuration`
framed dispatch/decode for the official singleton empty-body fixture.

```text
client.jar ClientboundStartConfigurationPacket.INSTANCE
  -> StreamCodec.unit(INSTANCE)
    -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x76
      -> oracle answer frame/body
        -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_start_configuration_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundStartConfigurationPacket.INSTANCE`; `ClientboundStartConfigurationPacket.STREAM_CODEC`; `StreamCodec.unit(INSTANCE)`; `ClientboundStartConfigurationPacket.isTerminal()`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_start_configuration_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_start_configuration_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x76` mapping |

Stop boundary: this is packet framing and body-shape evidence only. It does not
prove runtime Play-to-Configuration state transition handling, Play readiness,
or client-load completion.
