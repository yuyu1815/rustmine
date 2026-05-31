# configuration_registry_data_clientbound_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof package for Configuration
clientbound `minecraft:registry_data` framed dispatch/decode without expanding
it into real registry contents, `RegistrySynchronization.packRegistries`
output, registry hydration, Configuration completion, Play entry, world load,
render readiness, or runtime behavior.

## Evidence Map

```text
client.jar Registries.DIMENSION_TYPE
  -> ClientboundRegistryDataPacket(ResourceKey, List.of())
    -> ClientboundRegistryDataPacket.STREAM_CODEC
      -> ConfigurationProtocols.CLIENTBOUND.codec().encode/decode
        -> ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets
          -> oracle/answers/775/configuration_registry_data_clientbound_framed_dispatch.answer.jsonl
            -> oracle/test-manifests/775/configuration_registry_data_clientbound_framed_dispatch.test-manifest.json
              -> oracle/rust-tests/tests/oracle_contracts.rs
                -> packet::packet_by_id(775, Configuration, Clientbound, official id, official body)
                  -> current result: RegistryData compatibility identity and consumed body
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_registry_data_clientbound_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `Registries.DIMENSION_TYPE`; `ClientboundRegistryDataPacket(ResourceKey, List)`; `ClientboundRegistryDataPacket.STREAM_CODEC`; `ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundRegistryDataPacket)`; `ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundRegistryDataPacket.registry()`; `ClientboundRegistryDataPacket.entries()` |
| Bytecode witness | `_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.configuration.ClientboundRegistryDataPacket 'net.minecraft.core.RegistrySynchronization$PackedRegistryEntry' net.minecraft.network.protocol.configuration.ConfigurationPacketTypes net.minecraft.network.protocol.configuration.ConfigurationProtocols net.minecraft.core.registries.Registries` |
| Generated answer | `oracle/answers/775/configuration_registry_data_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_registry_data_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_registry_data_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Clientbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_registry_data_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_registry_data_clientbound_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration clientbound
`minecraft:registry_data` frame for a `DIMENSION_TYPE` registry-key fixture with
an official empty entry list, dispatches that frame back to
`ClientboundRegistryDataPacket`, preserves the registry key and entry count,
and leaves no unread bytes after official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, decoded packet class, registry key, entry count, and remaining
decode byte count.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, the test passes and maps the official
`minecraft:registry_data` packet to the current compatibility alias
`Packet::PluginMessageClientbound` with channel `RegistryData` while consuming
the official body bytes.

```text
oracle/failures/775/configuration_registry_data_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_registry_data_clientbound_framed_dispatch.rust-fix-task.json
```

## Does Not Prove

This does not prove real registry contents,
`RegistrySynchronization.packRegistries` output, registry hydration,
Configuration completion, Play entry, world load, render readiness, or any
later client-load phase.
