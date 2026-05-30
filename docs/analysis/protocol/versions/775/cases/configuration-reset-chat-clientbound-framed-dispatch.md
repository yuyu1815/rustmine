# configuration_reset_chat_clientbound_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof package for Configuration
clientbound `minecraft:reset_chat` framed dispatch/decode without expanding it
into chat UI reset behavior, screen flow, Configuration completion, registry
hydration, Play entry, or runtime behavior.

## Evidence Map

```text
client.jar ClientboundResetChatPacket.INSTANCE
  -> ClientboundResetChatPacket.STREAM_CODEC
    -> ConfigurationProtocols.CLIENTBOUND.codec().encode/decode
      -> ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets
        -> oracle/answers/775/configuration_reset_chat_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/configuration_reset_chat_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> packet::packet_by_id(775, Configuration, Clientbound, official id, official body)
                -> current result: matching reset_chat identity and consumed empty body
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_reset_chat_clientbound_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `ClientboundResetChatPacket.INSTANCE`; `ClientboundResetChatPacket.STREAM_CODEC`; `ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundResetChatPacket)`; `ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundResetChatPacket.type()` |
| Bytecode witness | `_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.configuration.ClientboundResetChatPacket net.minecraft.network.protocol.configuration.ConfigurationPacketTypes net.minecraft.network.protocol.configuration.ConfigurationProtocols` shows `ClientboundResetChatPacket.STREAM_CODEC` and the clientbound table entry |
| Generated answer | `oracle/answers/775/configuration_reset_chat_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_reset_chat_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_reset_chat_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Clientbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_reset_chat_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_reset_chat_clientbound_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration clientbound
`minecraft:reset_chat` frame for the singleton
`ClientboundResetChatPacket.INSTANCE`, dispatches that frame back to
`ClientboundResetChatPacket`, preserves singleton identity, and leaves no
unread bytes after official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, decoded packet class, singleton equality, and remaining decode byte
count.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, the test passes and maps the official
`minecraft:reset_chat` packet to the current compatibility alias
`Packet::PluginMessageClientbound` with channel `ResetChat` while consuming the
official empty body.

```text
oracle/failures/775/configuration_reset_chat_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_reset_chat_clientbound_framed_dispatch.rust-fix-task.json
```

## Does Not Prove

This does not prove chat UI reset behavior, screen flow, Configuration
completion, Play entry, registry hydration, world load, render readiness, or
any later client-load phase.
