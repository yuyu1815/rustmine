# AI Resume Card

Purpose: restore the current location and next action after compaction, fresh
session, or handoff. This file is a recovery pointer only.

## Current Location

| Field | Value |
|---|---|
| Current location | Protocol 775 `configuration_server_links_clientbound_framed_dispatch` packet-support package now passes: the exact oracle test validates the official Configuration clientbound `minecraft:server_links` answer for one empty links list fixture and decodes it through `packet::packet_by_id(775, State::Configuration, Direction::Clientbound, official id, body)` |
| Last touched area | `oracle/cases/775/`, `oracle/contracts/775/`, `oracle/answers/775/`, `oracle/test-manifests/775/`, `oracle/failures/775/`, `oracle/rust-tests/`, `oracle/harness/java/`, `stevenarella/protocol/src/protocol/{mod.rs,packet.rs,versions/v26_1_2.rs}`, `docs/analysis/protocol/versions/775/`, `docs/analysis/client-load/`, `docs/analysis/current-evidence/client-load.md`, `docs/ai/00-RESUME.md` |
| Next read entry | `docs/ai/README.md`, `CONTEXT.md` for project terms, then `docs/analysis/responsibility/README.md` and the shard named by the active task |
| Explicit uncertainty | `configuration_server_links_clientbound_framed_dispatch` proves only the Configuration clientbound server_links packet id/body dispatch for one official empty links list fixture and current compatibility alias decode. It does not prove server-links UI behavior, trust/link-opening policy, Configuration completion, runtime Configuration-to-Play transition, Play readiness, world load, render readiness, or client load completion. |

## Recovery Flow

```text
Read AGENTS.md
  -> read docs/ai/README.md
  -> read CONTEXT.md when project vocabulary is unclear or being sharpened
  -> read docs/analysis/README.md for domain routing
  -> read only the active shard, workflow, agent role, or task artifact
  -> update this card only for current location, next action, or recovery route
```

## Next Action

```text
For future work:
  start from docs/ai/README.md
    -> choose the owning responsibility shard for the active task
      -> keep parent Codex responsible for user answers, route decisions,
         final summaries, and recovery pointer updates
        -> use subagents only for bounded evidence, mapping, review, or
           implementation work packages
          -> for packet-support loop, continue from
             network_login_configuration
            -> next likely target: ask an oracle subagent to create the next
               missing Configuration clientbound packet proof,
               likely minecraft:clear_dialog / 0x11; keep runtime
               Configuration-to-Play, registry, Play, world, render, and
               interaction readiness as later phases
```

## Stop Boundary

Do not use this card as a durable evidence ledger or decision record.
