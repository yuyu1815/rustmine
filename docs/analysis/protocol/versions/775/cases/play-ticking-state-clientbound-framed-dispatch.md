# play_ticking_state_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:ticking_state` framed
dispatch/decode for one primitive tick-rate/frozen fixture.

```text
client.jar ClientboundTickingStatePacket(float, boolean)
  -> STREAM_CODEC writes float + boolean
    -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x7f
      -> oracle answer frame/body
        -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_ticking_state_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundTickingStatePacket(float, boolean)`; `ClientboundTickingStatePacket.STREAM_CODEC`; `ClientboundTickingStatePacket.tickRate()`; `ClientboundTickingStatePacket.isFrozen()`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_ticking_state_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_ticking_state_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x7f` mapping |

Stop boundary: this is packet framing and body-shape evidence only. It does
not prove `TickRateManager` runtime semantics, world ticking behavior, or
client-load completion.
