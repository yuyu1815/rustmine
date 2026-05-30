# configuration_keepalive_runtime_protocol_echo

Purpose: document the Protocol 775 root-owned protocol-crate socket probe for
Configuration keep_alive echo behavior before taking on the heavier
`spawn_reader` runtime path.

## Evidence Map

```text
oracle/answers/775/configuration_keepalive_clientbound_framed_dispatch.answer.jsonl
  -> official Configuration clientbound keep_alive network frame
    -> Conn::read_packet() in State::Configuration
      -> Packet::map()
        -> MappedPacket::KeepAliveClientbound(id)
          -> packet::send_keep_alive(&mut Conn, id)
            -> official Configuration serverbound keep_alive network frame
              from oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_keepalive_runtime_protocol_echo` |
| Corridor | `Login -> Configuration -> Play` |
| Inbound official source | Existing generated answer from `client.jar` calls in `configuration_keepalive_clientbound_framed_dispatch`: `ConfigurationProtocols.CLIENTBOUND.codec().encode(...)`; `ConfigurationProtocols.CLIENTBOUND.codec().decode(...)`; `ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundKeepAlivePacket.getId()` |
| Outbound official source | Existing generated answer from `client.jar` calls in `configuration_keepalive_framed_dispatch`: `ConfigurationProtocols.SERVERBOUND.codec().encode(...)`; `ConfigurationProtocols.SERVERBOUND.codec().decode(...)`; `ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...)`; `ServerboundKeepAlivePacket.getId()` |
| Runtime manifest | `oracle/test-manifests/775/runtime/configuration_keepalive_runtime_protocol_echo.test-manifest.json` |
| Runtime contract metadata | `oracle/contracts/775/runtime/configuration_keepalive_runtime_protocol_echo.contract.json` |
| Runtime case metadata | `oracle/cases/775/runtime/configuration_keepalive_runtime_protocol_echo.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_keepalive_runtime_protocol_echo_reads_maps_and_sends_official_frame` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `Conn::read_packet()`; `stevenarella/protocol/src/protocol/mapped_packet.rs`; `Packet::map()`; `stevenarella/protocol/src/protocol/packet.rs`; `packet::send_keep_alive(&mut Conn, id)` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves When Passing

With compression disabled, the protocol crate can consume an official
Configuration clientbound keep_alive network frame, map it to
`MappedPacket::KeepAliveClientbound(id)`, and echo that id as the official
Configuration serverbound keep_alive network frame:

```text
read official clientbound frame
  -> map official id
    -> send official serverbound frame
```

## Current Rust Result

The exact Rust oracle test passes in the current checkout. No Stevenarella Rust
implementation change or rust-fix-task is required for this probe.

## Does Not Prove

This does not prove the full `spawn_reader` thread path, runtime
Configuration-to-Play transition behavior, Play entry, registry hydration, or
any later client-load phase.
