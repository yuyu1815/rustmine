# play_ticking_step_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:ticking_step` framed
dispatch/decode for one primitive tick-step fixture.

```text
client.jar ClientboundTickingStepPacket(int)
  -> STREAM_CODEC writes one VarInt
    -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x80
      -> oracle answer frame/body
        -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_ticking_step_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundTickingStepPacket(int)`; `ClientboundTickingStepPacket.STREAM_CODEC`; `ClientboundTickingStepPacket.tickSteps()`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_ticking_step_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_ticking_step_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x80` mapping |

Stop boundary: this is packet framing and body-shape evidence only. It does
not prove `TickRateManager` runtime semantics, world ticking behavior, or
client-load completion.
