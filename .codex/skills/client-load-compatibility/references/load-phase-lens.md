# Client Load Phase Lens

Purpose: keep "client loaded" from collapsing into packet work. Every load
claim should name its phase or justified cross-phase target, evidence surface,
and done condition.

Mutable phase state lives in:

```text
docs/analysis/client-load/README.md
docs/analysis/client-load/phases/*.md
docs/analysis/current-evidence/client-load.md
```

## Phases

| Phase | User-visible meaning | Primary owner | Typical proof |
|---|---|---|---|
| `local_boot_resources` | Client starts and local resources/assets are available enough to run | client runtime / resources | resource fixture, startup smoke, render setup proof |
| `network_login_configuration` | Handshake, login, configuration, and transition toward play are coherent | wire protocol / runtime behavior | jar-backed protocol oracle, state transition contract, fake/server smoke |
| `registry_hydration` | Registry, dimension, feature, known-pack, and configuration data are accepted and stored | protocol + runtime bridge | initialized official harness or source-backed registry artifact |
| `play_entry` | Client reaches Play with enough state to receive world data | runtime behavior | transition contract plus smoke milestone |
| `world_hydration` | Chunks, light, block states, biomes, and world time become usable world state | runtime behavior / verification | chunk/world oracle fixture, smoke, visual/probe proof |
| `entity_player_hydration` | Local/remote players and entities become usable runtime state | runtime behavior | entity/player contract, spawn/position smoke |
| `render_ready` | The loaded state is visible and stable enough to inspect | runtime/render bridge | screenshot/pixel or render milestone proof |
| `control_interact_ready` | Movement, interaction, inventory, and combat actions work after load | runtime behavior / e2e | corridor probe or own-server smoke |

## Phase Selection

```text
complaint or goal
  -> user-visible load failure
    -> current evidence
      -> structural scan
        -> default earliest unproven/failing phase or explicit target
          -> narrowest official evidence
            -> reset-proof test/probe
```

Default to the earliest phase that can explain the failure and whose proof is
missing or failing. A task may stay on a later phase or cross-phase target when
its scope, evidence, and stop boundary explicitly justify that target. Do not
jump to packet work if a later runtime/render phase is the actual claim, and do
not mark earlier phases complete from path existence.

## Evidence Labels

| Label | Meaning | Can mark done? |
|---|---|---|
| `unproven` | No root-owned proof exists | no |
| `observed_only` | Current checkout has paths/modules that look related | no |
| `partial` | Some proof exists, but the phase claim is narrower than done | no |
| `verified` | Root-owned proof satisfies the phase done condition | yes |
| `blocked` | Required evidence surface cannot be produced yet | no |

## Done Conditions

Each phase entry needs:

```text
phase
claim
current proof label
official evidence or accepted smoke basis
project-level test/probe
Stevenarella owner under test
current proof state
next gap
```

If the done condition is "client visibly loads," the proof cannot be only a
codec/golden test. It needs at least a runtime or smoke/probe surface.
