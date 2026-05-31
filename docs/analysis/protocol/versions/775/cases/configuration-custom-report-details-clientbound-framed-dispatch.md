# configuration_custom_report_details_clientbound_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof package for Configuration
clientbound `minecraft:custom_report_details` framed dispatch/decode without
expanding it into report UI behavior, moderation/reporting semantics,
Configuration completion, Play entry, world load, render readiness, or runtime
behavior.

## Evidence Map

```text
client.jar ClientboundCustomReportDetailsPacket(Map<String, String>)
  -> ClientboundCustomReportDetailsPacket.STREAM_CODEC
    -> ConfigurationProtocols.CLIENTBOUND.codec().encode/decode
      -> ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets
        -> oracle/answers/775/configuration_custom_report_details_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/configuration_custom_report_details_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> packet::packet_by_id(775, Configuration, Clientbound, official id, official body)
                -> current result: CustomReportDetails compatibility identity and consumed body
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_custom_report_details_clientbound_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `ClientboundCustomReportDetailsPacket(Map<String, String>)`; `ClientboundCustomReportDetailsPacket.STREAM_CODEC`; `ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundCustomReportDetailsPacket)`; `ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundCustomReportDetailsPacket.details()` |
| Bytecode witness | `_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundCustomReportDetailsPacket net.minecraft.network.protocol.common.CommonPacketTypes net.minecraft.network.protocol.configuration.ConfigurationProtocols` |
| Generated answer | `oracle/answers/775/configuration_custom_report_details_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_custom_report_details_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_custom_report_details_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Clientbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_custom_report_details_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_custom_report_details_clientbound_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration clientbound
`minecraft:custom_report_details` frame for one empty details map fixture,
dispatches that frame back to `ClientboundCustomReportDetailsPacket`, preserves
the empty map, and leaves no unread bytes after official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, decoded packet class, decoded fields, and remaining decode byte
count.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, the test passes and maps the official
`minecraft:custom_report_details` packet to the current compatibility alias
`Packet::PluginMessageClientbound` with channel `CustomReportDetails`,
preserving the packet identity while consuming the official details-map body
bytes.

```text
oracle/failures/775/configuration_custom_report_details_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_custom_report_details_clientbound_framed_dispatch.rust-fix-task.json
```

## Does Not Prove

This does not prove report UI behavior, moderation/reporting semantics,
Configuration completion, Play entry, world load, render readiness, runtime
behavior, or any later client-load phase.
