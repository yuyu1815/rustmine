# configuration_select_known_packs_clientbound_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof package for Configuration
clientbound `minecraft:select_known_packs` framed dispatch/decode without
expanding it into registry hydration, known-pack negotiation completion,
Configuration completion, Play entry, world load, render readiness, or runtime
behavior.

## Evidence Map

```text
client.jar ClientboundSelectKnownPacks(List<KnownPack>)
  -> ClientboundSelectKnownPacks.STREAM_CODEC
    -> ConfigurationProtocols.CLIENTBOUND.codec().encode/decode
      -> ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets
        -> oracle/answers/775/configuration_select_known_packs_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/configuration_select_known_packs_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> packet::packet_by_id(775, Configuration, Clientbound, official id, official body)
                -> current result: SelectKnownPacks compatibility identity and consumed body
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_select_known_packs_clientbound_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `ClientboundSelectKnownPacks(List<KnownPack>)`; `ClientboundSelectKnownPacks.STREAM_CODEC`; `ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundSelectKnownPacks)`; `ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundSelectKnownPacks.knownPacks()` |
| Bytecode witness | `_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.configuration.ClientboundSelectKnownPacks net.minecraft.network.protocol.configuration.ConfigurationProtocols net.minecraft.network.protocol.configuration.ConfigurationPacketTypes net.minecraft.server.packs.repository.KnownPack` |
| Generated answer | `oracle/answers/775/configuration_select_known_packs_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_select_known_packs_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_select_known_packs_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Clientbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_select_known_packs_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_select_known_packs_clientbound_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration clientbound
`minecraft:select_known_packs` frame for one empty known-pack list fixture,
dispatches that frame back to `ClientboundSelectKnownPacks`, preserves the
empty list, and leaves no unread bytes after official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, decoded packet class, decoded fields, and remaining decode byte
count.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, the test passes and maps the official
`minecraft:select_known_packs` packet to the current compatibility alias
`Packet::PluginMessageClientbound` with channel `SelectKnownPacks`, preserving
the packet identity while consuming the official known-pack list body bytes.

```text
oracle/failures/775/configuration_select_known_packs_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_select_known_packs_clientbound_framed_dispatch.rust-fix-task.json
```

## Does Not Prove

This does not prove registry hydration, known-pack negotiation completion,
Configuration completion, Play entry, world load, render readiness, runtime
behavior, or any later client-load phase.
