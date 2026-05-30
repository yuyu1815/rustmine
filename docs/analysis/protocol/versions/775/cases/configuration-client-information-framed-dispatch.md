# configuration_client_information_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof for Configuration
serverbound `minecraft:client_information` framed dispatch/decode without
expanding it into runtime client settings send behavior.

## Evidence Map

```text
client.jar ClientInformation.createDefault()
  -> ServerboundClientInformationPacket.STREAM_CODEC
  -> ConfigurationProtocols.SERVERBOUND.codec().encode/decode
  -> ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets
    -> oracle/answers/775/configuration_client_information_framed_dispatch.answer.jsonl
      -> oracle/test-manifests/775/configuration_client_information_framed_dispatch.test-manifest.json
        -> oracle/rust-tests/tests/oracle_contracts.rs
          -> packet::packet_by_id(775, Configuration, Serverbound, official id, official body)
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_client_information_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `ClientInformation.createDefault()`; `ServerboundClientInformationPacket.STREAM_CODEC`; `ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundClientInformationPacket)`; `ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...)`; `ServerboundClientInformationPacket.information()` |
| Generated answer | `oracle/answers/775/configuration_client_information_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_client_information_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_client_information_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Serverbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_client_information_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_client_information_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration serverbound
`minecraft:client_information` frame, dispatches that frame back to
`ServerboundClientInformationPacket`, preserves the `ClientInformation` record
created by `ClientInformation.createDefault()`, and leaves no unread bytes
after official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, and record fields.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, that test passes: Protocol 775 Configuration serverbound
id `0x00` dispatches as `client_information`, preserves identity in the debug
surface, and consumes the official body bytes.

## Does Not Prove

This does not prove runtime client settings send behavior, Configuration
completion, Play entry, registry hydration, or any later client-load phase.
