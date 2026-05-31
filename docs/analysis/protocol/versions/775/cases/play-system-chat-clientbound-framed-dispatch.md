# play_system_chat_clientbound_framed_dispatch

Purpose: prove Protocol 775 Play clientbound `minecraft:system_chat` framed
dispatch/decode for one simple trusted Component plus false overlay fixture.

```text
client.jar Component.literal("rustmine system chat") + false
  -> ClientboundSystemChatPacket(Component, boolean)
    -> TRUSTED_STREAM_CODEC writes NBT string component bytes, then BOOL
      -> GameProtocols.CLIENTBOUND_TEMPLATE assigns 0x79
        -> oracle answer frame/body
          -> Stevenarella packet_by_id Play Clientbound
```

| Field | Value |
|---|---|
| Case id | `play_system_chat_clientbound_framed_dispatch` |
| Official source | `client.jar` `Component.literal(String)`; `ClientboundSystemChatPacket(Component, boolean)`; `ClientboundSystemChatPacket.STREAM_CODEC`; `ComponentSerialization.TRUSTED_STREAM_CODEC`; `ByteBufCodecs.BOOL`; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/play_system_chat_clientbound_framed_dispatch.answer.jsonl` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_system_chat_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner | `stevenarella/protocol/src/protocol/versions/v26_1_2.rs` Play clientbound `0x79` mapping |

Stop boundary: this is packet framing and body-shape evidence only for the
simple NBT string Component produced by `Component.literal(String)` and the
`overlay=false` branch. It does not prove rich Component forms, signed chat,
chat HUD behavior, Play readiness, or client-load completion.
