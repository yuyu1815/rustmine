# configuration_accept_code_of_conduct_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof package for Configuration
serverbound `minecraft:accept_code_of_conduct` framed dispatch/decode without
expanding it into UI consent flow, legal acceptance semantics, Configuration
completion, or Play readiness.

## Evidence Map

```text
client.jar ServerboundAcceptCodeOfConductPacket.INSTANCE
  -> ServerboundAcceptCodeOfConductPacket.STREAM_CODEC
    -> ConfigurationProtocols.SERVERBOUND.codec().encode/decode
    -> ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets
      -> oracle/answers/775/configuration_accept_code_of_conduct_framed_dispatch.answer.jsonl
        -> oracle/test-manifests/775/configuration_accept_code_of_conduct_framed_dispatch.test-manifest.json
          -> oracle/rust-tests/tests/oracle_contracts.rs
            -> packet::packet_by_id(775, Configuration, Serverbound, official id, official body)
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_accept_code_of_conduct_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `ServerboundAcceptCodeOfConductPacket.INSTANCE`; `ServerboundAcceptCodeOfConductPacket.STREAM_CODEC`; `ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundAcceptCodeOfConductPacket)`; `ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...)`; `ServerboundAcceptCodeOfConductPacket.type()`; `ServerboundAcceptCodeOfConductPacket.equals(Object)` |
| Generated answer | `oracle/answers/775/configuration_accept_code_of_conduct_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_accept_code_of_conduct_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_accept_code_of_conduct_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Serverbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_accept_code_of_conduct_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_accept_code_of_conduct_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration serverbound
`minecraft:accept_code_of_conduct` frame for
`ServerboundAcceptCodeOfConductPacket`, dispatches that frame back to
`ServerboundAcceptCodeOfConductPacket`, preserves the official singleton
packet identity, and leaves no unread bytes after official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, decoded packet class, singleton equality result, and remaining
decode byte count.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, that test passes: Protocol 775 Configuration serverbound
id `0x09` dispatches to the current public `Packet::PluginMessageServerbound`
compatibility alias with channel `AcceptCodeOfConduct`, carries no alias data,
and consumes the official empty body.

## Does Not Prove

This does not prove UI consent flow, legal acceptance semantics,
Configuration completion, Play entry, world hydration, or any later
client-load phase.
