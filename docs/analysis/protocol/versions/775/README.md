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
| `configuration_resource_pack_response_framed_dispatch` case note | [cases/configuration-resource-pack-response-framed-dispatch.md](cases/configuration-resource-pack-response-framed-dispatch.md) |
| `configuration_select_known_packs_framed_dispatch` case note | [cases/configuration-select-known-packs-framed-dispatch.md](cases/configuration-select-known-packs-framed-dispatch.md) |
| `configuration_custom_click_action_framed_dispatch` case note | [cases/configuration-custom-click-action-framed-dispatch.md](cases/configuration-custom-click-action-framed-dispatch.md) |
| `configuration_accept_code_of_conduct_framed_dispatch` case note | [cases/configuration-accept-code-of-conduct-framed-dispatch.md](cases/configuration-accept-code-of-conduct-framed-dispatch.md) |
| `configuration_keepalive_codec` case note | [cases/configuration-keepalive-codec.md](cases/configuration-keepalive-codec.md) |
| `configuration_keepalive_framed_dispatch` case note | [cases/configuration-keepalive-framed-dispatch.md](cases/configuration-keepalive-framed-dispatch.md) |
| `configuration_keepalive_clientbound_framed_dispatch` case note | [cases/configuration-keepalive-clientbound-framed-dispatch.md](cases/configuration-keepalive-clientbound-framed-dispatch.md) |
| `configuration_ping_pong_framed_dispatch` case note | [cases/configuration-ping-pong-framed-dispatch.md](cases/configuration-ping-pong-framed-dispatch.md) |
| `configuration_keepalive_runtime_send_helper` case note | [cases/configuration-keepalive-runtime-send-helper.md](cases/configuration-keepalive-runtime-send-helper.md) |
| `configuration_keepalive_runtime_protocol_echo` case note | [cases/configuration-keepalive-runtime-protocol-echo.md](cases/configuration-keepalive-runtime-protocol-echo.md) |
| `configuration_finish_framed_terminal` case note | [cases/configuration-finish-framed-terminal.md](cases/configuration-finish-framed-terminal.md) |
| Oracle workbench workflow | `.codex/skills/stevenarella-oracle-workbench/SKILL.md` |

## Evidence Snapshot

At this snapshot, `configuration_client_information_framed_dispatch`,
`configuration_keepalive_codec`,
`configuration_keepalive_framed_dispatch`,
`configuration_keepalive_clientbound_framed_dispatch`,
`configuration_ping_pong_framed_dispatch`,
`configuration_resource_pack_response_framed_dispatch`,
`configuration_select_known_packs_framed_dispatch`,
`configuration_custom_click_action_framed_dispatch`,
`configuration_accept_code_of_conduct_framed_dispatch`, and
`configuration_finish_framed_terminal` are the proven jar-backed answer rows in
this 775 shard. In the current run, their answers were regenerated from the
official client jar and the manifest-declared Rust oracle tests passed against
the current Leafish checkout. `configuration_keepalive_runtime_send_helper` is
also passing as a root-owned runtime-send probe that reuses the official
serverbound keep_alive answer, and
`configuration_keepalive_runtime_protocol_echo` is passing as a root-owned
protocol-crate socket echo probe. No broader Protocol 775 or client-load phase
is complete from those proofs.

`configuration_accept_code_of_conduct_framed_dispatch` is packet-support
evidence only. It does not prove UI consent flow, legal acceptance semantics,
Configuration completion, or Play entry.
