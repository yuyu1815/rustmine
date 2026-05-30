# network_login_configuration

| Field | Value |
|---|---|
| Lens position | 2 of 8 |
| Load claim | Client can satisfy official login/configuration wire contracts. |
| Evidence surface | Official jar codec/state evidence |
| Proof label | `partial` |
| Current proof | `configuration_keepalive_codec`, `configuration_keepalive_framed_dispatch`, `configuration_keepalive_clientbound_framed_dispatch`, and `configuration_finish_framed_terminal` have regenerated jar-backed answers for Configuration keep-alive body/table-id, serverbound/clientbound framed dispatch/decode, and finish_configuration framed/terminal fields; `configuration_keepalive_runtime_send_helper` proves `packet::send_keep_alive` writes the official Configuration serverbound keep_alive frame in Configuration state; `configuration_keepalive_runtime_protocol_echo` proves the protocol crate can read an official Configuration clientbound keep_alive frame, map it to `MappedPacket::KeepAliveClientbound(id)`, and send the official Configuration serverbound keep_alive response. The exact reset-proof Rust oracle tests pass against the current Leafish checkout. |
| Project-level test/probe | `oracle/rust-tests/tests/oracle_contracts.rs` |
| Candidate checkout owner under test | `stevenarella/protocol/src/protocol/packet.rs` |
| Candidate evidence gap | Add full `spawn_reader` keep-alive reaction and runtime Configuration-to-Play transition proof. |

## Proven Slice

The currently proven compatibility slices are `configuration_keepalive_codec`,
`configuration_keepalive_framed_dispatch`,
`configuration_keepalive_clientbound_framed_dispatch`, and
`configuration_finish_framed_terminal`, backed by official jar output and
stored outside the reset-prone checkout. In the current run, the official
answers were regenerated and the manifest-declared Rust oracle tests passed:

```text
oracle/answers/775/configuration_keepalive_codec.answer.jsonl
oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_keepalive_clientbound_framed_dispatch.answer.jsonl
oracle/answers/775/configuration_finish_framed_terminal.answer.jsonl
oracle/test-manifests/775/configuration_keepalive_codec.test-manifest.json
oracle/test-manifests/775/configuration_keepalive_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_keepalive_clientbound_framed_dispatch.test-manifest.json
oracle/test-manifests/775/configuration_finish_framed_terminal.test-manifest.json
oracle/rust-tests/tests/oracle_contracts.rs
```

## Not Proven

This phase is still not complete. The proven slice does not prove full
login/configuration runtime behavior, keep-alive response loop behavior,
runtime Configuration-to-Play transition behavior, or Play readiness.

## Current Runtime-Send Probe

```text
configuration_keepalive_runtime_send_helper
  -> packet::send_keep_alive(&mut Conn in State::Configuration, official id)
    -> expected: VarInt(length) + official Configuration serverbound keep_alive frame
      from oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl
    -> observed: matching frame on localhost listener
```

Regression packet kept for traceability:

```text
oracle/failures/775/configuration_keepalive_runtime_send_helper.why-what-answer.jsonl
```

## Current Protocol Echo Probe

```text
configuration_keepalive_runtime_protocol_echo
  -> official Configuration clientbound keep_alive network frame
    from oracle/answers/775/configuration_keepalive_clientbound_framed_dispatch.answer.jsonl
  -> Conn::read_packet() in State::Configuration
    -> Packet::map()
      -> MappedPacket::KeepAliveClientbound(id)
        -> packet::send_keep_alive(&mut Conn, id)
          -> expected: VarInt(length) + official Configuration serverbound keep_alive frame
            from oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl
          -> observed: matching response frame on localhost listener
```

This still stops before the full `spawn_reader` thread path.
