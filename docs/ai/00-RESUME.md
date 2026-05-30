# AI Resume Card

Purpose: restore the current location and next action after compaction, fresh
session, or handoff. This file is a recovery pointer only.

## Current Location

| Field | Value |
|---|---|
| Current location | Protocol 775 `configuration_keepalive_runtime_spawn_reader_reaction` root-owned runtime package now passes: the exact oracle test validates official inbound/outbound keep_alive answers and executes the same factored reader-loop keep_alive branch used by `Server::spawn_reader` |
| Last touched area | `oracle/cases/775/runtime/`, `oracle/contracts/775/runtime/`, `oracle/test-manifests/775/runtime/`, `oracle/failures/775/`, `oracle/rust-tests/`, `docs/analysis/protocol/versions/775/`, `docs/analysis/client-load/`, `docs/analysis/current-evidence/client-load.md`, `docs/ai/00-RESUME.md` |
| Next read entry | `docs/ai/README.md`, `CONTEXT.md` for project terms, then `docs/analysis/responsibility/README.md` and the shard named by the active task |
| Explicit uncertainty | `configuration_keepalive_runtime_spawn_reader_reaction` proves only one Configuration keep_alive reaction through the factored reader-loop branch. It does not prove Configuration completion, runtime Configuration-to-Play transition, Play readiness, registry hydration, world load, render readiness, or client load completion. |

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
          -> for client-load/playability, continue from
             network_login_configuration
            -> next likely target: choose the next missing runtime proof
               after keep_alive reader-loop reaction, likely the
               Configuration-to-Play transition / finish path; keep registry,
               Play, world, render, and interaction readiness as later phases
```

## Stop Boundary

Do not use this card as a durable evidence ledger or decision record.
