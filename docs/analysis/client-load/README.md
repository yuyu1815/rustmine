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
| `network_login_configuration` | [phases/network_login_configuration.md](phases/network_login_configuration.md) | `partial` | `configuration_keepalive_codec`, `configuration_keepalive_framed_dispatch`, `configuration_keepalive_clientbound_framed_dispatch`, and `configuration_finish_framed_terminal` jar-backed answers regenerated and exact Rust oracle tests passed against the current Leafish checkout; `configuration_keepalive_runtime_send_helper` and `configuration_keepalive_runtime_protocol_echo` exact runtime socket probes passed against the current Leafish checkout | Add full `spawn_reader` keep-alive reaction and runtime Configuration-to-Play transition proof |
| `registry_hydration` | [phases/registry_hydration.md](phases/registry_hydration.md) | `unproven` | none | Define heavy harness proof; do not fake initialized state |
| `play_entry` | [phases/play_entry.md](phases/play_entry.md) | `unproven` | none | Define transition answer and smoke milestone |
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
