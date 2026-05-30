# configuration_finish_framed_terminal

Purpose: document the Protocol 775 jar-backed proof for Configuration
`finish_configuration` framed packet dispatch/decode and terminal flags without
expanding it into runtime Configuration-to-Play readiness.

## Evidence Map

```text
client.jar ConfigurationProtocols.SERVERBOUND.codec().encode/decode
  + ServerboundFinishConfigurationPacket.INSTANCE.isTerminal()
client.jar ConfigurationProtocols.CLIENTBOUND.codec().encode/decode
  + ClientboundFinishConfigurationPacket.INSTANCE.isTerminal()
    -> oracle/answers/775/configuration_finish_framed_terminal.answer.jsonl
      -> oracle/test-manifests/775/configuration_finish_framed_terminal.test-manifest.json
        -> oracle/rust-tests/tests/oracle_contracts.rs
          -> packet::packet_by_id(775, Configuration, Serverbound/Clientbound, official id, official body)
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_finish_framed_terminal` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `ConfigurationProtocols.SERVERBOUND.codec().encode/decode(ServerboundFinishConfigurationPacket.INSTANCE)`; `ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundFinishConfigurationPacket.INSTANCE)`; `ServerboundFinishConfigurationPacket.INSTANCE.isTerminal()`; `ClientboundFinishConfigurationPacket.INSTANCE.isTerminal()` |
| Generated answer | `oracle/answers/775/configuration_finish_framed_terminal.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_finish_framed_terminal.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_finish_framed_terminal_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::{Serverbound, Clientbound}, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_finish_framed_terminal.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_finish_framed_terminal.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits and decodes the full Configuration `finish_configuration`
frame in both directions, records the assigned packet ids through the generated
framed bytes and packet tables, and records each packet INSTANCE terminal flag.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, that test passes for both directions: the packets
dispatch, preserve the official `finish_configuration` identity used by the
test surface, and consume the official empty body bytes.

## Does Not Prove

This does not prove the runtime Configuration-to-Play transition, registry
hydration, Play entry, world hydration, render readiness, or client load
completion.
