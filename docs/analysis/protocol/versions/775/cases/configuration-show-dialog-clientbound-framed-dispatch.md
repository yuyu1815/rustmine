# configuration_show_dialog_clientbound_framed_dispatch

Purpose: document the Protocol 775 jar-backed proof package for Configuration
clientbound `minecraft:show_dialog` framed dispatch/decode without expanding
it into dialog UI display behavior, screen flow, registry-backed dialogs,
Configuration completion, Play entry, world load, render readiness, or runtime
behavior.

## Evidence Map

```text
client.jar ClientboundShowDialogPacket(Holder.direct(NoticeDialog))
  -> ClientboundShowDialogPacket.CONTEXT_FREE_STREAM_CODEC
    -> ConfigurationProtocols.CLIENTBOUND.codec().encode/decode
      -> ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets
        -> oracle/answers/775/configuration_show_dialog_clientbound_framed_dispatch.answer.jsonl
          -> oracle/test-manifests/775/configuration_show_dialog_clientbound_framed_dispatch.test-manifest.json
            -> oracle/rust-tests/tests/oracle_contracts.rs
              -> packet::packet_by_id(775, Configuration, Clientbound, official id, official body)
                -> current result: ShowDialog compatibility identity and consumed context-free dialog body
```

## Facts

| Field | Value |
|---|---|
| Case id | `configuration_show_dialog_clientbound_framed_dispatch` |
| Corridor | `Login -> Configuration -> Play` |
| Official source | `client.jar` `ClientboundShowDialogPacket(Holder.direct(NoticeDialog))`; `ClientboundShowDialogPacket.CONTEXT_FREE_STREAM_CODEC`; `Dialog.CONTEXT_FREE_STREAM_CODEC`; `ConfigurationProtocols.CLIENTBOUND.codec().encode/decode(ClientboundShowDialogPacket)`; `ConfigurationProtocols.CLIENTBOUND_TEMPLATE.details().listPackets(...)`; `ClientboundShowDialogPacket.dialog()`; `NoticeDialog.DEFAULT_ACTION` |
| Bytecode witness | `_tools/java/jdk-25-full/Contents/Home/bin/javap -classpath _analysis/minecraft-26.1.2/client.jar -c -p net.minecraft.network.protocol.common.ClientboundShowDialogPacket net.minecraft.server.dialog.Dialog net.minecraft.server.dialog.NoticeDialog net.minecraft.server.dialog.CommonDialogData net.minecraft.server.dialog.ActionButton net.minecraft.server.dialog.CommonButtonData net.minecraft.network.protocol.common.CommonPacketTypes net.minecraft.network.protocol.configuration.ConfigurationProtocols` shows the context-free dialog codec and the clientbound table entry |
| Generated answer | `oracle/answers/775/configuration_show_dialog_clientbound_framed_dispatch.answer.jsonl` |
| Test manifest | `oracle/test-manifests/775/configuration_show_dialog_clientbound_framed_dispatch.test-manifest.json` |
| Rust oracle test | `oracle/rust-tests/tests/oracle_contracts.rs::configuration_show_dialog_clientbound_framed_dispatch_matches_official_oracle_answer` |
| Internal owner under test | `stevenarella/protocol/src/protocol/mod.rs`; `packet::packet_by_id(775, State::Configuration, Direction::Clientbound, packet id, body)` |
| Failure packet | `oracle/failures/775/configuration_show_dialog_clientbound_framed_dispatch.why-what-answer.jsonl` |
| Rust fix task | `oracle/failures/775/configuration_show_dialog_clientbound_framed_dispatch.rust-fix-task.json` |
| Traceability row | `docs/analysis/protocol/versions/775/traceability.md` |

## Proves

The official jar emits the full Configuration clientbound
`minecraft:show_dialog` frame for one direct NoticeDialog fixture through
`ClientboundShowDialogPacket.CONTEXT_FREE_STREAM_CODEC`, dispatches that frame
back to `ClientboundShowDialogPacket`, preserves the decoded direct
NoticeDialog fixture fields, and leaves no unread bytes after official decode.

The generated answer, not this note, owns the exact packet id, frame bytes,
body bytes, decoded packet class, decoded dialog fields, and remaining decode
byte count.

## Current Rust Result

The manifest-declared Rust oracle test reads the generated answer and calls
Stevenarella `packet::packet_by_id` with the official packet id and body. In
the current checkout, the test passes and maps the official
`minecraft:show_dialog` packet to the current compatibility alias
`Packet::PluginMessageClientbound` with channel `ShowDialog` while consuming
and preserving the official context-free dialog body.

```text
oracle/failures/775/configuration_show_dialog_clientbound_framed_dispatch.why-what-answer.jsonl
oracle/failures/775/configuration_show_dialog_clientbound_framed_dispatch.rust-fix-task.json
```

## Does Not Prove

This does not prove dialog UI display behavior, screen flow, registry-backed
dialogs, custom actions, Configuration completion, Play entry, registry
hydration, world load, render readiness, or any later client-load phase.
