# Protocol 775

Purpose: keep Protocol 775 work tied to official answers, reset-proof tests,
and the relevant client-load/playability claim without turning this version
shard into a root-level rule.

## Spatial Map

```text
official jar function
  -> oracle case
    -> contract
      -> answer
        -> test manifest
          -> project-level Rust oracle test
            -> internal owner under test
              -> corridor milestone
```

## Index

| Need | Location |
|---|---|
| Traceability map | [traceability.md](traceability.md) |
| `configuration_client_information_framed_dispatch` case note | [cases/configuration-client-information-framed-dispatch.md](cases/configuration-client-information-framed-dispatch.md) |
| `configuration_cookie_request_framed_dispatch` case note | [cases/configuration-cookie-request-framed-dispatch.md](cases/configuration-cookie-request-framed-dispatch.md) |
| `configuration_cookie_response_framed_dispatch` case note | [cases/configuration-cookie-response-framed-dispatch.md](cases/configuration-cookie-response-framed-dispatch.md) |
| `configuration_custom_payload_clientbound_framed_dispatch` case note | [cases/configuration-custom-payload-clientbound-framed-dispatch.md](cases/configuration-custom-payload-clientbound-framed-dispatch.md) |
| `configuration_custom_payload_framed_dispatch` case note | [cases/configuration-custom-payload-framed-dispatch.md](cases/configuration-custom-payload-framed-dispatch.md) |
| `configuration_disconnect_clientbound_framed_dispatch` case note | [cases/configuration-disconnect-clientbound-framed-dispatch.md](cases/configuration-disconnect-clientbound-framed-dispatch.md) |
| `configuration_reset_chat_clientbound_framed_dispatch` case note | [cases/configuration-reset-chat-clientbound-framed-dispatch.md](cases/configuration-reset-chat-clientbound-framed-dispatch.md) |
| `configuration_resource_pack_response_framed_dispatch` case note | [cases/configuration-resource-pack-response-framed-dispatch.md](cases/configuration-resource-pack-response-framed-dispatch.md) |
| `configuration_update_enabled_features_clientbound_framed_dispatch` case note | [cases/configuration-update-enabled-features-clientbound-framed-dispatch.md](cases/configuration-update-enabled-features-clientbound-framed-dispatch.md) |
| `configuration_update_tags_clientbound_framed_dispatch` case note | [cases/configuration-update-tags-clientbound-framed-dispatch.md](cases/configuration-update-tags-clientbound-framed-dispatch.md) |
| `configuration_select_known_packs_clientbound_framed_dispatch` case note | [cases/configuration-select-known-packs-clientbound-framed-dispatch.md](cases/configuration-select-known-packs-clientbound-framed-dispatch.md) |
| `configuration_custom_report_details_clientbound_framed_dispatch` case note | [cases/configuration-custom-report-details-clientbound-framed-dispatch.md](cases/configuration-custom-report-details-clientbound-framed-dispatch.md) |
| `configuration_server_links_clientbound_framed_dispatch` case note | [cases/configuration-server-links-clientbound-framed-dispatch.md](cases/configuration-server-links-clientbound-framed-dispatch.md) |
| `configuration_clear_dialog_clientbound_framed_dispatch` case note | [cases/configuration-clear-dialog-clientbound-framed-dispatch.md](cases/configuration-clear-dialog-clientbound-framed-dispatch.md) |
| `configuration_show_dialog_clientbound_framed_dispatch` case note | [cases/configuration-show-dialog-clientbound-framed-dispatch.md](cases/configuration-show-dialog-clientbound-framed-dispatch.md) |
| `configuration_code_of_conduct_clientbound_framed_dispatch` case note | [cases/configuration-code-of-conduct-clientbound-framed-dispatch.md](cases/configuration-code-of-conduct-clientbound-framed-dispatch.md) |
| `configuration_select_known_packs_framed_dispatch` case note | [cases/configuration-select-known-packs-framed-dispatch.md](cases/configuration-select-known-packs-framed-dispatch.md) |
| `configuration_custom_click_action_framed_dispatch` case note | [cases/configuration-custom-click-action-framed-dispatch.md](cases/configuration-custom-click-action-framed-dispatch.md) |
| `configuration_accept_code_of_conduct_framed_dispatch` case note | [cases/configuration-accept-code-of-conduct-framed-dispatch.md](cases/configuration-accept-code-of-conduct-framed-dispatch.md) |
| `configuration_keepalive_codec` case note | [cases/configuration-keepalive-codec.md](cases/configuration-keepalive-codec.md) |
| `configuration_keepalive_framed_dispatch` case note | [cases/configuration-keepalive-framed-dispatch.md](cases/configuration-keepalive-framed-dispatch.md) |
| `configuration_keepalive_clientbound_framed_dispatch` case note | [cases/configuration-keepalive-clientbound-framed-dispatch.md](cases/configuration-keepalive-clientbound-framed-dispatch.md) |
| `configuration_ping_pong_framed_dispatch` case note | [cases/configuration-ping-pong-framed-dispatch.md](cases/configuration-ping-pong-framed-dispatch.md) |
| `configuration_keepalive_runtime_send_helper` case note | [cases/configuration-keepalive-runtime-send-helper.md](cases/configuration-keepalive-runtime-send-helper.md) |
| `configuration_keepalive_runtime_protocol_echo` case note | [cases/configuration-keepalive-runtime-protocol-echo.md](cases/configuration-keepalive-runtime-protocol-echo.md) |
| `configuration_keepalive_runtime_spawn_reader_reaction` case note | [cases/configuration-keepalive-runtime-spawn-reader-reaction.md](cases/configuration-keepalive-runtime-spawn-reader-reaction.md) |
| `configuration_finish_framed_terminal` case note | [cases/configuration-finish-framed-terminal.md](cases/configuration-finish-framed-terminal.md) |
| Oracle workbench workflow | `.codex/skills/stevenarella-oracle-workbench/SKILL.md` |

## Evidence Snapshot

At this snapshot, `configuration_client_information_framed_dispatch`,
`configuration_cookie_request_framed_dispatch`,
`configuration_cookie_response_framed_dispatch`,
`configuration_custom_payload_clientbound_framed_dispatch`,
`configuration_custom_payload_framed_dispatch`,
`configuration_disconnect_clientbound_framed_dispatch`,
`configuration_reset_chat_clientbound_framed_dispatch`,
`configuration_keepalive_codec`,
`configuration_keepalive_framed_dispatch`,
`configuration_keepalive_clientbound_framed_dispatch`,
`configuration_ping_pong_framed_dispatch`,
`configuration_resource_pack_response_framed_dispatch`,
`configuration_update_enabled_features_clientbound_framed_dispatch`,
`configuration_update_tags_clientbound_framed_dispatch`,
`configuration_select_known_packs_clientbound_framed_dispatch`,
`configuration_custom_report_details_clientbound_framed_dispatch`,
`configuration_server_links_clientbound_framed_dispatch`,
`configuration_clear_dialog_clientbound_framed_dispatch`,
`configuration_show_dialog_clientbound_framed_dispatch`,
`configuration_code_of_conduct_clientbound_framed_dispatch`,
`configuration_select_known_packs_framed_dispatch`,
`configuration_custom_click_action_framed_dispatch`,
`configuration_accept_code_of_conduct_framed_dispatch`, and
`configuration_finish_framed_terminal` are the passing jar-backed answer rows
in this 775 shard. Their answers were regenerated from the official client jar
and the manifest-declared Rust oracle tests passed against the current Leafish
checkout.

`configuration_custom_payload_framed_dispatch` is packet-support evidence for
one official BrandPayload fixture only. It does not prove arbitrary
plugin-channel handling, payload routing policy, Configuration completion, or
Play entry.

`configuration_custom_payload_clientbound_framed_dispatch` is packet-support
evidence for one official clientbound BrandPayload fixture only. It does not
prove arbitrary plugin-channel handling, payload routing policy, Configuration
completion, registry hydration, Play entry, or runtime behavior.

`configuration_disconnect_clientbound_framed_dispatch` is packet-support
evidence for one official empty literal Component reason fixture only. It does
not prove UI disconnect handling, screen flow, Configuration completion,
registry hydration, Play entry, or runtime behavior.

`configuration_reset_chat_clientbound_framed_dispatch` is packet-support
evidence for the official singleton empty-body reset_chat packet only. It does
not prove chat UI reset behavior, screen flow, Configuration completion,
registry hydration, Play entry, or runtime behavior.

`configuration_cookie_response_framed_dispatch` is packet-support evidence for
one non-null payload fixture only. It does not prove cookie storage policy,
cookie request/response runtime behavior, Configuration completion, or Play
entry.

`configuration_cookie_request_framed_dispatch` is packet-support evidence for
one Identifier key fixture only. It does not prove cookie storage policy,
cookie request/response runtime behavior, Configuration completion, or Play
entry.

`configuration_keepalive_runtime_send_helper` is
also passing as a root-owned runtime-send probe that reuses the official
serverbound keep_alive answer, and
`configuration_keepalive_runtime_protocol_echo` is passing as a root-owned
protocol-crate socket echo probe. `configuration_keepalive_runtime_spawn_reader_reaction`
is passing as a root-owned runtime probe for the same factored keep_alive branch
used by `Server::spawn_reader`. No broader Protocol 775 or client-load phase is
complete from those proofs.

The next runtime gap remains outside keep_alive: Configuration completion /
Configuration-to-Play transition and later registry, Play, world, render, and
interaction readiness are still unproven.

`configuration_accept_code_of_conduct_framed_dispatch` is packet-support
evidence only. It does not prove UI consent flow, legal acceptance semantics,
Configuration completion, or Play entry.

`configuration_update_enabled_features_clientbound_framed_dispatch` is
packet-support evidence for one official empty feature-set fixture only. It
does not prove feature registry hydration, enabled-feature semantics,
Configuration completion, or Play entry.

`configuration_update_tags_clientbound_framed_dispatch` is packet-support
evidence for one official empty tag-payload map fixture only. It does not prove
real tag contents, tag registry hydration, Configuration completion, or Play
entry.

`configuration_select_known_packs_clientbound_framed_dispatch` is
packet-support evidence for one official empty known-pack list fixture only. It
does not prove registry hydration, known-pack negotiation completion,
Configuration completion, or Play entry.

`configuration_custom_report_details_clientbound_framed_dispatch` is
packet-support evidence for one official empty details map fixture only. It
does not prove report UI behavior, moderation/reporting semantics,
Configuration completion, Play entry, world load, render readiness, or runtime
behavior.

`configuration_server_links_clientbound_framed_dispatch` is packet-support
evidence for one official empty links list fixture only. It does not prove
server-links UI behavior, trust/link-opening policy, Configuration completion,
Play entry, world load, render readiness, or runtime behavior.

`configuration_clear_dialog_clientbound_framed_dispatch` is packet-support
evidence for the official singleton empty-body clear_dialog packet only. It
does not prove dialog UI clearing behavior, screen flow, Configuration
completion, Play entry, world load, render readiness, or runtime behavior.

`configuration_show_dialog_clientbound_framed_dispatch` is packet-support
evidence for one official direct NoticeDialog context-free fixture only. It
does not prove dialog UI display behavior, screen flow, registry-backed
dialogs, custom actions, Configuration completion, Play entry, world load,
render readiness, or runtime behavior.

`configuration_code_of_conduct_clientbound_framed_dispatch` is packet-support
evidence for one official String fixture only. It does not prove UI consent
flow, legal acceptance semantics, Configuration completion, Play entry, world
load, render readiness, or runtime behavior. The generated Configuration
clientbound packet table now has jar-backed packet-support rows through
`minecraft:code_of_conduct` / `0x13`, the last clientbound table entry in the
current official 26.1.2 answer.
