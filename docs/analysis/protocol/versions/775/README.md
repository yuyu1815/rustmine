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
| `handshake_intention_framed_dispatch` case note | [cases/handshake-intention-framed-dispatch.md](cases/handshake-intention-framed-dispatch.md) |
| `login_hello_serverbound_framed_dispatch` case note | [cases/login-hello-serverbound-framed-dispatch.md](cases/login-hello-serverbound-framed-dispatch.md) |
| `login_key_serverbound_framed_dispatch` case note | [cases/login-key-serverbound-framed-dispatch.md](cases/login-key-serverbound-framed-dispatch.md) |
| `login_custom_query_answer_serverbound_framed_dispatch` case note | [cases/login-custom-query-answer-serverbound-framed-dispatch.md](cases/login-custom-query-answer-serverbound-framed-dispatch.md) |
| `login_acknowledged_serverbound_framed_dispatch` case note | [cases/login-acknowledged-serverbound-framed-dispatch.md](cases/login-acknowledged-serverbound-framed-dispatch.md) |
| `login_cookie_response_serverbound_framed_dispatch` case note | [cases/login-cookie-response-serverbound-framed-dispatch.md](cases/login-cookie-response-serverbound-framed-dispatch.md) |
| `login_disconnect_clientbound_framed_dispatch` case note | [cases/login-disconnect-clientbound-framed-dispatch.md](cases/login-disconnect-clientbound-framed-dispatch.md) |
| `login_hello_clientbound_framed_dispatch` case note | [cases/login-hello-clientbound-framed-dispatch.md](cases/login-hello-clientbound-framed-dispatch.md) |
| `login_finished_clientbound_framed_dispatch` case note | [cases/login-finished-clientbound-framed-dispatch.md](cases/login-finished-clientbound-framed-dispatch.md) |
| `login_compression_clientbound_framed_dispatch` case note | [cases/login-compression-clientbound-framed-dispatch.md](cases/login-compression-clientbound-framed-dispatch.md) |
| `login_custom_query_clientbound_framed_dispatch` case note | [cases/login-custom-query-clientbound-framed-dispatch.md](cases/login-custom-query-clientbound-framed-dispatch.md) |
| `login_cookie_request_clientbound_framed_dispatch` case note | [cases/login-cookie-request-clientbound-framed-dispatch.md](cases/login-cookie-request-clientbound-framed-dispatch.md) |
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
| `play_bundle_delimiter_clientbound_framed_dispatch` case note | [cases/play-bundle-delimiter-clientbound-framed-dispatch.md](cases/play-bundle-delimiter-clientbound-framed-dispatch.md) |
| `play_add_entity_clientbound_framed_dispatch` case note | [cases/play-add-entity-clientbound-framed-dispatch.md](cases/play-add-entity-clientbound-framed-dispatch.md) |
| `play_animate_clientbound_framed_dispatch` case note | [cases/play-animate-clientbound-framed-dispatch.md](cases/play-animate-clientbound-framed-dispatch.md) |
| `play_award_stats_clientbound_framed_dispatch` case note | [cases/play-award-stats-clientbound-framed-dispatch.md](cases/play-award-stats-clientbound-framed-dispatch.md) |
| `play_block_changed_ack_clientbound_framed_dispatch` case note | [cases/play-block-changed-ack-clientbound-framed-dispatch.md](cases/play-block-changed-ack-clientbound-framed-dispatch.md) |
| Oracle workbench workflow | `.codex/skills/stevenarella-oracle-workbench/SKILL.md` |

## Evidence Snapshot

At this snapshot, `handshake_intention_framed_dispatch`,
`login_hello_serverbound_framed_dispatch`,
`login_key_serverbound_framed_dispatch`,
`login_custom_query_answer_serverbound_framed_dispatch`,
`login_acknowledged_serverbound_framed_dispatch`,
`login_cookie_response_serverbound_framed_dispatch`,
`login_disconnect_clientbound_framed_dispatch`,
`login_hello_clientbound_framed_dispatch`,
`login_finished_clientbound_framed_dispatch`,
`login_compression_clientbound_framed_dispatch`,
`login_custom_query_clientbound_framed_dispatch`,
`login_cookie_request_clientbound_framed_dispatch`,
`configuration_client_information_framed_dispatch`,
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
`configuration_accept_code_of_conduct_framed_dispatch`,
`configuration_finish_framed_terminal`,
`play_bundle_delimiter_clientbound_framed_dispatch`,
`play_add_entity_clientbound_framed_dispatch`,
`play_animate_clientbound_framed_dispatch`, and
`play_award_stats_clientbound_framed_dispatch`, and
`play_block_changed_ack_clientbound_framed_dispatch` are the passing jar-backed
answer rows in this 775 shard. Their answers were regenerated from the
official client jar and the manifest-declared Rust oracle tests passed against
the current Leafish checkout.

`handshake_intention_framed_dispatch` is packet-support evidence for one
official LOGIN-intent fixture only. It does not prove Login authentication,
Configuration entry, or client-load completion. It is the only current official
Protocol 775 Handshaking serverbound table row in the generated answer.

`login_hello_serverbound_framed_dispatch` is packet-support evidence for one
official Login serverbound hello fixture only. It proves the official
`minecraft:hello` / `0x00` row, name field, profileId field, and full body
consumption through Stevenarella dispatch. It does not prove authentication
success, encryption/key exchange, login acknowledgement, Configuration entry,
or client-load completion.

`login_key_serverbound_framed_dispatch` is packet-support evidence for one
official minimal Login serverbound key fixture only. It proves the official
`minecraft:key` / `0x01` row, `keybytes`/`encryptedChallenge` body order, and
full body consumption through Stevenarella dispatch. It does not prove
encryption success, private-key validation, authentication success, login
acknowledgement, Configuration entry, or client-load completion.

`login_custom_query_answer_serverbound_framed_dispatch` is packet-support
evidence for one official null-payload Login serverbound custom_query_answer
fixture only. It proves the official `minecraft:custom_query_answer` / `0x02`
row, transaction id field, nullable payload marker body, and full body
consumption through Stevenarella dispatch. It does not prove plugin channel
handling, custom payload semantics, Configuration entry, or client-load
completion.

`login_acknowledged_serverbound_framed_dispatch` is packet-support evidence
for the official singleton Login serverbound login_acknowledged fixture only.
It proves the official `minecraft:login_acknowledged` / `0x03` row,
empty-body unit codec, terminal flag, and full body consumption through
Stevenarella dispatch. It does not prove Configuration entry, state transition
handling, Play readiness, or client-load completion.

`login_cookie_response_serverbound_framed_dispatch` is packet-support evidence
for one official Login serverbound cookie_response non-null payload fixture
only. It proves the official `minecraft:cookie_response` / `0x04` row,
Identifier key field, nullable payload marker/body, and full body consumption
through Stevenarella dispatch. It does not prove cookie storage policy,
cookie request/response runtime behavior, Configuration entry, Play readiness,
or client-load completion.

`login_disconnect_clientbound_framed_dispatch` is packet-support evidence for
one official Login clientbound login_disconnect empty literal Component reason
fixture only. It proves the official `minecraft:login_disconnect` / `0x00`
row and full body consumption through Stevenarella dispatch. It does not prove
UI disconnect handling, screen flow, authentication failure handling,
Configuration entry, Play readiness, or client-load completion.

`login_hello_clientbound_framed_dispatch` is packet-support evidence for one
official Login clientbound hello fixture only. It proves the official
`minecraft:hello` / `0x01` row, `serverId`, `publicKey`, `challenge`, and
`shouldAuthenticate` body order, and full body consumption through Stevenarella
dispatch. It does not prove encryption success, authentication success, key
validation, login state transition handling, Configuration entry, Play
readiness, or client-load completion.

`login_finished_clientbound_framed_dispatch` is packet-support evidence for
one official Login clientbound login_finished fixture only. It proves the
official `minecraft:login_finished` / `0x02` row, GameProfile UUID/name,
empty property count from `PropertyMap.EMPTY`, terminal flag, and full body
consumption through Stevenarella dispatch. It does not prove authentication
success, Login-to-Configuration state transition handling, profile property
semantics, skin/session trust, Configuration entry, Play readiness, or
client-load completion.

`login_compression_clientbound_framed_dispatch` is packet-support evidence for
one official Login clientbound login_compression fixture only. It proves the
official `minecraft:login_compression` / `0x03` row, one VarInt
`compressionThreshold` field, and full body consumption through Stevenarella
dispatch. It does not prove compression negotiation policy, connection
compression activation, Login-to-Configuration state transition handling,
Configuration entry, Play readiness, or client-load completion.

`login_custom_query_clientbound_framed_dispatch` is packet-support evidence
for one official Login clientbound custom_query fixture only. It proves the
official `minecraft:custom_query` / `0x04` row, transactionId VarInt, payload
Identifier, empty `DiscardedQueryPayload` body, and full body consumption
through Stevenarella dispatch. It does not prove plugin channel handling,
custom query semantics, login acknowledgement behavior,
Login-to-Configuration state transition handling, Configuration entry, Play
readiness, or client-load completion.

`login_cookie_request_clientbound_framed_dispatch` is packet-support evidence
for one official Login clientbound cookie_request fixture only. It proves the
official `minecraft:cookie_request` / `0x05` row, one Identifier key field,
and full body consumption through Stevenarella dispatch. It does not prove
cookie storage policy, cookie request/response runtime behavior,
Login-to-Configuration state transition handling, Configuration entry, Play
readiness, or client-load completion. Login clientbound packet-support is now
complete through the current official `LoginProtocols.CLIENTBOUND_TEMPLATE`
rows.

`play_bundle_delimiter_clientbound_framed_dispatch` is packet-support evidence
for the official Play clientbound bundle_delimiter registered singleton only.
It proves the official `minecraft:bundle_delimiter` / `0x00` row, empty body,
and full body consumption through Stevenarella dispatch. It does not prove
bundle grouping behavior, Play state transition handling, world load, spawn
readiness, render readiness, or client-load completion.

`play_add_entity_clientbound_framed_dispatch` is packet-support evidence for
one official Play clientbound add_entity constructor fixture only. It proves
the official `minecraft:add_entity` / `0x01` row, body order, built-in
`minecraft:pig` entity type registry id, zero `Vec3.LP_STREAM_CODEC` movement
fixture, and full body consumption through Stevenarella dispatch. It does not
prove arbitrary entity registry contents, initialized `Entity`/`ServerEntity`
behavior, spawn readiness, world load, render readiness, or client-load
completion. The generated answer observed 141 Play clientbound rows from
`GameProtocols.CLIENTBOUND_TEMPLATE`; first Play clientbound rows are
`minecraft:bundle_delimiter` / `0x00`, `minecraft:add_entity` / `0x01`,
`minecraft:animate` / `0x02`, `minecraft:award_stats` / `0x03`, and
`minecraft:block_changed_ack` / `0x04`.

`play_animate_clientbound_framed_dispatch` is packet-support evidence for one
official Play clientbound animate `STREAM_CODEC` decode fixture only. It proves
the official `minecraft:animate` / `0x02` row, body order, entity id VarInt,
`SWING_MAIN_HAND` unsigned-byte action constant `0`, and full body consumption
through Stevenarella dispatch. It does not prove entity existence, animation
semantics, initialized `Entity`/`Level` behavior, spawn readiness, world load,
render readiness, or client-load completion.

`play_award_stats_clientbound_framed_dispatch` is packet-support evidence for
one official Play clientbound award_stats empty-stats fixture only. It proves
the official `minecraft:award_stats` / `0x03` row, body shape for an
`Object2IntMap<Stat<?>>` as VarInt count followed by Stat key and VarInt value
per entry, the empty fixture body count `0`, and full body consumption through
Stevenarella dispatch. It does not prove non-empty Stat registry entry
decoding, stat semantics, UI/stat screen behavior, spawn readiness, world load,
render readiness, or client-load completion.

`play_block_changed_ack_clientbound_framed_dispatch` is packet-support
evidence for one official Play clientbound block_changed_ack sequence fixture
only. It proves the official `minecraft:block_changed_ack` / `0x04` row, body
shape as a single sequence VarInt, fixture sequence `12345`, and full body
consumption through Stevenarella dispatch. It does not prove block prediction
semantics, client world correction behavior, initialized game state, spawn
readiness, world load, render readiness, or client-load completion. The next
packet-support target by the same ordering rule is Play clientbound
`minecraft:block_destruction` / `0x05`.

`play_block_destruction_clientbound_framed_dispatch` is packet-support
evidence for one official Play clientbound block_destruction breaker id, block
position, and progress fixture only. It proves the official
`minecraft:block_destruction` / `0x05` row, body shape as breaker id VarInt,
BlockPos, and unsigned-byte progress, fixture breaker id `123`, position
`x=12, y=64, z=-7`, progress `5`, and full body consumption through
Stevenarella dispatch. It does not prove block break animation semantics,
entity existence for the breaker id, client world state, initialized game
state, spawn readiness, world load, render readiness, or client-load
completion.

`play_block_entity_data_clientbound_framed_dispatch` is packet-support
evidence for one official Play clientbound block_entity_data block position,
built-in chest block entity type, and empty tag fixture only. It proves the
official `minecraft:block_entity_data` / `0x06` row, body shape as BlockPos,
block entity type registry id, and trusted compound tag, fixture position
`x=12, y=64, z=-7`, type `minecraft:chest`, empty tag `{}`, and full body
consumption through Stevenarella dispatch. It requires bootstrapped built-in
registries but not initialized `Level`, `BlockEntity`, or game state. It does
not prove block entity semantics, NBT schema, world/chunk state, initialized
game state, spawn readiness, world load, render readiness, or client-load
completion. The next packet-support target by the same ordering rule is Play
clientbound `minecraft:block_event` / `0x07`.

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
