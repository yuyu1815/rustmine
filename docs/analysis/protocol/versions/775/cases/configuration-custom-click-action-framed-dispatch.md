# configuration_custom_click_action_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof package for Configuration
serverbound `minecraft:custom_click_action` framed dispatch/decode without
expanding it into UI behavior, command execution, interaction readiness,
Configuration completion, or Play readiness.

## Evidence Map

```text
client.jar Identifier.parse(String)
  -> CompoundTag.putString(String, String)
    -> ServerboundCustomClickActionPacket(Identifier, Optional<Tag>)
      -> ServerboundCustomClickActionPacket.STREAM_CODEC
        -> ConfigurationProtocols.SERVERBOUND.codec().encode/decode
        -> ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets
          -> oracle/answers/775/configuration_custom_click_action_framed_dispatch.answer.jsonl
            -> oracle/test-manifests/775/configuration_custom_click_action_framed_dispatch.test-manifest.json
              -> oracle/rust-tests/tests/oracle_contracts.rs
                -> packet::packet_by_id(775, Configuration, Serverbound, official id, official body)
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_custom_click_action_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `Identifier.parse(String)`; `CompoundTag.putString(String, String)`; `ServerboundCustomClickActionPacket(Identifier, Optional<Tag>)`; `ServerboundCustomClickActionPacket.STREAM_CODEC`; `ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundCustomClickActionPacket)`; `ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...)`; `ServerboundCustomClickActionPacket.id()`; `ServerboundCustomClickActionPacket.payload()`; `Tag.getId()`; `Tag.getType()`; `Tag.toString()` |
| Generated answer | `oracle/answers/775/configuration_custom_click_action_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_custom_click_action_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_custom_click_action_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Serverbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_custom_click_action_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_custom_click_action_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration serverbound
`minecraft:custom_click_action` frame for
`ServerboundCustomClickActionPacket`, dispatches that frame back to
`ServerboundCustomClickActionPacket`, preserves the official `Identifier` and
optional `CompoundTag` payload fixture, and leaves no unread bytes after
official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, identifier encoding, optional payload encoding, and decoded payload
fields.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, that test passes: Protocol 775 Configuration serverbound
id `0x08` dispatches to the custom-click-action body decoder and consumes the
official identifier/optional-payload body bytes.

## Does Not Prove

This does not prove UI behavior, command execution, interaction readiness,
Configuration completion, Play entry, world hydration, or any later
client-load phase.
