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
| `network_login_configuration` | `partial` | `handshake_intention_framed_dispatch`; `login_hello_serverbound_framed_dispatch`; `login_key_serverbound_framed_dispatch`; `configuration_client_information_framed_dispatch`; `configuration_cookie_request_framed_dispatch`; `configuration_cookie_response_framed_dispatch`; `configuration_custom_payload_clientbound_framed_dispatch`; `configuration_custom_payload_framed_dispatch`; `configuration_disconnect_clientbound_framed_dispatch`; `configuration_reset_chat_clientbound_framed_dispatch`; `configuration_registry_data_clientbound_framed_dispatch`; `configuration_resource_pack_pop_clientbound_framed_dispatch`; `configuration_resource_pack_push_clientbound_framed_dispatch`; `configuration_store_cookie_clientbound_framed_dispatch`; `configuration_transfer_clientbound_framed_dispatch`; `configuration_update_enabled_features_clientbound_framed_dispatch`; `configuration_update_tags_clientbound_framed_dispatch`; `configuration_select_known_packs_clientbound_framed_dispatch`; `configuration_custom_report_details_clientbound_framed_dispatch`; `configuration_server_links_clientbound_framed_dispatch`; `configuration_clear_dialog_clientbound_framed_dispatch`; `configuration_show_dialog_clientbound_framed_dispatch`; `configuration_code_of_conduct_clientbound_framed_dispatch`; `configuration_keepalive_codec`; `configuration_keepalive_framed_dispatch`; `configuration_keepalive_clientbound_framed_dispatch`; `configuration_ping_pong_framed_dispatch`; `configuration_finish_framed_terminal`; `configuration_resource_pack_response_framed_dispatch`; `configuration_select_known_packs_framed_dispatch`; `configuration_custom_click_action_framed_dispatch`; `configuration_accept_code_of_conduct_framed_dispatch`; `configuration_keepalive_runtime_send_helper`; `configuration_keepalive_runtime_protocol_echo`; `configuration_keepalive_runtime_spawn_reader_reaction`; `oracle/rust-tests/tests/oracle_contracts.rs`; `bash oracle/scripts/run_jar_backed_oracle_tests.sh` passed on 2026-05-31 for direct jar-backed cases | Handshaking serverbound intention LOGIN fixture framed dispatch/decode; Login serverbound hello name/profileId framed dispatch/decode with body consumption; Login serverbound key keybytes/encryptedChallenge framed dispatch/decode with body consumption; Configuration serverbound client_information framed dispatch/decode, clientbound cookie_request Identifier-key framed dispatch/decode for one key fixture, serverbound cookie_response key/nullable-payload framed dispatch/decode for one non-null payload fixture, clientbound custom_payload BrandPayload framed dispatch/decode for one official BrandPayload fixture, serverbound custom_payload BrandPayload framed dispatch/decode for one official BrandPayload fixture, clientbound disconnect empty literal Component reason framed dispatch/decode, clientbound reset_chat singleton empty-body framed dispatch/decode, clientbound registry_data DIMENSION_TYPE empty-entry framed dispatch/decode with body consumption, clientbound resource_pack_pop present-UUID framed dispatch/decode with body consumption, clientbound resource_pack_push no-prompt framed dispatch/decode with body consumption, clientbound store_cookie Identifier-key/payload framed dispatch/decode with body consumption, clientbound transfer host/port framed dispatch/decode with body consumption, clientbound update_enabled_features empty feature-set framed dispatch/decode with body consumption, clientbound update_tags empty tag-payload map framed dispatch/decode with body consumption, clientbound select_known_packs empty known-pack list framed dispatch/decode with body consumption, clientbound custom_report_details empty details-map framed dispatch/decode with body consumption, clientbound server_links empty links-list framed dispatch/decode with body consumption, clientbound clear_dialog singleton empty-body framed dispatch/decode with body consumption, clientbound show_dialog direct NoticeDialog context-free fixture framed dispatch/decode with body consumption, clientbound code_of_conduct String fixture framed dispatch/decode with body consumption, serverbound keep-alive packet id/body, serverbound/clientbound keep-alive framed dispatch/decode, clientbound ping/serverbound pong framed dispatch/decode, finish_configuration framed dispatch/decode plus official terminal flags, serverbound resource_pack response frame dispatch/decode with UUID/action body consumption, serverbound select_known_packs known-pack list dispatch/decode with full body consumption, serverbound custom_click_action identifier/optional-payload dispatch/decode with full body consumption, serverbound accept_code_of_conduct empty-body dispatch/decode with full body consumption, outgoing helper send of the official Configuration serverbound keep_alive frame, protocol-crate socket echo from official Configuration clientbound keep_alive to official Configuration serverbound keep_alive, and the factored `Server::spawn_reader` keep_alive branch response all match reset-proof evidence against the current Leafish checkout. | Login authentication success, Login encryption success/private-key validation, Login custom-query/login-ack/cookie-response support, arbitrary plugin-channel handling, payload routing policy, UI disconnect handling, screen flow, chat UI reset behavior, UI consent flow, legal acceptance semantics, report UI behavior, moderation/reporting semantics, server-links UI behavior, trust/link-opening policy, dialog UI clearing behavior, dialog UI display behavior, registry-backed dialogs, cookie storage policy, cookie persistence, cookie request/response runtime behavior, server transfer UX, reconnect behavior, network reconnection, runtime custom-click UI behavior, command execution, interaction readiness, real registry contents, RegistrySynchronization.packRegistries output, feature registry hydration, enabled-feature semantics, tag registry hydration, registry hydration, runtime known-pack negotiation completion, resource-pack UI behavior, pack removal policy, pack download/reload/application behavior, runtime resource pack UI/accept/reject behavior, runtime client settings send behavior, runtime ping response behavior, full login/configuration runtime behavior, runtime Configuration-to-Play transition, play transition |
| `registry_hydration` | `unproven` | none | nothing yet | registry/dimension/known-pack/feature state |
| `play_entry` | `unproven` | none | nothing yet | successful entry into Play |
| `world_hydration` | `unproven` | none | nothing yet | chunks, light, block states, biomes, world time |
| `entity_player_hydration` | `unproven` | none | nothing yet | local player, remote players, entities, spawn readiness |
| `render_ready` | `unproven` | none | nothing yet | visible loaded world, screenshot/pixel readiness |
| `control_interact_ready` | `unproven` | none | nothing yet | movement, interact, inventory, combat after load |

## Snapshot Note

At this snapshot, the proven compatibility is only
`handshake_intention_framed_dispatch`,
`login_hello_serverbound_framed_dispatch`,
`login_key_serverbound_framed_dispatch`,
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
`configuration_custom_click_action_framed_dispatch`, and
`configuration_accept_code_of_conduct_framed_dispatch`, jar-backed,
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
