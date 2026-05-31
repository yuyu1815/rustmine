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
| Traceability map | `docs/analysis/protocol/versions/775/traceability.md` |
| `handshake_intention_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/handshake-intention-framed-dispatch.md` |
| `login_hello_serverbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/login-hello-serverbound-framed-dispatch.md` |
| `login_key_serverbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/login-key-serverbound-framed-dispatch.md` |
| `login_custom_query_answer_serverbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/login-custom-query-answer-serverbound-framed-dispatch.md` |
| `login_acknowledged_serverbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/login-acknowledged-serverbound-framed-dispatch.md` |
| `login_cookie_response_serverbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/login-cookie-response-serverbound-framed-dispatch.md` |
| `login_disconnect_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/login-disconnect-clientbound-framed-dispatch.md` |
| `login_hello_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/login-hello-clientbound-framed-dispatch.md` |
| `login_finished_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/login-finished-clientbound-framed-dispatch.md` |
| `login_compression_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/login-compression-clientbound-framed-dispatch.md` |
| `login_custom_query_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/login-custom-query-clientbound-framed-dispatch.md` |
| `login_cookie_request_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/login-cookie-request-clientbound-framed-dispatch.md` |
| `configuration_client_information_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-client-information-framed-dispatch.md` |
| `configuration_cookie_request_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-cookie-request-framed-dispatch.md` |
| `configuration_cookie_response_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-cookie-response-framed-dispatch.md` |
| `configuration_custom_payload_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-custom-payload-clientbound-framed-dispatch.md` |
| `configuration_custom_payload_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-custom-payload-framed-dispatch.md` |
| `configuration_disconnect_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-disconnect-clientbound-framed-dispatch.md` |
| `configuration_reset_chat_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-reset-chat-clientbound-framed-dispatch.md` |
| `configuration_resource_pack_response_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-resource-pack-response-framed-dispatch.md` |
| `configuration_update_enabled_features_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-update-enabled-features-clientbound-framed-dispatch.md` |
| `configuration_update_tags_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-update-tags-clientbound-framed-dispatch.md` |
| `configuration_select_known_packs_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-select-known-packs-clientbound-framed-dispatch.md` |
| `configuration_custom_report_details_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-custom-report-details-clientbound-framed-dispatch.md` |
| `configuration_server_links_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-server-links-clientbound-framed-dispatch.md` |
| `configuration_clear_dialog_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-clear-dialog-clientbound-framed-dispatch.md` |
| `configuration_show_dialog_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-show-dialog-clientbound-framed-dispatch.md` |
| `configuration_code_of_conduct_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-code-of-conduct-clientbound-framed-dispatch.md` |
| `configuration_select_known_packs_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-select-known-packs-framed-dispatch.md` |
| `configuration_custom_click_action_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-custom-click-action-framed-dispatch.md` |
| `configuration_accept_code_of_conduct_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-accept-code-of-conduct-framed-dispatch.md` |
| `configuration_keepalive_codec` case note | `docs/analysis/protocol/versions/775/cases/configuration-keepalive-codec.md` |
| `configuration_keepalive_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-keepalive-framed-dispatch.md` |
| `configuration_keepalive_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-keepalive-clientbound-framed-dispatch.md` |
| `configuration_ping_pong_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/configuration-ping-pong-framed-dispatch.md` |
| `configuration_keepalive_runtime_send_helper` case note | `docs/analysis/protocol/versions/775/cases/configuration-keepalive-runtime-send-helper.md` |
| `configuration_keepalive_runtime_protocol_echo` case note | `docs/analysis/protocol/versions/775/cases/configuration-keepalive-runtime-protocol-echo.md` |
| `configuration_keepalive_runtime_spawn_reader_reaction` case note | `docs/analysis/protocol/versions/775/cases/configuration-keepalive-runtime-spawn-reader-reaction.md` |
| `configuration_finish_framed_terminal` case note | `docs/analysis/protocol/versions/775/cases/configuration-finish-framed-terminal.md` |
| `play_bundle_delimiter_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-bundle-delimiter-clientbound-framed-dispatch.md` |
| `play_add_entity_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-add-entity-clientbound-framed-dispatch.md` |
| `play_animate_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-animate-clientbound-framed-dispatch.md` |
| `play_award_stats_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-award-stats-clientbound-framed-dispatch.md` |
| `play_block_changed_ack_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-block-changed-ack-clientbound-framed-dispatch.md` |
| `play_container_set_content_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-container-set-content-clientbound-framed-dispatch.md` |
| `play_container_set_data_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-container-set-data-clientbound-framed-dispatch.md` |
| `play_container_set_slot_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-container-set-slot-clientbound-framed-dispatch.md` |
| `play_cookie_request_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-cookie-request-clientbound-framed-dispatch.md` |
| `play_cooldown_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-cooldown-clientbound-framed-dispatch.md` |
| `play_custom_chat_completions_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-custom-chat-completions-clientbound-framed-dispatch.md` |
| `play_custom_payload_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-custom-payload-clientbound-framed-dispatch.md` |
| `play_custom_report_details_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-custom-report-details-clientbound-framed-dispatch.md` |
| Protocol 775 Play clientbound deferred rows | `docs/analysis/protocol/versions/775/play-clientbound-deferred.md` |
| `play_disconnect_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-disconnect-clientbound-framed-dispatch.md` |
| `play_entity_position_sync_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-entity-position-sync-clientbound-framed-dispatch.md` |
| `play_forget_level_chunk_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-forget-level-chunk-clientbound-framed-dispatch.md` |
| `play_game_event_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-game-event-clientbound-framed-dispatch.md` |
| `play_mount_screen_open_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-mount-screen-open-clientbound-framed-dispatch.md` |
| `play_hurt_animation_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-hurt-animation-clientbound-framed-dispatch.md` |
| `play_initialize_border_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-initialize-border-clientbound-framed-dispatch.md` |
| `play_keep_alive_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-keep-alive-clientbound-framed-dispatch.md` |
| `play_set_border_center_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-border-center-clientbound-framed-dispatch.md` |
| `play_set_border_lerp_size_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-border-lerp-size-clientbound-framed-dispatch.md` |
| `play_set_border_size_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-border-size-clientbound-framed-dispatch.md` |
| `play_set_border_warning_delay_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-border-warning-delay-clientbound-framed-dispatch.md` |
| `play_set_border_warning_distance_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-border-warning-distance-clientbound-framed-dispatch.md` |
| `play_move_vehicle_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-move-vehicle-clientbound-framed-dispatch.md` |
| `play_open_book_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-open-book-clientbound-framed-dispatch.md` |
| `play_set_entity_motion_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-entity-motion-clientbound-framed-dispatch.md` |
| `play_set_experience_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-experience-clientbound-framed-dispatch.md` |
| `play_set_health_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-health-clientbound-framed-dispatch.md` |
| `play_set_held_slot_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-held-slot-clientbound-framed-dispatch.md` |
| `play_set_simulation_distance_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-simulation-distance-clientbound-framed-dispatch.md` |
| `play_server_links_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-server-links-clientbound-framed-dispatch.md` |
| `play_set_display_objective_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-display-objective-clientbound-framed-dispatch.md` |
| `play_set_entity_link_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-entity-link-clientbound-framed-dispatch.md` |
| `play_set_score_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-score-clientbound-framed-dispatch.md` |
| `play_set_passengers_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-passengers-clientbound-framed-dispatch.md` |
| `play_set_time_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-time-clientbound-framed-dispatch.md` |
| `play_set_titles_animation_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-set-titles-animation-clientbound-framed-dispatch.md` |
| `play_sound_entity_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-sound-entity-clientbound-framed-dispatch.md` |
| `play_sound_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-sound-clientbound-framed-dispatch.md` |
| `play_start_configuration_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-start-configuration-clientbound-framed-dispatch.md` |
| `play_stop_sound_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-stop-sound-clientbound-framed-dispatch.md` |
| `play_store_cookie_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-store-cookie-clientbound-framed-dispatch.md` |
| `play_take_item_entity_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-take-item-entity-clientbound-framed-dispatch.md` |
| `play_ticking_state_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-ticking-state-clientbound-framed-dispatch.md` |
| `play_ticking_step_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-ticking-step-clientbound-framed-dispatch.md` |
| `play_transfer_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-transfer-clientbound-framed-dispatch.md` |
| `play_clear_dialog_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-clear-dialog-clientbound-framed-dispatch.md` |
| `play_update_tags_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-update-tags-clientbound-framed-dispatch.md` |
| `play_update_mob_effect_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-update-mob-effect-clientbound-framed-dispatch.md` |
| `play_show_dialog_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-show-dialog-clientbound-framed-dispatch.md` |
| `play_ping_clientbound_framed_dispatch` case note | `docs/analysis/protocol/versions/775/cases/play-ping-clientbound-framed-dispatch.md` |
| Oracle workbench router | `.codex/skills/stevenarella-oracle-workbench/SKILL.md` |
| Oracle case package workflow | `.codex/skills/stevenarella-oracle-case-builder/SKILL.md` |

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
`play_animate_clientbound_framed_dispatch`,
`play_award_stats_clientbound_framed_dispatch`,
`play_block_changed_ack_clientbound_framed_dispatch`,
`play_block_destruction_clientbound_framed_dispatch`,
`play_block_entity_data_clientbound_framed_dispatch`,
`play_block_event_clientbound_framed_dispatch`,
`play_block_update_clientbound_framed_dispatch`,
`play_boss_event_clientbound_framed_dispatch`,
`play_change_difficulty_clientbound_framed_dispatch`,
`play_chunk_batch_finished_clientbound_framed_dispatch`,
`play_chunk_batch_start_clientbound_framed_dispatch`,
`play_chunks_biomes_clientbound_framed_dispatch`,
`play_clear_titles_clientbound_framed_dispatch`,
`play_command_suggestions_clientbound_framed_dispatch`,
`play_commands_clientbound_framed_dispatch`,
`play_container_close_clientbound_framed_dispatch`, and
`play_container_set_content_clientbound_framed_dispatch`,
`play_container_set_data_clientbound_framed_dispatch`, and
`play_container_set_slot_clientbound_framed_dispatch`,
`play_cookie_request_clientbound_framed_dispatch`,
`play_cooldown_clientbound_framed_dispatch`,
`play_custom_chat_completions_clientbound_framed_dispatch`, and
`play_custom_payload_clientbound_framed_dispatch`,
`play_disconnect_clientbound_framed_dispatch`,
`play_entity_position_sync_clientbound_framed_dispatch`,
`play_forget_level_chunk_clientbound_framed_dispatch`,
`play_game_event_clientbound_framed_dispatch`, and
`play_mount_screen_open_clientbound_framed_dispatch`,
`play_hurt_animation_clientbound_framed_dispatch`,
`play_initialize_border_clientbound_framed_dispatch`, and
`play_keep_alive_clientbound_framed_dispatch` are the passing jar-backed
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

`play_container_set_content_clientbound_framed_dispatch` is packet-support
evidence for one official Play clientbound container_set_content empty-items
fixture only. It proves the official `minecraft:container_set_content` /
`0x12` row, body shape as containerId, stateId, ItemStack optional-list count,
and carried ItemStack optional marker, fixture container id `7`, state id
`123`, zero item entries, empty carried stack, framed bytes `12077b0000`, and
full body consumption through Stevenarella dispatch. It does not prove
non-empty ItemStack/component registry handling, menu lifecycle behavior,
screen close behavior, inventory state, runtime Play entry, world load, render
readiness, or client-load completion.

`play_container_set_data_clientbound_framed_dispatch` is packet-support
evidence for one official Play clientbound container_set_data numeric fixture
only. It proves the official `minecraft:container_set_data` / `0x13` row, body
shape as containerId, id, and value, fixture container id `7`, id `2`, value
`300`, framed bytes `13070002012c`, and full body consumption through
Stevenarella dispatch. It does not prove menu property semantics, inventory
state, runtime Play entry, world load, render readiness, or client-load
completion.

`play_container_set_slot_clientbound_framed_dispatch` is packet-support
evidence for one official Play clientbound container_set_slot empty ItemStack
fixture only. It proves the official `minecraft:container_set_slot` / `0x14`
row, body shape as containerId, stateId, slot, and ItemStack optional marker,
fixture container id `7`, state id `123`, slot `4`, empty item marker, framed
bytes `14077b000400`, and full body consumption through Stevenarella dispatch.
It does not prove non-empty ItemStack/component registry handling, menu
lifecycle behavior, inventory state, runtime Play entry, world load, render
readiness, or client-load completion.

`play_cookie_request_clientbound_framed_dispatch` is packet-support evidence
for one official Play clientbound cookie_request Identifier-key fixture only.
It proves the official `minecraft:cookie_request` / `0x15` row, body shape as
one `Identifier` key via `FriendlyByteBuf.writeIdentifier`, fixture key `a:a`,
framed bytes `1503613a61`, body bytes `03613a61`, and full body consumption
through Stevenarella dispatch. It does not prove cookie storage policy, cookie
request/response runtime behavior, initialized client/server state, runtime
Play entry, world load, spawn readiness, render readiness, or client-load
completion.

`play_cooldown_clientbound_framed_dispatch` is packet-support evidence for one
official Play clientbound cooldown Identifier/duration fixture only. It proves
the official `minecraft:cooldown` / `0x16` row, body shape as Identifier
cooldown group plus VarInt duration, fixture key `a:a`, duration `123`, framed
bytes `1603613a617b`, body bytes `03613a617b`, and full body consumption
through Stevenarella dispatch. It does not prove item cooldown semantics, item
registry contents, UI cooldown behavior, runtime Play entry, world load, spawn
readiness, render readiness, or client-load completion.

`play_custom_chat_completions_clientbound_framed_dispatch` is packet-support
evidence for one official Play clientbound custom_chat_completions ADD/string
fixture only. It proves the official `minecraft:custom_chat_completions` /
`0x17` row, body shape as enum action plus VarInt UTF-8 string list, fixture
action `ADD`, entry `alpha`, framed bytes `17000105616c706861`, body bytes
`000105616c706861`, and full body consumption through Stevenarella dispatch.
It does not prove chat UI behavior, command context behavior, completion
lifecycle semantics, runtime Play entry, world load, spawn readiness, render
readiness, or client-load completion.

`play_custom_payload_clientbound_framed_dispatch` is packet-support evidence
for one official Play clientbound custom_payload BrandPayload fixture only. It
proves the official `minecraft:custom_payload` / `0x18` row, payload id
`minecraft:brand`, brand `rustmine-play-oracle-brand`, framed bytes
`180f6d696e6563726166743a6272616e641a727573746d696e652d706c61792d6f7261636c652d6272616e64`,
and full body consumption through Stevenarella dispatch. It does not prove
arbitrary plugin-channel handling, payload routing policy, runtime Play entry,
world load, spawn readiness, render readiness, or client-load completion.
`play-clientbound-deferred.md` parks `minecraft:damage_event` / `0x19` and
the intervening YELLOW/RED rows that require registry, chat, debug, entity,
particle/sound/world, or game-rule evidence before fixture selection.

`play_disconnect_clientbound_framed_dispatch` is packet-support evidence for
one official Play clientbound disconnect empty literal Component reason fixture
only. It proves the official `minecraft:disconnect` / `0x20` row,
`ComponentSerialization.TRUSTED_CONTEXT_FREE_STREAM_CODEC` body, framed bytes
`20080000`, body bytes `080000`, and full body consumption through
Stevenarella dispatch. It does not prove UI disconnect handling, screen flow,
runtime Play entry, world load, spawn readiness, render readiness, or
client-load completion.

`play_entity_position_sync_clientbound_framed_dispatch` is packet-support
evidence for one official Play clientbound entity_position_sync primitive
fixture only. It proves the official `minecraft:entity_position_sync` /
`0x23` row, entity id `123`, position `(1.25, 64.5, -2.75)`, delta movement
`(0.125, 0.0, -0.25)`, rotations `45.0` and `-10.0`, `onGround=true`, and
full body consumption through Stevenarella dispatch. It does not prove
initialized `Entity`/`Level` behavior, entity existence, spawn readiness,
render readiness, or client-load completion.

`play_forget_level_chunk_clientbound_framed_dispatch` is packet-support
evidence for one official Play clientbound forget_level_chunk fixture only. It
proves the official `minecraft:forget_level_chunk` / `0x25` row, chunk
position `x=12, z=-7`, framed bytes `25fffffff90000000c`, body bytes
`fffffff90000000c`, and full body consumption through Stevenarella dispatch.
It does not prove chunk unload behavior, client world state, world load,
render readiness, or client-load completion.

`play_game_event_clientbound_framed_dispatch` is packet-support evidence for
one official Play clientbound game_event START_RAINING fixture only. It proves
the official `minecraft:game_event` / `0x26` row, unsigned-byte event id `1`,
float parameter `0.5`, framed bytes `26013f000000`, body bytes `013f000000`,
and full body consumption through Stevenarella dispatch. It does not prove game
event semantics, initialized `Level`/player/weather state, render readiness,
or client-load completion.

`play_mount_screen_open_clientbound_framed_dispatch` is packet-support evidence
for one official Play clientbound mount_screen_open primitive fixture only. It
proves the official `minecraft:mount_screen_open` / `0x29` row, container id
`7`, inventory columns `5`, entity id `123`, framed bytes `2907050000007b`,
body bytes `07050000007b`, and full body consumption through Stevenarella
dispatch. It does not prove mount entity existence, inventory/menu semantics,
screen behavior, render readiness, or client-load completion.

`play_hurt_animation_clientbound_framed_dispatch` is packet-support evidence
for one official Play clientbound hurt_animation primitive fixture only. It
proves the official `minecraft:hurt_animation` / `0x2a` row, entity id `123`,
yaw `45.5`, framed bytes `2a7b42360000`, body bytes `7b42360000`, and full
body consumption through Stevenarella dispatch. It does not prove entity
existence, hurt animation semantics, world state, render readiness, or
client-load completion.

`play_initialize_border_clientbound_framed_dispatch` is packet-support
evidence for one official Play clientbound initialize_border primitive-field
fixture only. It proves the official `minecraft:initialize_border` / `0x2b`
row, center/size/lerp/warning body, framed bytes
`2b4029000000000000c01d00000000000040590000000000004050200000000000b960f086a70e050f`,
body bytes
`4029000000000000c01d00000000000040590000000000004050200000000000b960f086a70e050f`,
and full body consumption through Stevenarella dispatch. It does not prove
world-border runtime behavior, world state, render readiness, or client-load
completion.

`play_keep_alive_clientbound_framed_dispatch` is packet-support evidence for
one official Play clientbound keep_alive primitive id fixture only. It proves
the official `minecraft:keep_alive` / `0x2c` row, id `12345`, framed bytes
`2c0000000000003039`, body bytes `0000000000003039`, and full body
consumption through Stevenarella dispatch. It does not prove runtime
keep-alive response behavior, Play entry, render readiness, or client-load
completion.

`play_level_event_clientbound_framed_dispatch` is packet-support evidence for
one official Play clientbound level_event primitive fixture only. It proves the
official `minecraft:level_event` / `0x2e` row, type `2001`, block position
`(1,64,-2)`, data `1`, globalEvent `false`, framed bytes
`2e000007d10000007fffffe0400000000100`, body bytes
`000007d10000007fffffe0400000000100`, and full body consumption through
Stevenarella dispatch. It does not prove level event semantics,
sound/particle behavior, initialized Level state, render readiness, or
client-load completion.

`play_low_disk_space_warning_clientbound_framed_dispatch` is packet-support
evidence for the official Play clientbound low_disk_space_warning singleton
empty-body fixture only. It proves the official
`minecraft:low_disk_space_warning` / `0x32` row, framed bytes `32`, empty body
bytes, and full body consumption through Stevenarella dispatch. It does not
prove disk warning UI behavior, client storage state, render readiness, or
client-load completion.

`play_move_entity_pos_clientbound_framed_dispatch`,
`play_move_entity_pos_rot_clientbound_framed_dispatch`, and
`play_move_entity_rot_clientbound_framed_dispatch` are packet-support evidence
for one official primitive entity move fixture each. They prove the official
`minecraft:move_entity_pos` / `0x35` frame `357b1000f800008001`,
`minecraft:move_entity_pos_rot` / `0x36` frame
`367c0010ffe0003040e000`, and `minecraft:move_entity_rot` / `0x38` frame
`387d20f001`, with full body consumption through Stevenarella dispatch. They
do not prove entity existence, movement/rotation interpolation semantics,
initialized Level state, render readiness, or client-load completion.

`play_move_vehicle_clientbound_framed_dispatch`,
`play_open_book_clientbound_framed_dispatch`, and
`play_ping_clientbound_framed_dispatch` are packet-support evidence for one
official primitive/context-free fixture each. They prove the official
`minecraft:move_vehicle` / `0x39` frame
`3940290000000000004050100000000000c01f00000000000043078000c1b20000`,
`minecraft:open_book` / `0x3a` frame `3a00`, and `minecraft:ping` / `0x3d`
frame `3d10203040`, with full body consumption through Stevenarella dispatch.
They do not prove vehicle existence, book UI behavior, runtime pong response
behavior, Play entry, render readiness, or client-load completion. The skipped
YELLOW rows `0x3b` / `0x3c` remain parked in
`play-clientbound-deferred.md`; the next official Play clientbound row after
this safe batch is `minecraft:pong_response` / `0x3e`.

`play_pong_response_clientbound_framed_dispatch`,
`play_player_abilities_clientbound_framed_dispatch`,
`play_player_combat_end_clientbound_framed_dispatch`,
`play_player_combat_enter_clientbound_framed_dispatch`, and
`play_remove_entities_clientbound_framed_dispatch` are packet-support evidence
for one official primitive/context-free fixture each. They prove the official
`minecraft:pong_response` / `0x3e` frame `3e0102030405060708`,
`minecraft:player_abilities` / `0x40` frame `40053d4ccccd3dcccccd`,
`minecraft:player_combat_end` / `0x42` frame `427b`,
`minecraft:player_combat_enter` / `0x43` frame `43`, and
`minecraft:remove_entities` / `0x4d` frame `4d027bd723`, with full body
consumption through Stevenarella dispatch. They do not prove runtime
ping/pong behavior, initialized player ability semantics, combat runtime
behavior, entity existence, Play entry, render readiness, or client-load
completion. The skipped YELLOW rows `0x3f`, `0x41`, `0x44`, `0x46`-`0x4c`,
and `0x4e`-`0x51` remain parked in `play-clientbound-deferred.md`; the next
official Play clientbound row after this safe batch is
`minecraft:respawn` / `0x52`.

`play_player_info_remove_clientbound_framed_dispatch`,
`play_rotate_head_clientbound_framed_dispatch`,
`play_select_advancements_tab_clientbound_framed_dispatch`,
`play_set_chunk_cache_center_clientbound_framed_dispatch`, and
`play_set_chunk_cache_radius_clientbound_framed_dispatch` are packet-support
evidence for one official UUID-list or primitive/context-free fixture each.
They prove the official `minecraft:player_info_remove` / `0x45` frame
`4502123e4567e89b12d3a45642661417404500000000000000000000000000000045`,
`minecraft:rotate_head` / `0x53` frame `537b40`,
`minecraft:select_advancements_tab` / `0x55` frame
`5501146d696e6563726166743a73746f72792f726f6f74`,
`minecraft:set_chunk_cache_center` / `0x5e` frame `5e07fdffffff0f`, and
`minecraft:set_chunk_cache_radius` / `0x5f` frame `5f0c`, with full body
consumption through Stevenarella dispatch. They do not prove GameProfile or
player-list state, entity existence, advancement UI behavior, chunk loading,
world hydration, render readiness, or client-load completion. The skipped
YELLOW rows through `0x64` remain parked in `play-clientbound-deferred.md`; the
next official Play clientbound row after this safe batch is
`minecraft:set_cursor_item` / `0x60`.

`play_set_border_center_clientbound_framed_dispatch`,
`play_set_border_lerp_size_clientbound_framed_dispatch`,
`play_set_border_size_clientbound_framed_dispatch`,
`play_set_border_warning_delay_clientbound_framed_dispatch`, and
`play_set_border_warning_distance_clientbound_framed_dispatch` are
packet-support evidence for the official primitive/context-free border rows
`minecraft:set_border_center` / `0x58` through
`minecraft:set_border_warning_distance` / `0x5c`. They prove frames
`584029000000000000c041600000000000`, `594059000000000000406f500000000000b960`,
`5a4080020000000000`, `5b2a`, and `5c07`, with full body consumption through
Stevenarella dispatch. They do not prove world-border runtime behavior,
warning UI behavior, world state, render readiness, or client-load completion.
`minecraft:set_entity_motion` / `0x65` has now been promoted to a jar-backed
packet-support proof package with a current Stevenarella mismatch; the skipped
YELLOW rows remain parked in `play-clientbound-deferred.md`. The
next official Play clientbound row after the latest proven contiguous area is
still `minecraft:set_cursor_item` / `0x60`.

`play_set_entity_motion_clientbound_framed_dispatch`,
`play_set_experience_clientbound_framed_dispatch`,
`play_set_health_clientbound_framed_dispatch`,
`play_set_held_slot_clientbound_framed_dispatch`, and
`play_set_simulation_distance_clientbound_framed_dispatch` have official
jar-backed answers for `minecraft:set_entity_motion` / `0x65`,
`minecraft:set_experience` / `0x67`, `minecraft:set_health` / `0x68`,
`minecraft:set_held_slot` / `0x69`, and
`minecraft:set_simulation_distance` / `0x6f`. The Java harness generated
frames `65b960f23f87febfff`, `673f2000002a944d`,
`68419400001140880000`, `6906`, and `6f0a` from `client.jar`. The exact Rust
oracle tests now pass after the Protocol 775 Play clientbound dispatch mappings
were added. These are packet-support proofs only and do not prove
entity existence, movement semantics, player state, inventory UI, world ticking,
render readiness, or client-load completion.

`play_set_titles_animation_clientbound_framed_dispatch`,
`play_start_configuration_clientbound_framed_dispatch`,
`play_stop_sound_clientbound_framed_dispatch`,
`play_store_cookie_clientbound_framed_dispatch`,
`play_take_item_entity_clientbound_framed_dispatch`,
`play_ticking_state_clientbound_framed_dispatch`,
`play_ticking_step_clientbound_framed_dispatch`, and
`play_transfer_clientbound_framed_dispatch` have official jar-backed answers
for `minecraft:set_titles_animation` / `0x73`,
`minecraft:start_configuration` / `0x76`, `minecraft:stop_sound` / `0x77`,
`minecraft:store_cookie` / `0x78`, `minecraft:take_item_entity` / `0x7c`,
`minecraft:ticking_state` / `0x7f`, `minecraft:ticking_step` / `0x80`, and
`minecraft:transfer` / `0x81`. Their exact Rust oracle tests pass against the
current Leafish checkout. These proofs do not cover title UI behavior,
Play-to-Configuration transition handling, named sound/source variants,
runtime cookie storage, entity existence, item stack contents, collection
runtime behavior, tick-manager runtime behavior, transfer handling, Play
readiness, or client-load completion.

`play_custom_report_details_clientbound_framed_dispatch`,
`play_server_links_clientbound_framed_dispatch`,
`play_clear_dialog_clientbound_framed_dispatch`,
`play_sound_clientbound_framed_dispatch`,
`play_update_mob_effect_clientbound_framed_dispatch`, and
`play_show_dialog_clientbound_framed_dispatch` have official jar-backed
answers for `minecraft:custom_report_details` / `0x88`,
`minecraft:server_links` / `0x89`, `minecraft:clear_dialog` / `0x8b`,
`minecraft:sound` / `0x75`, `minecraft:update_mob_effect` / `0x84`, and
`minecraft:show_dialog` / `0x8c`.
Their exact Rust oracle tests pass against the current Leafish checkout. These
proofs are scoped to an empty report-details map, an empty server-links list,
the clear-dialog singleton body, one SoundEvents.AMBIENT_CAVE world-position
sound fixture, one MobEffects.SPEED update fixture, and one direct
NoticeDialog show-dialog fixture; they do not prove non-empty entry semantics,
link UI behavior, arbitrary sound/effect/dialog holders, world sound playback,
entity effect runtime state, dialog UI behavior, Play readiness, or
client-load completion.

`play_set_entity_link_clientbound_framed_dispatch`,
`play_set_passengers_clientbound_framed_dispatch`, and
`play_sound_entity_clientbound_framed_dispatch` have official GameTest-backed
answers for `minecraft:set_entity_link` / `0x64`,
`minecraft:set_passengers` / `0x6b`, and `minecraft:sound_entity` / `0x74`.
Their exact Rust oracle tests pass against the current Leafish checkout. These
proofs are scoped to the initialized GameTest fixture: source pig id `1`,
destination armor stand id `2`, minecart vehicle id `3`, passenger pig id `4`,
and one `SoundEvents.AMBIENT_CAVE` entity sound. Other entity ids, passenger
topologies, sound holders, sources, entity ids, volume, pitch, and seed values
are rejected before broader entity/world/sound semantics, Play readiness, or
client-load completion.

`play_set_display_objective_clientbound_framed_dispatch`,
`play_set_score_clientbound_framed_dispatch`,
`play_set_time_clientbound_framed_dispatch`, and
`play_update_tags_clientbound_framed_dispatch` have official jar-backed
answers for `minecraft:set_display_objective` / `0x62`,
`minecraft:set_score` / `0x6e`, `minecraft:set_time` / `0x71`, and
`minecraft:update_tags` / `0x86`. Their exact Rust oracle tests pass against
the current Leafish checkout. These proofs are scoped to clear display
objective with null Objective, set_score with absent optional display and
number format, set_time with an empty clock update map, and update_tags with
an empty registry tag map; they do not prove scoreboard lifecycle, optional
Component payloads, number-format payloads, WorldClock or ClockNetworkState
entries, registry tag contents, Play readiness, or client-load completion.

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
