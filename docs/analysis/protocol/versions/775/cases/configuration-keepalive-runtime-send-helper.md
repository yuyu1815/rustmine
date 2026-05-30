# configuration_keepalive_runtime_send_helper

Purpose: document the Protocol 775 root-owned runtime-send probe for the
Stevenarella `packet::send_keep_alive` helper in Configuration state without
expanding it into the full `spawn_reader` keep-alive echo loop.

## Evidence Map

```text
oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl
  -> official input id + official Configuration serverbound keep_alive frame
    -> oracle/test-manifests/775/runtime/configuration_keepalive_runtime_send_helper.test-manifest.json
      -> oracle/rust-tests/tests/oracle_contracts.rs
        -> localhost TcpListener observes packet::send_keep_alive(&mut Conn, official id)
          -> compare observed bytes to VarInt(official frame length) + official frame
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_keepalive_runtime_send_helper` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | Existing generated answer from `client.jar` calls in `configuration_keepalive_framed_dispatch`: `ConfigurationProtocols.SERVERBOUND.codec().encode(...)`; `ConfigurationProtocols.SERVERBOUND.codec().decode(...)`; `ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...)`; `ServerboundKeepAlivePacket.getId()` |
| Generated answer used | `oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl` |
| Runtime manifest | `oracle/test-manifests/775/runtime/configuration_keepalive_runtime_send_helper.test-manifest.json` |
| Runtime contract metadata | `oracle/contracts/775/runtime/configuration_keepalive_runtime_send_helper.contract.json` |
| Runtime case metadata | `oracle/cases/775/runtime/configuration_keepalive_runtime_send_helper.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_keepalive_runtime_send_helper_sends_official_configuration_frame` |
| Internal owner under test | `stevenarella/protocol/src/protocol/packet.rs`; `packet::send_keep_alive(&mut Conn, id)` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves When Passing

With compression disabled, `packet::send_keep_alive` writes the official
Protocol 775 Configuration serverbound keep-alive frame to the network:

```text
outer network VarInt length
  -> official framed packet from configuration_keepalive_framed_dispatch.answer.jsonl
```

## Current Rust Result

The exact Rust oracle test passes in the current checkout. `packet::send_keep_alive`
branches on `State::Configuration` and writes a frame matching the existing
official serverbound Configuration keep_alive answer. The original failure
packet is kept as the regression task artifact:

```text
oracle/failures/775/configuration_keepalive_runtime_send_helper.why-what-answer.jsonl
```

## Does Not Prove

This does not prove `spawn_reader` handles a clientbound keep-alive and echoes
it, full keep-alive response loop behavior, Configuration completion, Play
entry, registry hydration, or any later client-load phase.
