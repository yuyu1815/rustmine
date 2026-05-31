# configuration_server_links_clientbound_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof package for Configuration
clientbound `minecraft:server_links` framed dispatch/decode without expanding
it into server-links UI behavior, trust/link-opening policy, Configuration
completion, Play entry, world load, render readiness, or runtime behavior.

## Evidence Map

```text
client.jar ClientboundServerLinksPacket(List<ServerLinks.UntrustedEntry>)
  -> ClientboundServerLinksPacket.STREAM_CODEC
    -> ConfigurationProtocols.CLIENTBOUND.codec().encode/decode
      -> ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets
        -> oracle/answers/775/configuration_server_links_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/configuration_server_links_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> packet::packet_by_id(775, Configuration, Clientbound, official id, official body)
                -> current result: ServerLinks compatibility identity and consumed body
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_server_links_clientbound_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `ClientboundServerLinksPacket(List<ServerLinks.UntrustedEntry>)`; `ClientboundServerLinksPacket.STREAM_CODEC`; `ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundServerLinksPacket)`; `ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundServerLinksPacket.links()`; `ServerLinks.UNTRUSTED_LINKS_STREAM_CODEC` |
| Bytecode witness | `_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundServerLinksPacket net.minecraft.server.ServerLinks net.minecraft.server.ServerLinks\\$UntrustedEntry net.minecraft.server.ServerLinks\\$KnownLinkType net.minecraft.network.protocol.common.CommonPacketTypes net.minecraft.network.protocol.configuration.ConfigurationProtocols` |
| Generated answer | `oracle/answers/775/configuration_server_links_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_server_links_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_server_links_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Clientbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_server_links_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_server_links_clientbound_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration clientbound
`minecraft:server_links` frame for one empty links list fixture, dispatches
that frame back to `ClientboundServerLinksPacket`, preserves the empty list,
and leaves no unread bytes after official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, decoded packet class, decoded fields, and remaining decode byte
count.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, the test passes and maps the official
`minecraft:server_links` packet to the current compatibility alias
`Packet::PluginMessageClientbound` with channel `ServerLinks`, preserving the
packet identity while consuming the official empty links-list body bytes.

```text
oracle/failures/775/configuration_server_links_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_server_links_clientbound_framed_dispatch.rust-fix-task.json
```

## Does Not Prove

This does not prove server-links UI behavior, trust/link-opening policy,
Configuration completion, Play entry, world load, render readiness, runtime
behavior, or any later client-load phase.
