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
| `network_login_configuration` | `partial` | `configuration_client_information_framed_dispatch`; `configuration_cookie_request_framed_dispatch`; `configuration_cookie_response_framed_dispatch`; `configuration_custom_payload_clientbound_framed_dispatch`; `configuration_custom_payload_framed_dispatch`; `configuration_keepalive_codec`; `configuration_keepalive_framed_dispatch`; `configuration_keepalive_clientbound_framed_dispatch`; `configuration_ping_pong_framed_dispatch`; `configuration_finish_framed_terminal`; `configuration_resource_pack_response_framed_dispatch`; `configuration_select_known_packs_framed_dispatch`; `configuration_custom_click_action_framed_dispatch`; `configuration_accept_code_of_conduct_framed_dispatch`; `configuration_keepalive_runtime_send_helper`; `configuration_keepalive_runtime_protocol_echo`; `configuration_keepalive_runtime_spawn_reader_reaction`; `oracle/rust-tests/tests/oracle_contracts.rs`; `bash oracle/scripts/run_jar_backed_oracle_tests.sh` passed on 2026-05-31 for direct jar-backed cases | Configuration serverbound client_information framed dispatch/decode, clientbound cookie_request Identifier-key framed dispatch/decode for one key fixture, serverbound cookie_response key/nullable-payload framed dispatch/decode for one non-null payload fixture, clientbound custom_payload BrandPayload framed dispatch/decode for one official BrandPayload fixture, serverbound custom_payload BrandPayload framed dispatch/decode for one official BrandPayload fixture, serverbound keep-alive packet id/body, serverbound/clientbound keep-alive framed dispatch/decode, clientbound ping/serverbound pong framed dispatch/decode, finish_configuration framed dispatch/decode plus official terminal flags, serverbound resource_pack response frame dispatch/decode with UUID/action body consumption, serverbound select_known_packs known-pack list dispatch/decode with full body consumption, serverbound custom_click_action identifier/optional-payload dispatch/decode with full body consumption, serverbound accept_code_of_conduct empty-body dispatch/decode with full body consumption, outgoing helper send of the official Configuration serverbound keep_alive frame, protocol-crate socket echo from official Configuration clientbound keep_alive to official Configuration serverbound keep_alive, and the factored `Server::spawn_reader` keep_alive branch response all match reset-proof evidence against the current Leafish checkout. | arbitrary plugin-channel handling, payload routing policy, UI consent flow, legal acceptance semantics, cookie storage policy, cookie request/response runtime behavior, runtime custom-click UI behavior, command execution, interaction readiness, runtime known-pack negotiation completion, registry hydration, runtime resource pack UI/accept/reject behavior, runtime client settings send behavior, runtime ping response behavior, full login/configuration runtime behavior, runtime Configuration-to-Play transition, play transition |
| `registry_hydration` | `unproven` | none | nothing yet | registry/dimension/known-pack/feature state |
| `play_entry` | `unproven` | none | nothing yet | successful entry into Play |
| `world_hydration` | `unproven` | none | nothing yet | chunks, light, block states, biomes, world time |
| `entity_player_hydration` | `unproven` | none | nothing yet | local player, remote players, entities, spawn readiness |
| `render_ready` | `unproven` | none | nothing yet | visible loaded world, screenshot/pixel readiness |
| `control_interact_ready` | `unproven` | none | nothing yet | movement, interact, inventory, combat after load |

## Snapshot Note

At this snapshot, the proven compatibility is only
`configuration_client_information_framed_dispatch`,
`configuration_cookie_request_framed_dispatch`,
`configuration_cookie_response_framed_dispatch`,
`configuration_custom_payload_clientbound_framed_dispatch`,
`configuration_custom_payload_framed_dispatch`,
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

`configuration_resource_pack_response_framed_dispatch` is packet-support
evidence only. It does not prove runtime resource pack UI, accept/reject
behavior, pack download/reload behavior, Configuration completion, or Play
entry.
