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
| `network_login_configuration` | `partial` | `configuration_client_information_framed_dispatch`; `configuration_keepalive_codec`; `configuration_keepalive_framed_dispatch`; `configuration_keepalive_clientbound_framed_dispatch`; `configuration_ping_pong_framed_dispatch`; `configuration_finish_framed_terminal`; `configuration_resource_pack_response_framed_dispatch`; `configuration_keepalive_runtime_send_helper`; `configuration_keepalive_runtime_protocol_echo`; `oracle/rust-tests/tests/oracle_contracts.rs`; `bash oracle/scripts/run_jar_backed_oracle_tests.sh` passed on 2026-05-31 | Configuration serverbound client_information framed dispatch/decode, serverbound keep-alive packet id/body, serverbound/clientbound keep-alive framed dispatch/decode, clientbound ping/serverbound pong framed dispatch/decode, finish_configuration framed dispatch/decode plus official terminal flags, serverbound resource_pack response frame dispatch/decode with UUID/action body consumption, outgoing helper send of the official Configuration serverbound keep_alive frame, and protocol-crate socket echo from official Configuration clientbound keep_alive to official Configuration serverbound keep_alive match reset-proof evidence against the current Leafish checkout | runtime resource pack UI/accept/reject behavior, runtime client settings send behavior, runtime ping response behavior, full login/configuration runtime behavior, full keep-alive response loop through `spawn_reader`, runtime Configuration-to-Play transition, play transition |
| `registry_hydration` | `unproven` | none | nothing yet | registry/dimension/known-pack/feature state |
| `play_entry` | `unproven` | none | nothing yet | successful entry into Play |
| `world_hydration` | `unproven` | none | nothing yet | chunks, light, block states, biomes, world time |
| `entity_player_hydration` | `unproven` | none | nothing yet | local player, remote players, entities, spawn readiness |
| `render_ready` | `unproven` | none | nothing yet | visible loaded world, screenshot/pixel readiness |
| `control_interact_ready` | `unproven` | none | nothing yet | movement, interact, inventory, combat after load |

## Snapshot Note

At this snapshot, the proven compatibility is only
`configuration_client_information_framed_dispatch`,
`configuration_keepalive_codec`, `configuration_keepalive_framed_dispatch`,
`configuration_keepalive_clientbound_framed_dispatch`,
`configuration_ping_pong_framed_dispatch`,
`configuration_finish_framed_terminal`, and
`configuration_resource_pack_response_framed_dispatch`, jar-backed,
regenerated in the current run, and checked by exact reset-proof Rust oracle
tests against the current Leafish checkout. `configuration_keepalive_runtime_send_helper` and
`configuration_keepalive_runtime_protocol_echo` are root-owned runtime socket
probes that now pass against the current Leafish checkout; they prove the
outgoing helper frame and the protocol-crate read/map/send echo path, not the
full `spawn_reader` loop.

`configuration_resource_pack_response_framed_dispatch` is packet-support
evidence only. It does not prove runtime resource pack UI, accept/reject
behavior, pack download/reload behavior, Configuration completion, or Play
entry.
