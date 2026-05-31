# play_stop_sound_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:stop_sound` framed
dispatch/decode for the null/null fixture only.

```text
client.jar ClientboundStopSoundPacket(null, null)
  -> STREAM_CODEC writes flags byte 0
    -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x77
      -> oracle answer frame/body
        -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_stop_sound_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundStopSoundPacket(null, null)`; `ClientboundStopSoundPacket.STREAM_CODEC`; `ClientboundStopSoundPacket.getName()`; `ClientboundStopSoundPacket.getSource()`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_stop_sound_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_stop_sound_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x77` mapping to existing `StopSound` body decoder |

Stop boundary: this is packet framing and body-shape evidence only for the
flags byte `0` null/null fixture. It does not prove named `SoundSource` ids,
named sound `Identifier` bodies, sound playback, or client-load completion.
