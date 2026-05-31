# Client Load

Purpose: keep "the client loads" as an evidence lens with named proof, not as a
single protocol packet claim or a mandatory route for unrelated work.

## Spatial Map

```text
local_boot_resources
  -> network_login_configuration
    -> registry_hydration
      -> play_entry
        -> world_hydration
          -> entity_player_hydration
            -> render_ready
              -> control_interact_ready
```

Protocol 775 belongs mainly to `network_login_configuration`,
`registry_hydration`, and `play_entry`. It is not the whole loading problem.
This map is the default diagnostic lens for client-load/playability work.
Targeted later-phase, cross-phase, protocol-only, docs-only, review-only, or
tooling tasks may enter at the owning shard named by the task/evidence.

## Phase Index

| Phase | Detail | Proof label | Evidence snapshot | Candidate evidence gap |
|---|---|---|---|---|
| `local_boot_resources` | [phases/local_boot_resources.md](phases/local_boot_resources.md) | `unproven` | none | Define resource-ready proof outside reset-prone tests |
| `network_login_configuration` | [phases/network_login_configuration.md](phases/network_login_configuration.md) | `partial` | `handshake_intention_framed_dispatch`, `login_hello_serverbound_framed_dispatch`, `login_key_serverbound_framed_dispatch`, `login_custom_query_answer_serverbound_framed_dispatch`, `login_acknowledged_serverbound_framed_dispatch`, `login_cookie_response_serverbound_framed_dispatch`, `login_disconnect_clientbound_framed_dispatch`, `login_hello_clientbound_framed_dispatch`, `login_finished_clientbound_framed_dispatch`, `login_compression_clientbound_framed_dispatch`, `login_custom_query_clientbound_framed_dispatch`, `login_cookie_request_clientbound_framed_dispatch`, `configuration_client_information_framed_dispatch`, `configuration_cookie_request_framed_dispatch`, `configuration_cookie_response_framed_dispatch`, `configuration_custom_payload_clientbound_framed_dispatch`, `configuration_custom_payload_framed_dispatch`, `configuration_disconnect_clientbound_framed_dispatch`, `configuration_reset_chat_clientbound_framed_dispatch`, `configuration_registry_data_clientbound_framed_dispatch`, `configuration_resource_pack_pop_clientbound_framed_dispatch`, `configuration_resource_pack_push_clientbound_framed_dispatch`, `configuration_store_cookie_clientbound_framed_dispatch`, `configuration_transfer_clientbound_framed_dispatch`, `configuration_update_enabled_features_clientbound_framed_dispatch`, `configuration_update_tags_clientbound_framed_dispatch`, `configuration_select_known_packs_clientbound_framed_dispatch`, `configuration_custom_report_details_clientbound_framed_dispatch`, `configuration_server_links_clientbound_framed_dispatch`, `configuration_clear_dialog_clientbound_framed_dispatch`, `configuration_show_dialog_clientbound_framed_dispatch`, `configuration_code_of_conduct_clientbound_framed_dispatch`, `configuration_keepalive_codec`, `configuration_keepalive_framed_dispatch`, `configuration_keepalive_clientbound_framed_dispatch`, `configuration_ping_pong_framed_dispatch`, `configuration_finish_framed_terminal`, `configuration_resource_pack_response_framed_dispatch`, `configuration_select_known_packs_framed_dispatch`, `configuration_custom_click_action_framed_dispatch`, and `configuration_accept_code_of_conduct_framed_dispatch` jar-backed answers regenerated and exact Rust oracle tests passed against the current Leafish checkout; `configuration_keepalive_runtime_send_helper`, `configuration_keepalive_runtime_protocol_echo`, and `configuration_keepalive_runtime_spawn_reader_reaction` exact runtime probes passed against the current Leafish checkout | Protocol 775 Handshaking serverbound packet-support table has its official `minecraft:intention` / `0x00` row; Login serverbound packet-support is complete through current official rows: `minecraft:hello` / `0x00`, `minecraft:key` / `0x01`, `minecraft:custom_query_answer` / `0x02`, `minecraft:login_acknowledged` / `0x03`, and `minecraft:cookie_response` / `0x04`; Login clientbound packet-support is complete through current official rows: `minecraft:login_disconnect` / `0x00`, `minecraft:hello` / `0x01`, `minecraft:login_finished` / `0x02`, `minecraft:login_compression` / `0x03`, `minecraft:custom_query` / `0x04`, and `minecraft:cookie_request` / `0x05`; Configuration packet-support tables are complete through current official clientbound/serverbound rows; next packet-support table target is an official Play table audit from `GameProtocols.CLIENTBOUND_TEMPLATE` / `SERVERBOUND_TEMPLATE` before choosing a Play packet case; runtime Configuration-to-Play, registry hydration, and Play readiness remain later gaps |
| `registry_hydration` | [phases/registry_hydration.md](phases/registry_hydration.md) | `unproven` | none | Define heavy harness proof; do not fake initialized state |
| `play_entry` | [phases/play_entry.md](phases/play_entry.md) | `partial` | `play_bundle_delimiter_clientbound_framed_dispatch` proves official Play clientbound `minecraft:bundle_delimiter` / `0x00` empty-body packet-support only; `play_add_entity_clientbound_framed_dispatch` proves official Play clientbound `minecraft:add_entity` / `0x01` packet-support for one built-in EntityType.PIG zero-movement fixture only; `play_animate_clientbound_framed_dispatch` proves official Play clientbound `minecraft:animate` / `0x02` packet-support for one entity id 123 and SWING_MAIN_HAND fixture only; `play_award_stats_clientbound_framed_dispatch` proves official Play clientbound `minecraft:award_stats` / `0x03` packet-support for one empty-stats fixture only; `play_block_changed_ack_clientbound_framed_dispatch` proves official Play clientbound `minecraft:block_changed_ack` / `0x04` packet-support for one sequence fixture only; `play_block_destruction_clientbound_framed_dispatch` proves official Play clientbound `minecraft:block_destruction` / `0x05` packet-support for one breaker id, block position, and progress fixture only; `play_block_entity_data_clientbound_framed_dispatch` proves official Play clientbound `minecraft:block_entity_data` / `0x06` packet-support for one block position, built-in chest block entity type, and empty tag fixture only; `play_block_event_clientbound_framed_dispatch` proves official Play clientbound `minecraft:block_event` / `0x07` packet-support for one block position, built-in note block, event type, and event data fixture only; `play_block_update_clientbound_framed_dispatch` proves official Play clientbound `minecraft:block_update` / `0x08` packet-support for one block position and built-in stone default block-state fixture only; `play_boss_event_clientbound_framed_dispatch` proves official Play clientbound `minecraft:boss_event` / `0x09` packet-support for one UUID remove-operation fixture only; `play_change_difficulty_clientbound_framed_dispatch` proves official Play clientbound `minecraft:change_difficulty` / `0x0a` packet-support for one difficulty/locked fixture only; `play_chunk_batch_finished_clientbound_framed_dispatch` proves official Play clientbound `minecraft:chunk_batch_finished` / `0x0b` packet-support for one batchSize fixture only; `play_chunk_batch_start_clientbound_framed_dispatch` proves official Play clientbound `minecraft:chunk_batch_start` / `0x0c` packet-support for the singleton empty-body fixture only; `play_chunks_biomes_clientbound_framed_dispatch` proves official Play clientbound `minecraft:chunks_biomes` / `0x0d` packet-support for the empty chunkBiomeData-list fixture only; `play_clear_titles_clientbound_framed_dispatch` proves official Play clientbound `minecraft:clear_titles` / `0x0e` packet-support for one resetTimes=true boolean fixture only; `play_command_suggestions_clientbound_framed_dispatch` proves official Play clientbound `minecraft:command_suggestions` / `0x0f` packet-support for one command id/range/empty-suggestions fixture only; `play_commands_clientbound_framed_dispatch` proves official Play clientbound `minecraft:commands` / `0x10` packet-support for one empty root-only command tree fixture only | Continue official Play table packet-support in order at `minecraft:container_close` / `0x11`, then define transition answer and smoke milestone |
| `world_hydration` | [phases/world_hydration.md](phases/world_hydration.md) | `unproven` | none | Define chunk/world oracle fixture strategy |
| `entity_player_hydration` | [phases/entity_player_hydration.md](phases/entity_player_hydration.md) | `unproven` | none | Define spawn/entity proof package |
| `render_ready` | [phases/render_ready.md](phases/render_ready.md) | `unproven` | none | Define screenshot/pixel or milestone proof |
| `control_interact_ready` | [phases/control_interact_ready.md](phases/control_interact_ready.md) | `unproven` | none | Define corridor probe proof |

## Evidence Route

For client-load/playability diagnosis, read:

```text
docs/analysis/current-evidence/README.md
docs/analysis/current-evidence/client-load.md
docs/analysis/current-evidence/structural-scan.md
```

Use the first missing or failing proof as the default diagnostic entry point.
For targeted later-phase or cross-phase work, choose the named phase(s) and
record the evidence reason in the owning shard. Path existence in
`stevenarella/` is `observed_only`; it cannot upgrade a phase from `unproven`
or `partial` to `verified`.

## Update Rule

Mutable phase facts live in this shard:

```text
docs/analysis/client-load/README.md
docs/analysis/client-load/phases/*.md
docs/analysis/current-evidence/*.md
```
