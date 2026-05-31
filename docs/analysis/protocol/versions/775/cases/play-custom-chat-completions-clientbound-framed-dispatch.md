# play_custom_chat_completions_clientbound_framed_dispatch

Purpose: preserve the Protocol 775 Play clientbound
`minecraft:custom_chat_completions` packet id/body contract as a reset-proof
packet-support slice.

```text
client.jar ClientboundCustomChatCompletionsPacket(Action.ADD, ["alpha"])
  -> ClientboundCustomChatCompletionsPacket.STREAM_CODEC
    -> GameProtocols.CLIENTBOUND_TEMPLATE table id 0x17
      -> GameProtocols.CLIENTBOUND codec encodes/decodes official frame
        -> oracle/answers/775/play_custom_chat_completions_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/play_custom_chat_completions_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> stevenarella/protocol/src/protocol/versions/v26_1_2.rs
```

| Field | Value |
|---|---|
| Case id | `play_custom_chat_completions_clientbound_framed_dispatch` |
| Corridor | `Configuration -> Play` |
| Official source | `client.jar` `ClientboundCustomChatCompletionsPacket(Action, List<String>)`; `ClientboundCustomChatCompletionsPacket.STREAM_CODEC`; `FriendlyByteBuf.writeEnum/readEnum`; `FriendlyByteBuf.writeCollection/readList` with UTF-8 strings; `GameProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `GameProtocols.CLIENTBOUND_TEMPLATE.bind(RegistryFriendlyByteBuf.decorator(RegistryAccess.EMPTY)).codec().encode/decode(ClientboundCustomChatCompletionsPacket)` |
| Generated answer | `oracle/answers/775/play_custom_chat_completions_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/play_custom_chat_completions_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::play_custom_chat_completions_clientbound_framed_dispatch_matches_official_oracle_answer` |

## Official Body Shape

| Order | Field | Fixture value |
|---|---|---|
| 1 | `action` via `FriendlyByteBuf.writeEnum` | `ADD` / ordinal `0` |
| 2 | `entries` via VarInt list of UTF-8 strings | `["alpha"]` |

The generated official frame is:

```text
17000105616c706861
```

## Stop Boundary

This is packet framing/dispatch/decode evidence for one official
`ClientboundCustomChatCompletionsPacket` fixture only. It does not prove chat
UI behavior, command context behavior, completion lifecycle semantics, runtime
Play entry, world load, spawn readiness, render readiness, or client-load
completion.
