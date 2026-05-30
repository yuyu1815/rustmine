# configuration_keepalive_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof for full Configuration
serverbound keep-alive framed dispatch/decode without expanding it into
runtime keep-alive behavior.

## Evidence Map

```text
client.jar ConfigurationProtocols.SERVERBOUND.codec().encode/decode
  + ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets
  + ServerboundKeepAlivePacket.getId()
    -> oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl
      -> oracle/test-manifests/775/configuration_keepalive_framed_dispatch.test-manifest.json
        -> oracle/rust-tests/tests/oracle_contracts.rs
          -> packet::packet_by_id(775, Configuration, Serverbound, official id, official body)
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_keepalive_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `ConfigurationProtocols.SERVERBOUND.codec().encode(...)`; `ConfigurationProtocols.SERVERBOUND.codec().decode(...)`; `ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...)`; `ServerboundKeepAlivePacket.getId()` |
| Generated answer | `oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_keepalive_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_keepalive_framed_dispatch_decodes_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Serverbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_keepalive_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_keepalive_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration serverbound keep-alive frame,
dispatches that frame back to `ServerboundKeepAlivePacket`, preserves the
payload id via `getId()`, and leaves no unread bytes after official decode.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, that test passes: the packet dispatches, decodes the
official payload id, and consumes the official body bytes.

## Does Not Prove

This does not prove runtime keep-alive echo behavior, Configuration completion,
Play entry, registry hydration, or any later client-load phase.
