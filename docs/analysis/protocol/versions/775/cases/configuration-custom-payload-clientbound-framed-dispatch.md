# configuration_custom_payload_clientbound_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof package for Configuration
clientbound `minecraft:custom_payload` framed dispatch/decode without expanding
it into arbitrary plugin-channel handling, payload routing policy,
Configuration completion, registry hydration, Play entry, or runtime behavior.

## Evidence Map

```text
client.jar BrandPayload(String)
  -> BrandPayload.STREAM_CODEC
    -> ClientboundCustomPayloadPacket(CustomPacketPayload)
      -> ClientboundCustomPayloadPacket.CONFIG_STREAM_CODEC
        -> ConfigurationProtocols.CLIENTBOUND.codec().encode/decode
          -> ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets
            -> oracle/answers/775/configuration_custom_payload_clientbound_framed_dispatch.answer.jsonl
              -> oracle/test-manifests/775/configuration_custom_payload_clientbound_framed_dispatch.test-manifest.json
                -> oracle/rust-tests/tests/oracle_contracts.rs
                  -> packet::packet_by_id(775, Configuration, Clientbound, official id, official body)
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_custom_payload_clientbound_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `BrandPayload(String)`; `BrandPayload.STREAM_CODEC`; `ClientboundCustomPayloadPacket(CustomPacketPayload)`; `ClientboundCustomPayloadPacket.CONFIG_STREAM_CODEC`; `ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundCustomPayloadPacket)`; `ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundCustomPayloadPacket.payload()`; `BrandPayload.type()`; `BrandPayload.brand()` |
| Generated answer | `oracle/answers/775/configuration_custom_payload_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_custom_payload_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_custom_payload_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Clientbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_custom_payload_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_custom_payload_clientbound_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration clientbound
`minecraft:custom_payload` frame for `ClientboundCustomPayloadPacket` carrying
an official `BrandPayload`, dispatches that frame back to
`ClientboundCustomPayloadPacket`, preserves the payload id `minecraft:brand`,
preserves the brand string, and leaves no unread bytes after official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, payload body bytes, decoded packet class, decoded payload class,
payload id, brand string, and remaining decode byte count.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, the test passes and maps the official
`minecraft:custom_payload` packet to the current compatibility alias
`Packet::PluginMessageClientbound` with channel `minecraft:brand` while
preserving the official BrandPayload body bytes and consuming the full
official body.

```text
oracle/failures/775/configuration_custom_payload_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_custom_payload_clientbound_framed_dispatch.rust-fix-task.json
```

## Does Not Prove

This does not prove arbitrary plugin-channel handling, payload routing policy,
Configuration completion, registry hydration, Play entry, world load, render
readiness, or any later client-load phase.
