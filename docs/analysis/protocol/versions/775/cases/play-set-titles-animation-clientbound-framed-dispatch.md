# play_set_titles_animation_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:set_titles_animation`
framed dispatch/decode for one context-free timing fixture.

```text
client.jar ClientboundSetTitlesAnimationPacket(int, int, int)
  -> STREAM_CODEC writes three int fields
    -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x73
      -> oracle answer frame/body
        -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_set_titles_animation_clientbound_framed_dispatch` |
| Official source | `client.jar` `ClientboundSetTitlesAnimationPacket(int, int, int)`; `ClientboundSetTitlesAnimationPacket.STREAM_CODEC`; `FriendlyByteBuf.readInt/writeInt` x3; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_set_titles_animation_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_set_titles_animation_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x73` mapping |

Stop boundary: this is packet framing and body-shape evidence only. It does not
prove title text, UI display behavior, Play readiness, or client-load
completion.
