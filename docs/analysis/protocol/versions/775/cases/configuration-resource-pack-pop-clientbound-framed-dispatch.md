# configuration_resource_pack_pop_clientbound_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof package for Configuration
clientbound `minecraft:resource_pack_pop` framed dispatch/decode without
expanding it into resource-pack UI behavior, pack removal policy,
download/reload behavior, Configuration completion, Play entry, world load,
render readiness, or runtime behavior.

## Evidence Map

```text
client.jar ClientboundResourcePackPopPacket(Optional.of(UUID))
  -> ClientboundResourcePackPopPacket.STREAM_CODEC
    -> ConfigurationProtocols.CLIENTBOUND.codec().encode/decode
      -> ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets
        -> oracle/answers/775/configuration_resource_pack_pop_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/configuration_resource_pack_pop_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> packet::packet_by_id(775, Configuration, Clientbound, official id, official body)
                -> current result: ResourcePackPop compatibility identity and consumed body
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_resource_pack_pop_clientbound_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `ClientboundResourcePackPopPacket(Optional<UUID>)`; `ClientboundResourcePackPopPacket.STREAM_CODEC`; `ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundResourcePackPopPacket)`; `ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundResourcePackPopPacket.id()` |
| Bytecode witness | `_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundResourcePackPopPacket net.minecraft.network.protocol.configuration.ConfigurationProtocols` |
| Generated answer | `oracle/answers/775/configuration_resource_pack_pop_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_resource_pack_pop_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_resource_pack_pop_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Clientbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_resource_pack_pop_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_resource_pack_pop_clientbound_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration clientbound
`minecraft:resource_pack_pop` frame for a present UUID fixture, dispatches that
frame back to `ClientboundResourcePackPopPacket`, preserves optional UUID
presence and value, and leaves no unread bytes after official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, decoded packet class, optional UUID presence/value, and remaining
decode byte count.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, the test passes and maps the official
`minecraft:resource_pack_pop` packet to the current compatibility alias
`Packet::PluginMessageClientbound` with channel `ResourcePackPop` while
consuming the official body bytes.

```text
oracle/failures/775/configuration_resource_pack_pop_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_resource_pack_pop_clientbound_framed_dispatch.rust-fix-task.json
```

## Does Not Prove

This does not prove resource-pack UI behavior, pack removal policy,
download/reload behavior, Configuration completion, Play entry, world load,
render readiness, or any later client-load phase.
