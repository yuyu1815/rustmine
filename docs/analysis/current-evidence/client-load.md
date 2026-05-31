# Client Load Evidence

Purpose: provide a current evidence lens that separates proven compatibility
from structural observation for client-load and playability claims.

## Default Diagnostic Route

```text
1. Read docs/analysis/current-evidence/structural-scan.md
2. Treat path existence as observed_only
3. For client-load/playability diagnosis, start at the first missing or failing proof in the phase lens
4. For targeted later-phase, cross-phase, protocol-only, docs-only, review-only, or tooling work, use the owning shard named by the task/evidence
5. Add or update proof only through root-owned artifacts/tests/probes
6. Update docs/analysis/client-load/README.md and the phase detail file when phase evidence changed
```

## Evidence Snapshot

| Load phase | Proof state | Evidence | What this proves | What it does not prove |
|---|---|---|---|---|
| `local_boot_resources` | `unproven` | none | nothing yet | startup, resources, assets, render setup |
| `network_login_configuration` | `partial` | `handshake_intention_framed_dispatch`; `login_hello_serverbound_framed_dispatch`; `login_key_serverbound_framed_dispatch`; `login_custom_query_answer_serverbound_framed_dispatch`; `login_acknowledged_serverbound_framed_dispatch`; `login_cookie_response_serverbound_framed_dispatch`; `login_disconnect_clientbound_framed_dispatch`; `login_hello_clientbound_framed_dispatch`; `login_finished_clientbound_framed_dispatch`; `login_compression_clientbound_framed_dispatch`; `login_custom_query_clientbound_framed_dispatch`; `login_cookie_request_clientbound_framed_dispatch`; `configuration_client_information_framed_dispatch`; `configuration_cookie_request_framed_dispatch`; `configuration_cookie_response_framed_dispatch`; `configuration_custom_payload_clientbound_framed_dispatch`; `configuration_custom_payload_framed_dispatch`; `configuration_disconnect_clientbound_framed_dispatch`; `configuration_reset_chat_clientbound_framed_dispatch`; `configuration_registry_data_clientbound_framed_dispatch`; `configuration_resource_pack_pop_clientbound_framed_dispatch`; `configuration_resource_pack_push_clientbound_framed_dispatch`; `configuration_store_cookie_clientbound_framed_dispatch`; `configuration_transfer_clientbound_framed_dispatch`; `configuration_update_enabled_features_clientbound_framed_dispatch`; `configuration_update_tags_clientbound_framed_dispatch`; `configuration_select_known_packs_clientbound_framed_dispatch`; `configuration_custom_report_details_clientbound_framed_dispatch`; `configuration_server_links_clientbound_framed_dispatch`; `configuration_clear_dialog_clientbound_framed_dispatch`; `configuration_show_dialog_clientbound_framed_dispatch`; `configuration_code_of_conduct_clientbound_framed_dispatch`; `configuration_keepalive_codec`; `configuration_keepalive_framed_dispatch`; `configuration_keepalive_clientbound_framed_dispatch`; `configuration_ping_pong_framed_dispatch`; `configuration_finish_framed_terminal`; `configuration_resource_pack_response_framed_dispatch`; `configuration_select_known_packs_framed_dispatch`; `configuration_custom_click_action_framed_dispatch`; `configuration_accept_code_of_conduct_framed_dispatch`; `configuration_keepalive_runtime_send_helper`; `configuration_keepalive_runtime_protocol_echo`; `configuration_keepalive_runtime_spawn_reader_reaction`; `oracle/rust-tests/tests/oracle_contracts.rs`; `bash oracle/scripts/run_jar_backed_oracle_tests.sh` passed on 2026-05-31 for direct jar-backed cases | Handshaking serverbound intention LOGIN fixture framed dispatch/decode; Login serverbound hello name/profileId framed dispatch/decode with body consumption; Login serverbound key keybytes/encryptedChallenge framed dispatch/decode with body consumption; Login serverbound custom_query_answer transaction-id/null-payload-marker framed dispatch/decode with body consumption; Login serverbound login_acknowledged singleton empty-body terminal framed dispatch/decode with body consumption; Login serverbound cookie_response Identifier-key/non-null-payload framed dispatch/decode with body consumption; Login clientbound login_disconnect empty literal Component reason framed dispatch/decode with body consumption; Login clientbound hello serverId/publicKey/challenge/shouldAuthenticate framed dispatch/decode with body consumption; Login clientbound login_finished GameProfile UUID/name/empty-properties terminal framed dispatch/decode with body consumption; Login clientbound login_compression VarInt compressionThreshold framed dispatch/decode with body consumption; Login clientbound custom_query transaction-id/payload-Identifier/empty-payload framed dispatch/decode with body consumption; Login clientbound cookie_request Identifier-key framed dispatch/decode with body consumption; Configuration serverbound client_information framed dispatch/decode, clientbound cookie_request Identifier-key framed dispatch/decode for one key fixture, serverbound cookie_response key/nullable-payload framed dispatch/decode for one non-null payload fixture, clientbound custom_payload BrandPayload framed dispatch/decode for one official BrandPayload fixture, serverbound custom_payload BrandPayload framed dispatch/decode for one official BrandPayload fixture, clientbound disconnect empty literal Component reason framed dispatch/decode, clientbound reset_chat singleton empty-body framed dispatch/decode, clientbound registry_data DIMENSION_TYPE empty-entry framed dispatch/decode with body consumption, clientbound resource_pack_pop present-UUID framed dispatch/decode with body consumption, clientbound resource_pack_push no-prompt framed dispatch/decode with body consumption, clientbound store_cookie Identifier-key/payload framed dispatch/decode with body consumption, clientbound transfer host/port framed dispatch/decode with body consumption, clientbound update_enabled_features empty feature-set framed dispatch/decode with body consumption, clientbound update_tags empty tag-payload map framed dispatch/decode with body consumption, clientbound select_known_packs empty known-pack list framed dispatch/decode with body consumption, clientbound custom_report_details empty details-map framed dispatch/decode with body consumption, clientbound server_links empty links-list framed dispatch/decode with body consumption, clientbound clear_dialog singleton empty-body framed dispatch/decode with body consumption, clientbound show_dialog direct NoticeDialog context-free fixture framed dispatch/decode with body consumption, clientbound code_of_conduct String fixture framed dispatch/decode with body consumption, serverbound keep-alive packet id/body, serverbound/clientbound keep-alive framed dispatch/decode, clientbound ping/serverbound pong framed dispatch/decode, finish_configuration framed dispatch/decode plus official terminal flags, serverbound resource_pack response frame dispatch/decode with UUID/action body consumption, serverbound select_known_packs known-pack list dispatch/decode with full body consumption, serverbound custom_click_action identifier/optional-payload dispatch/decode with full body consumption, serverbound accept_code_of_conduct empty-body dispatch/decode with full body consumption, outgoing helper send of the official Configuration serverbound keep_alive frame, protocol-crate socket echo from official Configuration clientbound keep_alive to official Configuration serverbound keep_alive, and the factored `Server::spawn_reader` keep_alive branch response all match reset-proof evidence against the current Leafish checkout. | Login authentication success, Login encryption success/private-key validation, compression negotiation policy, connection compression activation, Login custom-query payload semantics, Login state-transition handling, arbitrary plugin-channel handling, payload routing policy, UI disconnect handling, screen flow, chat UI reset behavior, UI consent flow, legal acceptance semantics, report UI behavior, moderation/reporting semantics, server-links UI behavior, trust/link-opening policy, dialog UI clearing behavior, dialog UI display behavior, registry-backed dialogs, cookie storage policy, cookie persistence, cookie request/response runtime behavior, server transfer UX, reconnect behavior, network reconnection, runtime custom-click UI behavior, command execution, interaction readiness, real registry contents, RegistrySynchronization.packRegistries output, feature registry hydration, enabled-feature semantics, tag registry hydration, registry hydration, runtime known-pack negotiation completion, resource-pack UI behavior, pack removal policy, pack download/reload/application behavior, runtime resource pack UI/accept/reject behavior, runtime client settings send behavior, runtime ping response behavior, full login/configuration runtime behavior, runtime Configuration-to-Play transition, play transition |
| `registry_hydration` | `unproven` | none | nothing yet | registry/dimension/known-pack/feature state |
| `play_entry` | `partial` | `play_bundle_delimiter_clientbound_framed_dispatch`; `play_add_entity_clientbound_framed_dispatch`; `play_animate_clientbound_framed_dispatch`; `play_award_stats_clientbound_framed_dispatch`; `play_block_changed_ack_clientbound_framed_dispatch`; `play_block_destruction_clientbound_framed_dispatch` | Play clientbound bundle_delimiter framed dispatch/decode for the official registered empty-body singleton; Play clientbound add_entity framed dispatch/decode for one official built-in EntityType.PIG zero-movement fixture; Play clientbound animate framed dispatch/decode for one official STREAM_CODEC decode fixture with entity id 123 and SWING_MAIN_HAND; Play clientbound award_stats framed dispatch/decode for one official empty-stats fixture; Play clientbound block_changed_ack framed dispatch/decode for one official sequence fixture; Play clientbound block_destruction framed dispatch/decode for one official breaker id, block position, and progress fixture | successful entry into Play, bundle grouping behavior, arbitrary entity registry contents, initialized Entity/ServerEntity or Entity/Level behavior, entity existence, animation semantics, non-empty Stat registry entry decoding, stat semantics, UI/stat screen behavior, block prediction semantics, client world correction behavior, block break animation semantics, entity existence for breaker id, client world state, spawn readiness, world load, render readiness |
| `world_hydration` | `unproven` | none | nothing yet | chunks, light, block states, biomes, world time |
| `entity_player_hydration` | `unproven` | none | nothing yet | local player, remote players, entities, spawn readiness |
| `render_ready` | `unproven` | none | nothing yet | visible loaded world, screenshot/pixel readiness |
| `control_interact_ready` | `unproven` | none | nothing yet | movement, interact, inventory, combat after load |

## Snapshot Note

At this snapshot, the proven compatibility is only
`handshake_intention_framed_dispatch`,
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
`configuration_registry_data_clientbound_framed_dispatch`,
`configuration_resource_pack_pop_clientbound_framed_dispatch`,
`configuration_resource_pack_push_clientbound_framed_dispatch`,
`configuration_store_cookie_clientbound_framed_dispatch`,
`configuration_transfer_clientbound_framed_dispatch`,
`configuration_update_enabled_features_clientbound_framed_dispatch`,
`configuration_update_tags_clientbound_framed_dispatch`,
`configuration_select_known_packs_clientbound_framed_dispatch`,
`configuration_custom_report_details_clientbound_framed_dispatch`,
`configuration_server_links_clientbound_framed_dispatch`,
`configuration_clear_dialog_clientbound_framed_dispatch`,
`configuration_show_dialog_clientbound_framed_dispatch`,
`configuration_code_of_conduct_clientbound_framed_dispatch`,
`configuration_keepalive_codec`, `configuration_keepalive_framed_dispatch`,
`configuration_keepalive_clientbound_framed_dispatch`,
`configuration_ping_pong_framed_dispatch`,
`configuration_finish_framed_terminal`,
`configuration_resource_pack_response_framed_dispatch`,
`configuration_select_known_packs_framed_dispatch`,
`configuration_custom_click_action_framed_dispatch`,
`configuration_accept_code_of_conduct_framed_dispatch`,
`play_bundle_delimiter_clientbound_framed_dispatch`,
`play_add_entity_clientbound_framed_dispatch`, and
`play_animate_clientbound_framed_dispatch`, and
`play_award_stats_clientbound_framed_dispatch`, and
`play_block_changed_ack_clientbound_framed_dispatch`, and
`play_block_destruction_clientbound_framed_dispatch`, jar-backed,
regenerated in the current run, and checked by exact reset-proof Rust oracle
tests against the current Leafish checkout. `configuration_keepalive_runtime_send_helper`,
`configuration_keepalive_runtime_protocol_echo`, and
`configuration_keepalive_runtime_spawn_reader_reaction` are root-owned runtime
socket probes that now pass against the current Leafish checkout; they prove the
outgoing helper frame, the protocol-crate read/map/send echo path, and the same
factored keep_alive branch used by `Server::spawn_reader`, not Configuration
completion or Play entry.

`handshake_intention_framed_dispatch` is packet-support evidence for one
official LOGIN-intent fixture only. It does not prove Login authentication,
Configuration entry, or client-load completion.

`login_hello_serverbound_framed_dispatch` is packet-support evidence for one
official Login hello fixture only. It does not prove authentication success,
encryption/key exchange, login acknowledgement, Configuration entry, or
client-load completion.

`login_key_serverbound_framed_dispatch` is packet-support evidence for one
official minimal Login key fixture only. It does not prove encryption success,
private-key validation, authentication success, login acknowledgement,
Configuration entry, or client-load completion.

`login_custom_query_answer_serverbound_framed_dispatch` is packet-support
evidence for one official null-payload Login custom_query_answer fixture only.
It does not prove plugin channel handling, custom payload semantics,
Configuration entry, or client-load completion.

`login_acknowledged_serverbound_framed_dispatch` is packet-support evidence
for the official singleton Login login_acknowledged fixture only. It does not
prove Configuration entry, state transition handling, Play readiness, or
client-load completion.

`login_cookie_response_serverbound_framed_dispatch` is packet-support evidence
for one official Login cookie_response non-null payload fixture only. It does
not prove cookie storage policy, cookie request/response runtime behavior,
Configuration entry, Play readiness, or client-load completion.

`login_disconnect_clientbound_framed_dispatch` is packet-support evidence for
one official Login clientbound login_disconnect empty literal Component reason
fixture only. It does not prove UI disconnect handling, screen flow,
authentication failure handling, Configuration entry, Play readiness, or
client-load completion. The generated 26.1.2 Login clientbound table order is
`minecraft:login_disconnect` / `0x00`, `minecraft:hello` / `0x01`,
`minecraft:login_finished` / `0x02`, `minecraft:login_compression` / `0x03`,
`minecraft:custom_query` / `0x04`, and `minecraft:cookie_request` / `0x05`.

`login_hello_clientbound_framed_dispatch` is packet-support evidence for one
official Login clientbound hello fixture with empty `serverId`, empty
`publicKey`, empty `challenge`, and `shouldAuthenticate=false` only. It does
not prove encryption success, authentication success, key validation, login
state transition handling, Configuration entry, Play readiness, or client-load
completion.

`login_finished_clientbound_framed_dispatch` is packet-support evidence for
one official Login clientbound login_finished fixture with a zero UUID, empty
profile name, and empty profile properties only. It does not prove
authentication success, Login-to-Configuration state transition handling,
profile property semantics, skin/session trust, Configuration entry, Play
readiness, or client-load completion.

`login_compression_clientbound_framed_dispatch` is packet-support evidence for
one official Login clientbound login_compression fixture with
`compressionThreshold=0` only. It does not prove compression negotiation
policy, connection compression activation, Login-to-Configuration state
transition handling, Configuration entry, Play readiness, or client-load
completion.

`login_custom_query_clientbound_framed_dispatch` is packet-support evidence for
one official Login clientbound custom_query fixture with `transactionId=0`,
`payloadId=a:a`, and an empty `DiscardedQueryPayload` only. It does not prove
plugin channel handling, custom query semantics, login acknowledgement
behavior, Login-to-Configuration state transition handling, Configuration
entry, Play readiness, or client-load completion.

`login_cookie_request_clientbound_framed_dispatch` is packet-support evidence
for one official Login clientbound cookie_request fixture with key `a:a` only.
It does not prove cookie storage policy, cookie request/response runtime
behavior, Login-to-Configuration state transition handling, Configuration
entry, Play readiness, or client-load completion. Login clientbound
packet-support is complete through the current official rows:
`minecraft:login_disconnect` / `0x00`, `minecraft:hello` / `0x01`,
`minecraft:login_finished` / `0x02`, `minecraft:login_compression` / `0x03`,
`minecraft:custom_query` / `0x04`, and `minecraft:cookie_request` / `0x05`.

`play_bundle_delimiter_clientbound_framed_dispatch` is packet-support evidence
for the official Play clientbound bundle_delimiter registered singleton only.
It proves the official `minecraft:bundle_delimiter` / `0x00` row, empty body,
and full body consumption through Stevenarella dispatch. It does not prove
bundle grouping behavior, runtime Configuration-to-Play transition, world
load, spawn readiness, render readiness, or client-load completion.

`play_add_entity_clientbound_framed_dispatch` is packet-support evidence for
one official Play clientbound add_entity constructor fixture only. It proves
the official `minecraft:add_entity` / `0x01` row, body order, built-in
`minecraft:pig` entity type registry id, zero `Vec3.LP_STREAM_CODEC` movement
fixture, and full body consumption through Stevenarella dispatch. It does not
prove arbitrary entity registry contents, initialized `Entity`/`ServerEntity`
behavior, spawn readiness, world load, render readiness, or client-load
completion.

`play_animate_clientbound_framed_dispatch` is packet-support evidence for one
official Play clientbound animate `STREAM_CODEC` decode fixture only. It
proves the official `minecraft:animate` / `0x02` row, body order, entity id
VarInt, `SWING_MAIN_HAND` unsigned-byte action constant `0`, and full body
consumption through Stevenarella dispatch. It does not prove entity existence,
animation semantics, initialized `Entity`/`Level` behavior, spawn readiness,
world load, render readiness, or client-load completion.

`play_award_stats_clientbound_framed_dispatch` is packet-support evidence for
one official Play clientbound award_stats empty-stats fixture only. It proves
the official `minecraft:award_stats` / `0x03` row, body shape for an
`Object2IntMap<Stat<?>>` as VarInt count followed by Stat key and VarInt value
per entry, empty fixture body count `0`, and full body consumption through
Stevenarella dispatch. It does not prove non-empty Stat registry entry
decoding, stat semantics, UI/stat screen behavior, spawn readiness, world load,
render readiness, or client-load completion.

`play_block_changed_ack_clientbound_framed_dispatch` is packet-support evidence
for one official Play clientbound block_changed_ack sequence fixture only. It
proves the official `minecraft:block_changed_ack` / `0x04` row, body shape as
one sequence VarInt, fixture sequence `12345`, and full body consumption
through Stevenarella dispatch. It does not prove block prediction semantics,
client world correction behavior, initialized game state, spawn readiness,
world load, render readiness, or client-load completion.

`play_block_destruction_clientbound_framed_dispatch` is packet-support evidence
for one official Play clientbound block_destruction breaker id, block position,
and progress fixture only. It proves the official
`minecraft:block_destruction` / `0x05` row, body shape as breaker id VarInt,
BlockPos, and unsigned-byte progress, fixture breaker id `123`, position
`x=12, y=64, z=-7`, progress `5`, and full body consumption through
Stevenarella dispatch. It does not prove block break animation semantics,
entity existence for the breaker id, client world state, initialized game
state, spawn readiness, world load, render readiness, or client-load
completion. The next
packet-support target by official Play clientbound table order is
`minecraft:block_entity_data` / `0x06`; first determine whether a smallest
official block_entity_data fixture can be generated without initialized
Minecraft/game state.

`configuration_cookie_response_framed_dispatch` is packet-support evidence for
one non-null payload fixture only. It does not prove cookie storage policy,
cookie request/response runtime behavior, Configuration completion, or Play
entry.

`configuration_cookie_request_framed_dispatch` is packet-support evidence for
one Identifier key fixture only. It does not prove cookie storage policy,
cookie request/response runtime behavior, Configuration completion, or Play
entry.

`configuration_select_known_packs_framed_dispatch` is packet-support evidence
only. It does not prove registry hydration, known-pack negotiation completion,
Configuration completion, or Play entry.

`configuration_custom_click_action_framed_dispatch` is packet-support evidence
only. It does not prove UI behavior, command execution, interaction readiness,
Configuration completion, or Play entry.

`configuration_accept_code_of_conduct_framed_dispatch` is packet-support
evidence only. It does not prove UI consent flow, legal acceptance semantics,
Configuration completion, or Play entry.

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

`configuration_registry_data_clientbound_framed_dispatch` is packet-support
evidence for an official `DIMENSION_TYPE` registry-key fixture with an empty
entry list only. It does not prove real registry contents,
`RegistrySynchronization.packRegistries` output, registry hydration,
Configuration completion, Play entry, world load, render readiness, or runtime
behavior.

`configuration_resource_pack_pop_clientbound_framed_dispatch` is packet-support
evidence for one official present-UUID fixture only. It does not prove
resource-pack UI behavior, pack removal policy, download/reload behavior,
Configuration completion, Play entry, world load, render readiness, or runtime
behavior.

`configuration_resource_pack_push_clientbound_framed_dispatch` is
packet-support evidence for one official no-prompt fixture only. It does not
prove resource-pack UI behavior, pack download/reload/application behavior,
Configuration completion, Play entry, world load, render readiness, or runtime
behavior.

`configuration_store_cookie_clientbound_framed_dispatch` is packet-support
evidence for one official key/payload fixture only. It does not prove cookie
storage policy, persistence, cookie request/response runtime behavior,
Configuration completion, Play entry, world load, render readiness, or runtime
behavior.

`configuration_transfer_clientbound_framed_dispatch` is packet-support
evidence for one official host/port fixture only. It does not prove server
transfer UX, reconnect behavior, network reconnection, Configuration
completion, Play entry, world load, render readiness, or runtime behavior.

`configuration_update_enabled_features_clientbound_framed_dispatch` is
packet-support evidence for one official empty feature-set fixture only. It
does not prove feature registry hydration, enabled-feature semantics,
Configuration completion, Play entry, world load, render readiness, or runtime
behavior.

`configuration_update_tags_clientbound_framed_dispatch` is packet-support
evidence for one official empty tag-payload map fixture only. It does not prove
real tag contents, tag registry hydration, Configuration completion, Play
entry, world load, render readiness, or runtime behavior.

`configuration_select_known_packs_clientbound_framed_dispatch` is
packet-support evidence for one official empty known-pack list fixture only. It
does not prove registry hydration, known-pack negotiation completion,
Configuration completion, Play entry, world load, render readiness, or runtime
behavior.

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
load, render readiness, or runtime behavior. It is the current last official
Configuration clientbound table row in the generated 26.1.2 packet table.

`configuration_resource_pack_response_framed_dispatch` is packet-support
evidence only. It does not prove runtime resource pack UI, accept/reject
behavior, pack download/reload behavior, Configuration completion, or Play
entry.
