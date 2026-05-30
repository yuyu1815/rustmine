# configuration_keepalive_codec

Purpose: document the one current jar-backed Protocol 775 proof without
expanding it into a broader load claim.

## Evidence Map

```text
client.jar ServerboundKeepAlivePacket.STREAM_CODEC
  -> oracle/answers/775/configuration_keepalive_codec.answer.jsonl
    -> oracle/test-manifests/775/configuration_keepalive_codec.test-manifest.json
      -> oracle/rust-tests/tests/oracle_contracts.rs
        -> stevenarella/protocol/src/protocol/packet.rs behavior under test
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_keepalive_codec` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `ServerboundKeepAlivePacket.STREAM_CODEC`; recorded source label `ConfigurationProtocols.SERVERBOUND.details().listPackets(...)` |
| Generated answer | `oracle/answers/775/configuration_keepalive_codec.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_keepalive_codec.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_keepalive_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/packet.rs`; `packet::configuration::serverbound::ConfigurationKeepAliveServerbound_i64`; `PacketType::packet_id(775)` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

One Configuration serverbound keep-alive packet id/body matches the generated
official jar answer.

## Does Not Prove

This does not prove full login/configuration runtime behavior, keep-alive echo
behavior, Configuration completion, Play entry, or any later client-load phase.
