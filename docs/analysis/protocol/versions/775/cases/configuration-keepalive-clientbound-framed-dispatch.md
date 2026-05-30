# configuration_keepalive_clientbound_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof for full Configuration
clientbound keep-alive framed dispatch/decode without expanding it into
runtime keep-alive echo behavior.

## Evidence Map

```text
client.jar ConfigurationProtocols.CLIENTBOUND.codec().encode/decode
  + ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets
  + ClientboundKeepAlivePacket.getId()
    -> oracle/answers/775/configuration_keepalive_clientbound_framed_dispatch.answer.jsonl
      -> oracle/test-manifests/775/configuration_keepalive_clientbound_framed_dispatch.test-manifest.json
        -> oracle/rust-tests/tests/oracle_contracts.rs
          -> packet::packet_by_id(775, Configuration, Clientbound, official id, official body)
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_keepalive_clientbound_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `ConfigurationProtocols.CLIENTBOUND.codec().encode(...)`; `ConfigurationProtocols.CLIENTBOUND.codec().decode(...)`; `ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundKeepAlivePacket.getId()` |
| Generated answer | `oracle/answers/775/configuration_keepalive_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_keepalive_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_keepalive_clientbound_framed_dispatch_decodes_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Clientbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_keepalive_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_keepalive_clientbound_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration clientbound keep-alive frame,
dispatches that frame back to `ClientboundKeepAlivePacket`, preserves the
payload id via `getId()`, and leaves no unread bytes after official decode.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, that test passes after mapping Configuration clientbound
packet id `0x04` and decoding the official i64 body:

```text
cargo test --manifest-path oracle/rust-tests/Cargo.toml --test oracle_contracts \
  configuration_keepalive_clientbound_framed_dispatch_decodes_official_oracle_answer -- --exact
bash oracle/scripts/run_jar_backed_oracle_tests.sh
```

## Does Not Prove

This does not prove runtime keep-alive echo behavior, serverbound response
handling, Configuration completion, Play entry, registry hydration, or any
later client-load phase.
