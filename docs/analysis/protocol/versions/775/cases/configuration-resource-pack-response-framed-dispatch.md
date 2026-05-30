# configuration_resource_pack_response_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof package for Configuration
serverbound `minecraft:resource_pack` response framed dispatch/decode without
expanding it into runtime resource pack UI, accept/reject behavior, download,
or reload behavior.

## Evidence Map

```text
client.jar ServerboundResourcePackPacket(UUID, Action)
  -> ServerboundResourcePackPacket.STREAM_CODEC
  -> ConfigurationProtocols.SERVERBOUND.codec().encode/decode
  -> ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets
    -> oracle/answers/775/configuration_resource_pack_response_framed_dispatch.answer.jsonl
      -> oracle/test-manifests/775/configuration_resource_pack_response_framed_dispatch.test-manifest.json
        -> oracle/rust-tests/tests/oracle_contracts.rs
          -> packet::packet_by_id(775, Configuration, Serverbound, official id, official body)
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_resource_pack_response_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `ServerboundResourcePackPacket(UUID, Action)`; `ServerboundResourcePackPacket.STREAM_CODEC`; `ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundResourcePackPacket)`; `ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...)`; `ServerboundResourcePackPacket.id()`; `ServerboundResourcePackPacket.action()`; `ServerboundResourcePackPacket.Action.isTerminal()` |
| Generated answer | `oracle/answers/775/configuration_resource_pack_response_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_resource_pack_response_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_resource_pack_response_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Serverbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_resource_pack_response_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_resource_pack_response_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration serverbound
`minecraft:resource_pack` frame for `ServerboundResourcePackPacket`, dispatches
that frame back to `ServerboundResourcePackPacket`, preserves the UUID and
official `Action`, records the official action terminal flag, and leaves no
unread bytes after official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, UUID bytes, action encoding, and action terminal flag.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, that test passes: Protocol 775 Configuration serverbound
id `0x06` dispatches to the resource-pack response body decoder and consumes
the official UUID/action body bytes.

## Does Not Prove

This does not prove runtime resource pack UI, accept/reject behavior,
download/reload behavior, Configuration completion, Play entry, registry
hydration, or any later client-load phase.
