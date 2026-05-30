# configuration_keepalive_runtime_spawn_reader_reaction

Purpose: document the Protocol 775 root-owned runtime probe package for the
full Stevenarella `spawn_reader` Configuration keep_alive reaction path.

## Evidence Map

```text
oracle/answers/775/configuration_keepalive_clientbound_framed_dispatch.answer.jsonl
  -> official Configuration clientbound keep_alive network frame
    -> stevenarella/src/server/mod.rs Server::spawn_reader
      -> Conn::read_packet()
        -> Packet::map()
          -> MappedPacket::KeepAliveClientbound(id)
            -> packet::send_keep_alive(server.conn.write().as_mut().unwrap(), id)
              -> expected official Configuration serverbound keep_alive network frame
                from oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_keepalive_runtime_spawn_reader_reaction` |
| Corridor | `Login -> Configuration -> Play` |
| Inbound official source | Existing generated answer from `client.jar` calls in `configuration_keepalive_clientbound_framed_dispatch`: `ConfigurationProtocols.CLIENTBOUND.codec().encode(...)`; `ConfigurationProtocols.CLIENTBOUND.codec().decode(...)`; `ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundKeepAlivePacket.getId()` |
| Outbound official source | Existing generated answer from `client.jar` calls in `configuration_keepalive_framed_dispatch`: `ConfigurationProtocols.SERVERBOUND.codec().encode(...)`; `ConfigurationProtocols.SERVERBOUND.codec().decode(...)`; `ConfigurationProtocols.SERVERBOUND_TEMPLATE.details().listPackets(...)`; `ServerboundKeepAlivePacket.getId()` |
| Runtime manifest | `oracle/test-manifests/775/runtime/configuration_keepalive_runtime_spawn_reader_reaction.test-manifest.json` |
| Runtime contract metadata | `oracle/contracts/775/runtime/configuration_keepalive_runtime_spawn_reader_reaction.contract.json` |
| Runtime case metadata | `oracle/cases/775/runtime/configuration_keepalive_runtime_spawn_reader_reaction.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_keepalive_runtime_spawn_reader_reaction_echoes_official_frame` |
| Internal owner under test | `stevenarella/src/server/mod.rs`; `Server::spawn_reader`; `stevenarella/protocol/src/protocol/mod.rs`; `Conn::read_packet()`; `stevenarella/protocol/src/protocol/mapped_packet.rs`; `Packet::map()`; `stevenarella/protocol/src/protocol/packet.rs`; `packet::send_keep_alive(&mut Conn, id)` |
| Failure packets | `oracle/failures/775/configuration_keepalive_runtime_spawn_reader_reaction.why-what-answer.jsonl`; `oracle/failures/775/configuration_keepalive_runtime_spawn_reader_reaction.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Current Rust Result

The exact Rust oracle test now passes. It validates the official inbound and
outbound answer artifacts, then runs a crate-local probe through
`Server::handle_next_reader_packet_for_oracle`, the same factored keep_alive
branch used by `Server::spawn_reader`, and observes the official Configuration
serverbound keep_alive network frame.

The original failure packets remain as regression traceability for the narrow
reader-loop helper/factorization.

## Proven Slice

With compression disabled, the factored `Server::spawn_reader` keep_alive
branch consumes one official Configuration clientbound keep_alive frame and
writes the official Configuration serverbound keep_alive response frame:

```text
read official clientbound frame through spawn_reader
  -> map official keep_alive id
    -> send official serverbound frame from the same reader-loop branch
```

## Does Not Prove

This does not prove Configuration completion, runtime Configuration-to-Play
transition behavior, Play entry, registry hydration, world load, render
readiness, or any later client-load phase.
